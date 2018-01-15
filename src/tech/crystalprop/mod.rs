use std::ops::{Index, IndexMut};
use std::sync::{Arc, Mutex, Weak};

type Ptr<T> = Arc<Mutex<T> >;


pub enum CrystalProperty {
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
pub enum Tech {
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
  pub fn new() -> Self {
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


#[cfg(test)]
mod tests;
