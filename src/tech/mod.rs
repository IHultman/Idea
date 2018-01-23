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
    if get_node_ref(&self.advanced, tech).is_some() {
      Err("add_prereq(): Tech already added to advanced list".to_string() )
    } else if get_node_ref(&self.prereqs, tech).is_some() {
      Err("add_prereq(): Tech already added to preqreq list".to_string() )
    } else {
      self.prereqs.push(TechNode::new(tech) );
      Ok(() )
    }
  }
/*
  pub fn add_advanced_link(&mut self, prereq: Tech, adv: Tech) -> Result<(), String> {
    if prereq == adv {
      return Err("add_advanced_link(): Trying to link from tech to itself".to_string() );
    }

    let prereq_ref_mut: &mut TechNode = {
      get_node_ref_mut(&mut self.prereqs).
      or_else(|| get_node_ref_mut(&mut self.advanced) ).
      ok_or("add_advanced_link(): Prereq tech does not exist".to_string() )?
      //as *mut TechNode
    };

    let mut adv_ref_mut: Option<&mut TechNode> = get_node_ref_mut(&mut self.advanced);
    if adv_ref_mut.is_none() {
      self.advanced.push(TechNode::new(adv) );
      adv_ref_mut = self.advanced.last_mut();
    }

    let adv_ref_mut: &mut TechNode = {
      get_node_ref_mut(&mut self.advanced).
    };

    let adv_ref_mut: &mut TechNode = adv_ref_mut.unwrap();
    if unsafe {
      let mut edge_exists = false;
      if let Some(ref node_ptrs) = prereq_ref_mut.out_edges {
        for node in node_ptrs {
          if node.tech_name == adv_ref_mut.tech_name {
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
*/
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
