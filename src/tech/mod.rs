use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::iter::IntoIterator;
use std::ops::{Index, IndexMut};
use std::slice::IterMut;
use std::sync::{Arc, Mutex, Weak};

use self::crystalprop::{CrystalProperties, CrystalProperty, Tech};
use self::technode::*;


pub mod crystalprop;
pub mod technode;


type Ptr<T> = Arc<Mutex<T> >;


pub struct TechDiGraph {
  prereqs: Vec<TechNode>,
  advanced: Vec<TechNode>,
}


impl TechDiGraph {
  pub fn new() -> Self {
    TechDiGraph {
      prereqs: Vec::new(),
      advanced: Vec::new(),
    }
  }


  pub fn add_prereq(&mut self, tech: Tech) -> Result<(), String> {
    if Self::get_node_ref(&self.advanced, tech).is_some() {
      Err("add_prereq(): Tech already added to advanced list".to_string() )
  } else if Self::get_node_ref(&self.prereqs, tech).is_some() {
      Err("add_prereq(): Tech already added to preqreq list".to_string() )
    } else {
      self.prereqs.push(TechNode::new(tech) );
      Ok(() )
    }
  }

  pub fn add_advanced_link(&mut self, prereq: Tech, adv: Tech) -> Result<(), String> {
    if prereq == adv {
      return Err("add_advanced_link(): Trying to link from tech to itself".to_string() );
    }

    let prereq_ptr_mut_result: (usize, *mut TechNode) = {
      let (prereqs, advanced) = (&mut self.prereqs, &mut self.advanced);
      Self::get_node_ref_mut(prereqs, prereq).
      or_else(move || Self::get_node_ref_mut(advanced, prereq) ).
      map(|(i, r)| (i, r as *mut TechNode) ).
      ok_or("add_advanced_link(): Prereq tech does not exist".to_string() )?
    };

    let adv_ptr_mut_result: (usize, *mut TechNode) = {
      Self::get_node_ref_mut(&mut self.advanced, adv).
      map(|(i, r)| (i, r as *mut TechNode) ).
      unwrap_or_else(|| {
          self.advanced.push(TechNode::new(adv) );
          ((self.advanced.len() - 1), (self.advanced.last_mut().unwrap() as *mut TechNode) )
      })
    };

    if prereq_ptr_mut_result.1 == adv_ptr_mut_result.1 {
      return Err("add_advanced_link(): Trying to link from tech to itself".to_string() );
    }

    let prereq_ref_mut = unsafe {
      (prereq_ptr_mut_result.1).
      as_mut().
      ok_or("add_advanced_link(): Prereq tech does not exist".to_string() )?
    };

    let adv_ref_mut = unsafe {
      (adv_ptr_mut_result.1).
      as_mut().
      ok_or("add_advanced_link(): Adv tech does not exist after insert".to_string() )?
    };

    if {
      let mut edge_exists = false;
      if let Some(node_ptrs) = prereq_ref_mut.get_out_edges() {
        for node in node_ptrs {
          if (*node) == (adv_ref_mut as *mut TechNode) {
            edge_exists = true;
            break;
          }
        }
      }
      !edge_exists
    } {
      prereq_ref_mut.add_out_edge(adv_ref_mut as *mut TechNode);
      adv_ref_mut.add_in_edge(prereq_ref_mut as *const TechNode);
    }

    Ok(() )
  }

  fn get_node_ref<'a>(nodes: &'a [TechNode], tech: Tech) -> Option<(usize, &'a TechNode)> {
    for (i, node) in nodes.into_iter().enumerate() {
      if node.get_tech_name() == tech {
        return Some((i, node) );
      }
    }

    None
  }

  fn get_node_ref_mut<'a>(nodes: &'a mut [TechNode], tech: Tech) -> Option<(usize, &'a mut TechNode)> {
    for (i, node) in nodes.into_iter().enumerate() {
      if node.get_tech_name() == tech {
        return Some((i, node) );
      }
    }

    None
  }
/*
  pub fn mark_studied(&mut self, tech: Tech) -> Result<(), String> {
    let mut node_ref = Self::get_node_ref_mut(&mut self.prereqs, tech);
    if node_ref.is_none() {
      node_ref = Self::get_node_ref_mut(&mut self.advanced, tech);
    }
    if node_ref.is_none() {
      return Err("Tech not found".to_string() );
    }

    node_ref = node_ref.unwrap();

    node_ref.mark_researched()
  }
*/
}

pub struct TechState {
  properties: CrystalProperties,
  tech_graph: TechDiGraph,
}

impl TechState {
  pub fn new() -> Self {
    TechState {
      properties: CrystalProperties::new(),
      tech_graph: TechDiGraph::new(),
    }
  }
}


#[cfg(test)]
mod tests;
