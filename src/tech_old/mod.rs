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
      prereqs: Vec::with_capacity(100),
      advanced: Vec::with_capacity(100),
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
  // change to binary search
    for node in nodes {
      if node.get_tech_name() == tech {
        return Some(node);
      }
    }

    None
  }

  fn get_node_ref_mut<'a>(nodes: &'a mut [TechNode], tech: Tech) -> Option<&'a mut TechNode> {
  // change to binary search
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

  pub fn cleanup_edges(&mut self) -> Result<(), String> {
    let mut new_graph = TechDiGraph {
      prereqs: Vec::with_capacity(self.prereqs.len() ),
      advanced: Vec::with_capacity(self.advanced.len() ),
    };

    for prereq in self.prereqs.iter() {
      new_graph.add_prereq(prereq.get_tech_name() )?;
    }

    Self::filter_tech_list(&self.prereqs, &mut new_graph)?;
    Self::filter_tech_list(&self.advanced, &mut new_graph)?;

    (*self) = new_graph;
    Ok(() )
  }

  fn filter_tech_list(list: &Vec<TechNode>, new_graph: &mut TechDiGraph) -> Result<(), String> {
    for tech in list {
      if let Some(out_edges) = tech.get_out_edges() {
        for &t_ptr in out_edges {
          unsafe {
            let t_name = (*t_ptr).get_tech_name();
            for &oth_ptr in out_edges.into_iter().filter(|&&a| a != t_ptr) {
              if !Self::path_to_tech(oth_ptr, t_name) {
                new_graph.add_advanced_link(tech.get_tech_name(), t_name)?;
              }
            }
          }
        }
      }
    }

    Ok(() )
  }

  fn path_to_tech(t_ptr: *const TechNode, tech: Tech) -> bool {
    let mut q = Vec::with_capacity(10);
    q.push(t_ptr);

    while !q.is_empty() {
      let next_ptr = q.remove(0);
      unsafe {
        if (*next_ptr).get_tech_name() == tech {
          return true;
        } else if let Some(out_edges) = (*next_ptr).get_out_edges() {
          for &out_ptr in out_edges {
            q.push(out_ptr);
          }
        }
      }
    }

    false
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
    let mut tech_listed: Vec<Tech> = Vec::new();

    while !top_layer.is_empty() {
      let mut layer_str = "".to_string();
      let mut next_layer: Vec<*const TechNode> = Vec::new();

      for next in top_layer {
        unsafe {
          layer_str = layer_str + &*String::from((*next).get_tech_name() ) + "  ";
          if let Some(ie_vec) = (*next).get_all_in_edges() {
            for t in ie_vec {
              let t_name = (*t).get_tech_name();
              if Self::can_add_tech_to_list(&mut tech_listed, t_name) {
                next_layer.push(t);
              }
            }
          }
        }
      }

      println!("{}\n\n\n", layer_str);
      top_layer = next_layer;
    }

    Ok(() )
  }

  fn can_add_tech_to_list(tl: &mut Vec<Tech>, tech: Tech) -> bool {
    match tl.binary_search(&tech) {
      Ok(_)  => false,
      Err(i) => {
        tl.insert(i, tech);
        true
      },
    }
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
