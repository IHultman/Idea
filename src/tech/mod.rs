use std::boxed::Box;
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
  TimeWarp //Purple
  UltraDense, //Silver
  Vegetation, //Yellow
  YellowPropertyII //Yellow
}

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


pub struct CrystalProperties {
  properties: [[bool]; 5]; 12]
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
      CrystalProperties::Anomaly             => &self.properties[0],
      CrystalProperties::Combustible         => &self.properties[1],
      CrystalProperties::Conductivity        => &self.properties[2],
      CrystalProperties::GravityManipulation => &self.properties[3],
      CrystalProperties::Magnetism           => &self.properties[4],
      CrystalProperties::RockFormation       => &self.properties[5],
      CrystalProperties::SignalTransmission  => &self.properties[6],
      CrystalProperties::StoreEnergy         => &self.properties[7],
      CrystalProperties::TimeWarp            => &self.properties[8],
      CrystalProperties::UltraDense          => &self.properties[9],
      CrystalProperties::Vegetation          => &self.properties[10],
      CrystalProperties::YellowPropertyII    => &self.properties[11],
    }
  }
}

impl IndexMut<CrystalProperty> for CrystalProperties {
  fn index_mut(&self, index: CrystalProperty) -> &mut [bool; 5] {
    match index {
      CrystalProperties::Anomaly             => &mut self.properties[0],
      CrystalProperties::Combustible         => &mut self.properties[1],
      CrystalProperties::Conductivity        => &mut self.properties[2],
      CrystalProperties::GravityManipulation => &mut self.properties[3],
      CrystalProperties::Magnetism           => &mut self.properties[4],
      CrystalProperties::RockFormation       => &mut self.properties[5],
      CrystalProperties::SignalTransmission  => &mut self.properties[6],
      CrystalProperties::StoreEnergy         => &mut self.properties[7],
      CrystalProperties::TimeWarp            => &mut self.properties[8],
      CrystalProperties::UltraDense          => &mut self.properties[9],
      CrystalProperties::Vegetation          => &mut self.properties[10],
      CrystalProperties::YellowPropertyII    => &mut self.properties[11],
    }
  }
}


pub struct TechDiGraph {
  base: Vec<TechNode>,
}

impl TechDiGraph {
  pub fn new() -> Self {
    TechDiGraph {
      base: Vec::new(),
    }
  }
}

impl<'a> IntoIterator for  &'a mut TechDiGraph {
  type Item = &'a mut Arc<TechNode>;
  type IntoIter = IterMut<'a, Arc<TechNode> >;

  fn into_iter(self) -> Self::IntoIter {
    for tech in &mut self.base {

    }
  }
}


pub struct TechNode {
  tech_name: Tech,
  researched: bool,
  acquired_in_edges: Option<Vec<Weak<TechNode>> >,
  unacquired_in_edges: Option<Vec<Weak<TechNode>> >,
  out_edges: Option<Vec<Arc<TechNode>> >,
}

impl TechNode {
  new(name: Tech) -> Self {
    TechNode {
      tech_name: name,
      researched: false,
      acquired_in_edges: None,
      unacquired_in_edges: None,
      out_edges: None,
    }
  }

  fn add_in_edge(&mut self, tech: Weak<TechNode>) {
    let vec: &mut Vec<Weak<TechNode> > = self.unacquired_in_edges.get_or_insert(Vec::new() );
    vec.push(tech);
  }

  fn add_out_edge(&mut self, tech: Arc<TechNode>) {
    let vec: &mut Vec<Arc<TechNode> > = self.out_edges.get_or_insert(Vec::new() );
    vec.push(tech);
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
