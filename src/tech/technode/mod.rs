use super::crystalprop::*;


pub struct TechIdx {
  idx: usize,
  tech: Tech
}

impl TechIdx {
  pub fn new(idx: usize, tech: Tech) -> TechIdx {
    TechIdx {
      idx: idx,
      tech: tech,
    }
  }

  pub fn get_idx(&self) -> usize {
    self.idx
  }

  pub fn get_tech(&self) -> Tech {
    self.tech
  }

  pub fn set_idx(&mut self, idx: usize) {
    self.idx = idx;
  }

  pub fn set_tech(&mut self, tech: Tech) {
    self.tech = tech;
  }
}


pub struct TechNode {
  tech_name: Tech,
  available: bool,
  is_prereq: bool,
  researched: bool,
  acquired_in_edges: Option<Vec<TechIdx> >,
  unacquired_in_edges: Option<Vec<TechIdx> >,
  out_edges: Option<Vec<TechIdx> >,
}

impl TechNode {
  pub fn new(tech: Tech, as_prereq: bool) -> TechNode {
    TechNode {
      tech_name: tech,
      available: true,
      is_prereq: as_prereq,
      researched: false,
      acquired_in_edges: None,
      unacquired_in_edges: None,
      out_edges: None,
    }
  }

  pub fn add_in_edge(&mut self, tidx: TechIdx) -> Result<(), String> {
    if let Some(ref ai_edges) = self.acquired_in_edges {
      for i in ai_edges {
        if i.get_idx() == tidx.get_idx() {
          return Err("".to_string() );
        }
        if i.get_tech() == tidx.get_tech() {
          return Err("".to_string() );
        }
      }
    }

    let ui_edges: &mut Vec<TechIdx> = self.unacquired_in_edges.get_or_insert(Vec::new() );
    for i in ui_edges {
      if i.get_idx() == tidx.get_idx() {
        return Err("".to_string() );
      }
      if i.get_tech() == tidx.get_tech() {
        return Err("".to_string() );
      }
    }

    ui_edges.push(tidx);
    Ok(() )
  }

  pub fn add_out_edge(&mut self, tidx: TechIdx) -> Result<(), String> {

  }

  pub fn get_acquired_in_edges(&self) -> Option<&Vec<TechIdx> > {
    self.acquired_in_edges.as_ref()
  }

  pub fn get_all_in_edges(&self) {

  }

  pub fn get_out_edges(&self) -> Option<&Vec<TechIdx> > {
    self.out_edges.as_ref()
  }

  pub fn get_tech_name(&self) -> Tech {
    self.tech_name
  }

  pub fn get_unacquired_in_edges() -> Option<&Vec<TechIdx> > {
    self.unacquired_in_edges.as_ref()
  }

  pub fn mark_researched() {

  }
}


#[cfg(test)]
mod tests;
