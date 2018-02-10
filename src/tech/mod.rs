use self::crystalprop::{CrystalProperties, CrystalProperty, Tech};
use self::technode::*;

use std::ops::{Index, IndexMut};


pub mod crystalprop;
mod technode;


const TECHLISTSIZE: usize = 18;


pub struct TechDiGraph {
  tech_list: Vec<Option<TechNode> >,
}

pub enum TechDiGraphErrs {
  //
  A,
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

    Ok(() )
  }

  pub fn add_advanced_link(&mut self) -> Result<(), TechDiGraphErrs> {

    Ok(() )
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
