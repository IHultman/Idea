use super::crystalprop::*;


pub struct TechNode {
  tech_name: Tech,
  available: bool,
  is_prereq: bool,
  researched: bool,
  acquired_in_edges: Option<Vec<Tech> >,
  unacquired_in_edges: Option<Vec<Tech> >,
  out_edges: Option<Vec<Tech> >,
}

#[derive(Clone, Copy, Debug)]
pub enum TechNodeErrs {
  AlreadyResearched,
  AttemptToLinkToPrereq,
  AttemptToLinkToSelf,
  IllegallyMarkedAvailable,
  InEdgesTechAlreadyExists,
  OutEdgesTechAlreadyExists,
  TechAlreadyMarkedAcquired,
  TechNotAvailable,
  TechNotFound,
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

  pub fn add_in_edge(&mut self, tech: Tech) -> Result<(), TechNodeErrs> {
    if self.is_prereq {
      return Err(TechNodeErrs::AttemptToLinkToPrereq);
    }

    if self.tech_name == tech {
      return Err(TechNodeErrs::AttemptToLinkToSelf);
    }

    if let Some(ref ai_edges) = self.acquired_in_edges {
      if ai_edges.binary_search(&tech).is_ok() {
        return Err(TechNodeErrs::InEdgesTechAlreadyExists);
      }
    }

    let ui_edges: &mut Vec<Tech> = self.unacquired_in_edges.get_or_insert(Vec::with_capacity(5) );
    ui_edges.binary_search(&tech).
    map(|_| true).
    or_else(|i| {
      ui_edges.insert(i, tech);
      Ok(false)
    }).
    and_then(|p| if p {Err(TechNodeErrs::InEdgesTechAlreadyExists)} else {Ok(() )})  // Result<(), TechNodeErrs>
  }

  pub fn add_out_edge(&mut self, tech: Tech) -> Result<(), TechNodeErrs> {
    if self.tech_name == tech {
      return Err(TechNodeErrs::AttemptToLinkToSelf);
    }
    
    let o_edges: &mut Vec<Tech> = self.out_edges.get_or_insert(Vec::with_capacity(5) );
    o_edges.binary_search(&tech).
    map(|_| true).
    or_else(|i| {
      o_edges.insert(i, tech);
      Ok(false)
    }).
    and_then(|p| if p {Err(TechNodeErrs::OutEdgesTechAlreadyExists)} else {Ok(() )})
  }

  pub fn get_acquired_in_edges(&self) -> Option<&Vec<Tech> > {
    self.acquired_in_edges.as_ref()
  }

  pub fn get_all_in_edges(&self) -> Option<Vec<Tech> > {
    self.get_acquired_in_edges().
    map(|rv| {
      let mut v = rv.clone();
      self.get_unacquired_in_edges().
      map(|rv| v.extend_from_slice(rv) );
      v
    }).
    or_else(||
      self.get_unacquired_in_edges().
      map(|rv| rv.clone() )
    )
  }

  pub fn get_out_edges(&self) -> Option<&Vec<Tech> > {
    self.out_edges.as_ref()
  }

  pub fn get_tech_name(&self) -> Tech {
    self.tech_name
  }

  pub fn get_unacquired_in_edges(&self) -> Option<&Vec<Tech> > {
    self.unacquired_in_edges.as_ref()
  }

  pub fn mark_researched(&mut self) -> Result<(), TechNodeErrs> {
    if self.available {
      if self.unacquired_in_edges.is_some() {
        self.available = false;
        return Err(TechNodeErrs::IllegallyMarkedAvailable);
      }

      if !self.researched {
        self.researched = true;
      } else {
        return Err(TechNodeErrs::AlreadyResearched);
      }
    } else {
      return Err(TechNodeErrs::TechNotAvailable)
    }

    Ok(() )
  }

  pub fn move_link_acquired(&mut self, tech: Tech) -> Result<(), TechNodeErrs> {
    self.unacquired_in_edges.as_mut().
    ok_or(TechNodeErrs::TechNotFound).
    and_then(|rmv|
      rmv.binary_search(&tech).
      or(Err(TechNodeErrs::TechNotFound) ).
      map(|i| {
        let t = rmv.remove(i);
        (t, rmv.is_empty() )
      })
    ).
    map(|(t, p)| {
      if p {self.unacquired_in_edges = None;} Ok(t)
    }).
    or_else(|e|
      self.get_acquired_in_edges().
      ok_or(e).
      and_then(|rv|
        rv.binary_search(&tech).
        or(Err(e) ).
        and(Err(TechNodeErrs::TechAlreadyMarkedAcquired) )
      )
    )?.
    and_then(|t| {
      let rmv = self.acquired_in_edges.get_or_insert(Vec::with_capacity(5) );
      rmv.binary_search(&t).
      and(Ok(true) ).
      or_else(|i| {
        rmv.insert(i, t);
        Ok(false)
      }).
      and_then(|p| if !p {Ok(() )} else {Err(TechNodeErrs::TechAlreadyMarkedAcquired)} )
    })
  }
}


#[cfg(test)]
mod tests;
