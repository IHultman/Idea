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
  AttemptToLinkToPrereq(Tech, Tech),
  AttemptToLinkToSelf(Tech),
  IllegallyMarkedAvailable(Tech),
  InEdgesTechAlreadyExists(Tech, Tech),
  LinkAlreadyAcquired(Tech, Tech),
  OtherEvent(Tech),
  OutEdgesTechAlreadyExists(Tech, Tech),
  TechAlreadyResearched(Tech),
  TechNotAvailable(Tech),
  TechNotFound(Tech),
}

impl TechNode {
  pub fn new(tech: Tech, is_prereq: bool) -> TechNode {
    TechNode {
      tech_name: tech,
      available: true,
      is_prereq: is_prereq,
      researched: false,
      acquired_in_edges: None,
      unacquired_in_edges: None,
      out_edges: None,
    }
  }

  pub fn add_in_edge(&mut self, tech: Tech) -> Result<(), TechNodeErrs> {
    let t_name = self.get_tech_name();
    if self.tech_name == tech {
      return Err(TechNodeErrs::AttemptToLinkToSelf(tech) );
    }

    if self.is_prereq {
      return Err(TechNodeErrs::AttemptToLinkToPrereq(tech, t_name) );
    }

    if let Some(ref ai_edges) = self.acquired_in_edges {
      if ai_edges.binary_search(&tech).is_ok() {
        return Err(TechNodeErrs::InEdgesTechAlreadyExists(tech, t_name) );
      }
    }

    let ui_edges: &mut Vec<Tech> = self.unacquired_in_edges.get_or_insert(Vec::with_capacity(5) );
    self.available = false;
    ui_edges.binary_search(&tech).
    map(|_| true).
    or_else(|i| {
      ui_edges.insert(i, tech);
      Ok(false)
    }).
    and_then(|p|
      if p {
        Err(TechNodeErrs::InEdgesTechAlreadyExists(tech, t_name) )
      } else {
        Ok(() )
    })
  }

  pub fn add_out_edge(&mut self, tech: Tech) -> Result<(), TechNodeErrs> {
    let t_name = self.get_tech_name();
    if t_name == tech {
      return Err(TechNodeErrs::AttemptToLinkToSelf(tech) );
    }

    let o_edges: &mut Vec<Tech> = self.out_edges.get_or_insert(Vec::with_capacity(5) );
    o_edges.binary_search(&tech).
    map(|_| true).
    or_else(|i| {
      o_edges.insert(i, tech);
      Ok(false)
    }).
    and_then(|p|
      if p {
        Err(TechNodeErrs::OutEdgesTechAlreadyExists(t_name, tech) )
      } else {
        Ok(() )
    })
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
      v.sort();
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
        let v = self.unacquired_in_edges.take().ok_or(TechNodeErrs::OtherEvent(self.get_tech_name()) )?;
        if !v.is_empty() {
          self.available = false;
          self.unacquired_in_edges = Some(v);
        }
        return Err(TechNodeErrs::IllegallyMarkedAvailable(self.get_tech_name()) );
      }

      if !self.researched {
        self.researched = true;
      } else {
        return Err(TechNodeErrs::TechAlreadyResearched(self.get_tech_name()) );
      }
    } else {
      return Err(TechNodeErrs::TechNotAvailable(self.get_tech_name()) )
    }

    Ok(() )
  }

  pub fn move_link_acquired(&mut self, tech: Tech) -> Result<(), TechNodeErrs> {
    let t_name = self.get_tech_name();
    self.unacquired_in_edges.as_mut().
    ok_or(TechNodeErrs::TechNotFound(tech) ).
    and_then(|rmv|
      rmv.binary_search(&tech).
      or(Err(TechNodeErrs::TechNotFound(tech) ) ).
      map(|i| {
        let t = rmv.remove(i);
        (t, rmv.is_empty() )
      })
    ).
    map(|(t, p)| {
      if p {self.unacquired_in_edges = None; self.available = true;} Ok(t)
    }).
    or_else(|e|
      self.get_acquired_in_edges().
      ok_or(e).
      and_then(|rv|
        rv.binary_search(&tech).
        or(Err(e) ).
        and(Err(TechNodeErrs::LinkAlreadyAcquired(tech, t_name)) )
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
      and_then(|p|
        if !p {
          Ok(() )
        } else {
          Err(TechNodeErrs::LinkAlreadyAcquired(tech, t_name) )
      })
    })
  }
}


#[cfg(test)]
mod tests;
