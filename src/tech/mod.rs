use self::crystalprop::{CrystalProperties, CrystalProperty, Tech};
use self::technode::*;

use std::convert::From;
use std::ops::{Index, IndexMut};


pub mod crystalprop;
mod technode;


const TECHLISTSIZE: usize = 18;


pub struct TechDiGraph {
  tech_list: Vec<Option<TechNode> >,
}

#[derive(Clone, Copy, Debug)]
pub enum TechDiGraphErrs {
  IllegalLink(Tech, Tech),
  LinkToTechAlreadyAcquired(Tech, Tech),
  LinkToTechAlreadyInsertedToGraph(Tech, Tech),
  OtherEvent(Tech),
  PrereqAlreadyInsertedToGraph(Tech),
  TechAlreadyResearched(Tech),
  TechNotAvailable(Tech),
  TechNotFound(Tech),
}

impl From<TechNodeErrs> for TechDiGraphErrs {
  fn from(e: TechNodeErrs) -> Self {
    match e {
      TechNodeErrs::AttemptToLinkToPrereq(t1, t2)     => TechDiGraphErrs::IllegalLink(t1, t2),
      TechNodeErrs::AttemptToLinkToSelf(t)            => TechDiGraphErrs::IllegalLink(t, t),
      TechNodeErrs::IllegallyMarkedAvailable(t)       => TechDiGraphErrs::TechNotAvailable(t),
      TechNodeErrs::InEdgesTechAlreadyExists(t1, t2)  => TechDiGraphErrs::LinkToTechAlreadyInsertedToGraph(t1, t2),
      TechNodeErrs::LinkAlreadyAcquired(t1, t2)       => TechDiGraphErrs::LinkToTechAlreadyAcquired(t1, t2),
      TechNodeErrs::OtherEvent(t)                     => TechDiGraphErrs::OtherEvent(t),
      TechNodeErrs::OutEdgesTechAlreadyExists(t1, t2) => TechDiGraphErrs::LinkToTechAlreadyInsertedToGraph(t1, t2),
      TechNodeErrs::TechAlreadyResearched(t)          => TechDiGraphErrs::TechAlreadyResearched(t),
      TechNodeErrs::TechNotAvailable(t)               => TechDiGraphErrs::TechNotAvailable(t),
      TechNodeErrs::TechNotFound(t)                   => TechDiGraphErrs::TechNotFound(t),
    }
  }
}

impl TechDiGraph {
  pub fn new() -> TechDiGraph {
    let mut tdg = TechDiGraph {
      tech_list: Vec::with_capacity(TECHLISTSIZE),
    };

    for i in 0..TECHLISTSIZE {
      tdg.tech_list.push(None);
    }

    tdg
  }

  pub fn add_prereq(&mut self, tech: Tech) -> Result<(), TechDiGraphErrs> {
    if self[tech].is_none() {
      self[tech] = Some(TechNode::new(tech, true) );
    } else {
      return Err(TechDiGraphErrs::PrereqAlreadyInsertedToGraph(tech) );
    }

    Ok(() )
  }

  pub fn add_advanced_link(&mut self, from_t: Tech, to_t: Tech) -> Result<(), TechDiGraphErrs> {
    self[from_t].as_mut().
      ok_or(TechDiGraphErrs::TechNotFound(from_t) ).
      and_then(|tn_rm|
        tn_rm.add_out_edge(to_t).
        map_err(|e| TechDiGraphErrs::from(e))
      ).
      or_else(|e| if let TechDiGraphErrs::LinkToTechAlreadyInsertedToGraph(..) = e {Ok(() )} else {Err(e)})?;

    self[to_t].get_or_insert(TechNode::new(to_t, false) ).
      add_in_edge(from_t).
      map_err(|e| TechDiGraphErrs::from(e) )
  }

  pub fn get_node_ref(&self, tech: Tech) -> Option<&TechNode> {

    None
  }

  pub fn get_node_mut(&mut self, tech: Tech) -> Option<&mut TechNode> {

    None
  }
}

impl Index<Tech> for TechDiGraph {
  type Output = Option<TechNode>;

  fn index(&self, index: Tech) -> &Option<TechNode> {
    match index {
      Tech::Antimatter     => &self.tech_list[0],
      Tech::Earthquake     => &self.tech_list[1],
      Tech::EnergyShield   => &self.tech_list[2],
      Tech::Flight         => &self.tech_list[3],
      Tech::Teleport       => &self.tech_list[4],
      Tech::Walls          => &self.tech_list[5],
      Tech::WarningSystem  => &self.tech_list[6],
      Tech::WeatherControl => &self.tech_list[7],
      Tech::T1             => &self.tech_list[8],
      Tech::T2             => &self.tech_list[9],
      Tech::T3             => &self.tech_list[10],
      Tech::T4             => &self.tech_list[11],
      Tech::T5             => &self.tech_list[12],
      Tech::T6             => &self.tech_list[13],
      Tech::T7             => &self.tech_list[14],
      Tech::T8             => &self.tech_list[15],
      Tech::T9             => &self.tech_list[16],
      Tech::T10            => &self.tech_list[17],
    }
  }
}

impl IndexMut<Tech> for TechDiGraph {
  fn index_mut(&mut self, index: Tech) -> &mut Option<TechNode> {
    match index {
      Tech::Antimatter     => &mut self.tech_list[0],
      Tech::Earthquake     => &mut self.tech_list[1],
      Tech::EnergyShield   => &mut self.tech_list[2],
      Tech::Flight         => &mut self.tech_list[3],
      Tech::Teleport       => &mut self.tech_list[4],
      Tech::Walls          => &mut self.tech_list[5],
      Tech::WarningSystem  => &mut self.tech_list[6],
      Tech::WeatherControl => &mut self.tech_list[7],
      Tech::T1             => &mut self.tech_list[8],
      Tech::T2             => &mut self.tech_list[9],
      Tech::T3             => &mut self.tech_list[10],
      Tech::T4             => &mut self.tech_list[11],
      Tech::T5             => &mut self.tech_list[12],
      Tech::T6             => &mut self.tech_list[13],
      Tech::T7             => &mut self.tech_list[14],
      Tech::T8             => &mut self.tech_list[15],
      Tech::T9             => &mut self.tech_list[16],
      Tech::T10            => &mut self.tech_list[17],
    }
  }
}


#[cfg(test)]
mod tests;
