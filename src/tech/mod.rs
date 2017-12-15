use std::boxed::Box;
use std::ops::{Index, IndexMut};
use std::sync::{Arc, Mutex, Weak};


type Ptr<T> = Arc<Mutex<T> >;

const TECH_STAGE_COST: [u64; 10] = [0, 200, 450, 900, 2000, 4000, 9000, 20000, 50000, 100000];


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


pub struct TechState {
  properties: CrystalProperties,
}

impl TechState {
  pub fn new() -> Self {
    TechState {
      properties: CrystalProperties::new(),
      tech_graph: TechDiGraph::new(),
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

pub struct TechNode {
  tech_name: Tech,
  available: bool,
  acquired_in_edges: Option<Vec<Weak<TechNode>> >,
  unacquired_in_edges: Option<Vec<Weak<TechNode>> >,
  out_edges: Option<Vec<Arc<TechNode>> >,
}

impl TechNode {
  new(name: Tech) -> Self {
    tech_name: name,
    available: false,
    acquired_in_edges: None,
    unacquired_in_edges: None,
    out_edges: None,
  }

  fn add_in(&mut self, tech: Weak<TechNode>) {
    let vec: &mut Vec<Weak<TechNode> > = self.unacquired_in_edges.get_or_insert(Vec::new() );
    vec.push(tech);
  }

  fn add_out(&mut self, tech: Arc<TechNode>) {
    let vec: &mut Vec<Arc<TechNode> > = self.out_edges.get_or_insert(Vec::new() );
    vec.push(tech);
  }
}


#[derive(Debug, Clone, Copy)]
pub struct tech_progress {
  level: u8,
  crystals_used: u64,
}

impl tech_progress {
  pub fn new() -> Self {
    tech_progress {
      level: 1,
      crystals_used: 0,
    }
  }

  pub fn get_lvl(&self) -> u8 {
    self.level
  }

  pub fn use_crystals(&mut self, crystals: u64) {
    if self.level < 10 {
      self.crystals_used += crystals;
      if self.crystals_used > TECH_STAGE_COST[self.level as usize] {
        self.level += 1;
      }
    }
  }
}
