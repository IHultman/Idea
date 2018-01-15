use std::sync::{Arc, Mutex, Weak};

use super::crystalprop::*;

type Ptr<T> = Arc<Mutex<T> >;


pub struct TechNode {
  tech_name: Tech,
  available: bool,
  researched: bool,
  acquired_in_edges: Option<Vec<*const TechNode> >,
  unacquired_in_edges: Option<Vec<*const TechNode> >,
  out_edges: Option<Vec<*mut TechNode> >,
}

impl TechNode {
  pub fn new(name: Tech) -> Self {
    TechNode {
      tech_name: name,
      available: false,
      researched: false,
      acquired_in_edges: None,
      unacquired_in_edges: None,
      out_edges: None,
    }
  }

  pub fn add_in_edge(&mut self, tech: *const TechNode) {
    let vec: &mut Vec<*const TechNode> = self.unacquired_in_edges.get_or_insert(Vec::new() );
    vec.push(tech);
  }

  pub fn add_out_edge(&mut self, tech: *mut TechNode) {
    let vec: &mut Vec<*mut TechNode> = self.out_edges.get_or_insert(Vec::new() );
    vec.push(tech);
  }

  pub fn mark_researched(&mut self) -> Result<(), String> {
    if self.available {
      if !self.researched {
        self.researched = true
      } else {
        return Err("Already researched".to_string() );
      }
    } else {
      return Err("Prerequisites not met".to_string() );
    }

    let this_addr = self as *const TechNode;
    if let Some(ref mut node_ptrs) = self.out_edges {
      for node in node_ptrs {
        unsafe {
          (**node).move_link_acquired(this_addr)?;
        }
      }
    }

    Ok(() )
  }

  unsafe fn move_link_acquired(&mut self, tech_ptr: *const TechNode) -> Result<(), String> {
    let mut empty = false;
    match self.unacquired_in_edges {
      Some(ref mut node_ptrs) => {
        let mut to_move = None;
        for (i, node) in node_ptrs.into_iter().enumerate() {
          if (*node) == tech_ptr {
            to_move = Some(i);
            break;
          }
        }

        if let Some(i) = to_move {
          let vec: &mut Vec<*const TechNode> = self.acquired_in_edges.get_or_insert(Vec::new() );
          vec.push(node_ptrs.remove(i) );
          if node_ptrs.is_empty() {
            empty = true;
          }
        } else {
          return Err("Link to tech not found".to_string() );
        }
      },
      None => {
        return Err("Link to tech not found".to_string() );
      },
    }

    if empty {
      self.unacquired_in_edges = None;
      self.available = true;
    }

    Ok(() )
  }
}


#[cfg(test)]
mod tests;
