use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::iter::IntoIterator;
use std::ops::{Index, IndexMut};
use std::slice::IterMut;
use std::sync::{Arc, Mutex, Weak};

use self::crystalprop::{CrystalProperties, CrystalProperty, Tech};
use self::technode::*;


pub mod crystalprop;
mod technode;


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

/*
  pub fn add_prereq(&mut self, tech: Tech) -> Result<(), String> {
    Self::get_node_ref(&self.advanced, tech).
    map_or(Some(() ), |_| None).
    ok_or("add_prereq(): Tech already added to advanced list".to_string() )?;

    Self::get_node_ref(&self.prereqs, tech).
    map_or(Some(() ), |_| None).
    ok_or("add_prereq(): Tech already added to prereq list".to_string() )?;

    self.prereqs.push(TechNode::new(tech) );
    Ok(() )
  }
*/

  pub fn add_advanced_link(&mut self, prereq: Tech, adv: Tech) -> Result<(), String> {
    if prereq == adv {
      return Err("add_advanced_link(): Trying to link from tech to itself".to_string() );
    }

    if Self::get_node_ref(&self.prereqs, adv).is_some() {
      return Err("add_advanced_link(): Trying to link to an illegal tech".to_string() );
    }

    let prereq_ptr_mut: *mut TechNode = {
      let (prereqs, advanced) = (&mut self.prereqs, &mut self.advanced);
      Self::get_node_ref_mut(prereqs, prereq).
      or_else(move || Self::get_node_ref_mut(advanced, prereq) ).
      map(|r| r as *mut TechNode).
      ok_or("add_advanced_link(): Prereq tech does not exist".to_string() )?
    };

    let adv_ptr_mut: *mut TechNode = {
      Self::get_node_ref_mut(&mut self.advanced, adv).
      map(|r| r as *mut TechNode).
      or_else(|| {
          self.advanced.push(TechNode::new(adv) );
          self.advanced.last_mut().
          map(|r| r as *mut TechNode)
      }).
      ok_or("add_advanced_link(): Adv cannot be inserted to digraph".to_string() )?
    };

    if prereq_ptr_mut == adv_ptr_mut {
      return Err("add_advanced_link(): Trying to link from tech to itself".to_string() );
    }

    let prereq_ref_mut = unsafe {
      prereq_ptr_mut.as_mut().
      ok_or("add_advanced_link(): Prereq tech does not exist".to_string() )?
    };

    let adv_ref_mut = unsafe {
      adv_ptr_mut.as_mut().
      ok_or("add_advanced_link(): Adv tech does not exist after insert".to_string() )?
    };

    prereq_ref_mut.add_out_edge(adv_ref_mut as *mut TechNode)?;
    adv_ref_mut.add_in_edge(prereq_ref_mut as *const TechNode)?;

    Ok(() )
  }

  fn get_node_ref<'a>(nodes: &'a [TechNode], tech: Tech) -> Option<&'a TechNode> {
    for node in nodes {
      if node.get_tech_name() == tech {
        return Some(node);
      }
    }

    None
  }

  fn get_node_ref_mut<'a>(nodes: &'a mut [TechNode], tech: Tech) -> Option<&'a mut TechNode> {
    for node in nodes {
      if node.get_tech_name() == tech {
        return Some(node);
      }
    }

    None
  }

  pub fn mark_studied(&mut self, tech: Tech) -> Result<(), String> {
    let (prereqs, advanced) = (&mut self.prereqs, &mut self.advanced);
    Self::get_node_ref_mut(prereqs, tech).
    or_else(move || Self::get_node_ref_mut(advanced, tech) ).
    ok_or("mark_studied(): Tech not found".to_string() )?.
    mark_researched()
  }

  pub fn show_graph(&self) {

  }

  pub fn show_local_graph(&self, tech: Tech) -> Result<(), String> {
    let tech_ref = {
      Self::get_node_ref(&self.prereqs, tech).
      or_else(|| Self::get_node_ref(&self.advanced, tech) ).
      ok_or("show_local_graph(): Tech not found".to_string() )?
      as *const TechNode
    };

    let mut top_layer: Vec<*const TechNode> = Vec::new();
    top_layer.push(tech_ref);

    while !top_layer.is_empty() {
      let mut layer_str = "".to_string();
      let mut next_layer: Vec<*const TechNode> = Vec::new();

      for next in top_layer {
        unsafe {
          layer_str = layer_str + &*String::from((*next).get_tech_name() ) + "  ";
          if let Some(ie_vec) = (*next).get_all_in_edges() {
            for t in ie_vec {
              next_layer.push(t);
            }
          }
        }
      }

      println!("{}", layer_str);
      top_layer = next_layer;
    }

    Ok(() )
  }
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
