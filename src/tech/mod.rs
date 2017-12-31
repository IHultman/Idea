use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::iter::IntoIterator;
use std::ops::{Index, IndexMut};
use std::slice::IterMut;
use std::sync::{Arc, Mutex, Weak};


type Ptr<T> = Arc<Mutex<T> >;


enum CrystalProperty {
  Anomaly, //Purple
  Combustible, //Red
  Conductivity, //Green
  GravityManipulation, //Blue
  Magnetism, //Blue
  RockFormation, //Silver
  SignalTransmission, //Green
  StoreEnergy, //Red
  TimeWarp, //Purple
  UltraDense, //Silver
  Vegetation, //Yellow
  YellowPropertyII //Yellow
}

#[derive(Clone, Copy, Debug, Eq)]
enum Tech {
  Antimatter,
  Earthquake,
  EnergyShield,
  Flight,
  Teleport,
  Walls,
  WarningSystem,
  WeatherControl,
}

impl PartialEq<Tech> for Tech {
  fn eq(&self, other: &Self) -> bool {
    match (*self) {
      Tech::Antimatter     => if let Tech::Antimatter = (*other) {true} else {false}
      Tech::Earthquake     => if let Tech::Earthquake = (*other) {true} else {false}
      Tech::EnergyShield   => if let Tech::EnergyShield = (*other) {true} else {false}
      Tech::Flight         => if let Tech::Flight = (*other) {true} else {false}
      Tech::Teleport       => if let Tech::Teleport = (*other) {true} else {false}
      Tech::Walls          => if let Tech::Walls = (*other) {true} else {false}
      Tech::WarningSystem  => if let Tech::WarningSystem = (*other) {true} else {false}
      Tech::WeatherControl => if let Tech::WeatherControl = (*other) {true} else {false}
    }
  }
}


pub struct CrystalProperties {
  properties: [[bool; 5]; 12]
}

impl CrystalProperties {
  fn new() -> Self {
    CrystalProperties {
      properties: [[false; 5]; 12],
    }
  }
}

impl Index<CrystalProperty> for CrystalProperties {
  type Output = [bool; 5];

  fn index(&self, index: CrystalProperty) -> &[bool; 5] {
    match index {
      CrystalProperty::Anomaly             => &self.properties[0],
      CrystalProperty::Combustible         => &self.properties[1],
      CrystalProperty::Conductivity        => &self.properties[2],
      CrystalProperty::GravityManipulation => &self.properties[3],
      CrystalProperty::Magnetism           => &self.properties[4],
      CrystalProperty::RockFormation       => &self.properties[5],
      CrystalProperty::SignalTransmission  => &self.properties[6],
      CrystalProperty::StoreEnergy         => &self.properties[7],
      CrystalProperty::TimeWarp            => &self.properties[8],
      CrystalProperty::UltraDense          => &self.properties[9],
      CrystalProperty::Vegetation          => &self.properties[10],
      CrystalProperty::YellowPropertyII    => &self.properties[11],
    }
  }
}

impl IndexMut<CrystalProperty> for CrystalProperties {
  fn index_mut(&mut self, index: CrystalProperty) -> &mut [bool; 5] {
    match index {
      CrystalProperty::Anomaly             => &mut self.properties[0],
      CrystalProperty::Combustible         => &mut self.properties[1],
      CrystalProperty::Conductivity        => &mut self.properties[2],
      CrystalProperty::GravityManipulation => &mut self.properties[3],
      CrystalProperty::Magnetism           => &mut self.properties[4],
      CrystalProperty::RockFormation       => &mut self.properties[5],
      CrystalProperty::SignalTransmission  => &mut self.properties[6],
      CrystalProperty::StoreEnergy         => &mut self.properties[7],
      CrystalProperty::TimeWarp            => &mut self.properties[8],
      CrystalProperty::UltraDense          => &mut self.properties[9],
      CrystalProperty::Vegetation          => &mut self.properties[10],
      CrystalProperty::YellowPropertyII    => &mut self.properties[11],
    }
  }
}


pub struct TechDiGraph {
  prereqs: Vec<TechNode>,
  advanced: Vec<TechNode>,
}

/*
impl TechDiGraph {
  pub fn new() -> Self {
    TechDiGraph {
      prereqs: Vec::new(),
      advanced: Vec::new(),
    }
  }

  pub fn add_prereq(&mut self, tech: Tech) {
    if Self::get_node_ref(&self.prereqs, tech).is_none() {
      self.prereqs.push(TechNode::new(tech) );
    }
  }

  pub fn add_advanced_link(&mut self, prereq: Tech, adv: Tech) -> Result<(), String> {
    let prereq_ref_mut: Option<&mut TechNode> = Self::get_node_ref_mut(&mut self.prereqs);
    if prereq_ref_mut.is_none() {
      return Err("Invalid prerequisite tech name".to_string() );
    }

    let mut adv_ref_mut: Option<&mut TechNode> = Self::get_node_ref_mut(&mut self.advanced);
    if adv_ref_mut.is_none() {
      self.advanced.push(TechNode::new(adv) );
      adv_ref_mut = self.advanced.last_mut();
    }

    let prereq_ref_mut: &mut TechNode = prereq_ref_mut.unwrap();
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

  fn get_node_ref<'a>(nodes: &'a [TechNode], tech: Tech) -> Option<&'a TechNode> {
    for node in nodes {
      if node.tech_name == tech {
        return Some(node);
      }
    }
    None
  }

  fn get_node_ref_mut<'a>(nodes: &'a mut [TechNode], tech: Tech) -> Option<&'a mut TechNode> {
    for node in nodes {
      if node.tech_name == tech {
        return Some(node);
      }
    }
    None
  }

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
}
*/

pub struct TechNode {
  tech_name: Tech,
  available: bool,
  researched: bool,
  acquired_in_edges: Option<Vec<*const TechNode> >,
  unacquired_in_edges: Option<Vec<*const TechNode> >,
  out_edges: Option<Vec<*mut TechNode> >,
}

/*
impl TechNode {
  new(name: Tech) -> Self {
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
      self.researched = true;
    } else {
      return Err("Prerequisites not met".to_string() );
    }

    if let Some(ref mut node_ptrs) = self.out_edges {
      for node in node_ptrs {
        unsafe {
          node.move_link_acquired(self as *const Technode);
        }
      }
    }
  }

  unsafe fn move_link_acquired(&mut self, tech_ptr: *const TechNode) -> Result<(), String)> {
    let mut empty = false;
    match self.unacquired_in_edges {
      Some(ref mut node_ptrs) => {
        let to_move = None;
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
          return Err("Link to tech not found");
        }
      },
      None => {
        return Err("Link to tech not found");
      },
    }

    if empty {
      self.unacquired_in_edges = None;
      self.available = true;
    }

    Ok(() )
  }
}
*/

pub struct TechState {
  properties: CrystalProperties,
  tech_graph: TechDiGraph,
}

/*
impl TechState {
  pub fn new() -> Self {
    TechState {
      properties: CrystalProperties::new(),
      tech_graph: TechDiGraph::new(),
    }
  }
}
*/

#[cfg(test)]
mod tests {
  #[test]
  fn tech_test() {

  }
}
