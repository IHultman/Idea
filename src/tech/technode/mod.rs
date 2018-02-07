use super::crystalprop::*;
use super::techidx::*;


pub struct TechNode {
  tech_name: Tech,
  available: bool,
  is_prereq: bool,
  researched: bool,
  acquired_in_edges: Option<Vec<TechIdx> >,
  unacquired_in_edges: Option<Vec<TechIdx> >,
  out_edges: Option<Vec<TechIdx> >,
}

enum TechNodeErrs {
  InEdges(EdgesErrs),
  OutEdges(EdgesErrs),
}

enum EdgesErrs {
  IdxTechMatch,
  InconsistentIdxMatch,
  InconsistentTechMatch,
}

impl TechNode {
  pub fn new(tech: Tech, is_prereq: bool) -> TechNode {
    TechNode {
      tech_name: tech,
      available: is_prereq,
      is_prereq: is_prereq,
      researched: false,
      acquired_in_edges: None,
      unacquired_in_edges: None,
      out_edges: None,
    }
  }

  pub fn add_in_edge(&mut self, tidx: TechIdx) -> Result<(), TechNodeErrs> {
    if let Some(ref ai_edges) = self.acquired_in_edges {
      for i in ai_edges {
        if i.get_idx() == tidx.get_idx() {
          if i.get_tech() != tidx.get_tech() {
            return Err(TechNodeErrs::InEdges(EdgesErrs::InconsistentIdxMatch) );
          } else {
            return Err(TechNodeErrs::InEdges(EdgesErrs::IdxTechMatch) );
          }
        }

        if i.get_tech() == tidx.get_tech() {
          return Err(TechNodeErrs::InEdges(EdgesErrs::InconsistentTechMatch) );
        }
      }
    }

    let ui_edges: &mut Vec<TechIdx> = self.unacquired_in_edges.get_or_insert(Vec::with_capacity(5) );
    for i in ui_edges {
      if i.get_idx() == tidx.get_idx() {
        if i.get_tech() != tidx.get_tech() {
          return Err(TechNodeErrs::InEdges(InEdgesErrs::InconsistentIdxMatch) );
        } else {
          return Err(TechNodeErrs::InEdges(InEdgesErrs::IdxTechMatch) );
        }
      }

      if i.get_tech() == tidx.get_tech() {
        return Err(TechNodeErrs::InEdges(InEdgesErrs::InconsistentTechMatch) );
      }
    }

    ui_edges.push(tidx);
    Ok(() )
  }

  pub fn add_out_edge(&mut self, tidx: TechIdx) -> Result<(), TechNodeErrs> {
    let o_edges: &mut Vec<TechIdx> = self.out_edges.get_or_insert(Vec::with_capacity(5) );
    for i in o_edges {
      if i.get_idx() == tidx.get_idx() {
        if i.get_tech() != tidx.get_tech() {
            return Err(TechNodeErrs::OutEdges(EdgesErrs::InconsistentIdxMatch) );
        } else {
            return Err(TechNodeErrs::OutEdges(EdgesErrs::IdxTechMatch) );
        }
      }

      if i.get_tech() == tidx.get_tech() {
        return Err(TechNodeErrs::OutEdges(EdgesErrs::InconsistentTechMatch) );
      }
    }

    o_edges.push(tidx);
    Ok(() )
  }

  pub fn get_acquired_in_edges(&self) -> Option<&Vec<TechIdx> > {
    self.acquired_in_edges.as_ref()
  }

  pub fn get_all_in_edges(&self) -> Option<Vec<TechIdx> > {
    //
    None
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

  pub fn mark_researched() -> Result<(), TechNodeErrs> {
    //
    Ok(() )
  }
}


#[cfg(test)]
mod tests;
