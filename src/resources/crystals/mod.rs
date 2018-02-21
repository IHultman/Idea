use std::cmp::PartialEq;
use std::ops::{Add, Index, IndexMut};
use std::sync::{Arc, Mutex};

use facility::location::Loc;

use resources::ResourceAccum;

use worker::Worker;


const PRODUCT: f64 = 25.0;

type Ptr<T> = Arc<Mutex<T> >;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
  Blue,
  Energy,
  Green,
  Purple,
  Red,
  Silver,
  Yellow,
}

#[derive(Clone, Copy, Debug)]
pub struct CrystalBatch {
  blue: u64,
  energy: u64,
  green: u64,
  purple: u64,
  red: u64,
  silver: u64,
  yellow: u64,
}

impl CrystalBatch {
  pub fn new_random(mut crystals: u64, color: Color) -> Self {
    let mut other_colors = match color {
      Color::Blue   => vec![Color::Green, Color::Purple, Color::Red, Color::Silver, Color::Yellow],
      Color::Energy => panic!(),
      Color::Green  => vec![Color::Blue, Color::Purple, Color::Red, Color::Silver, Color::Yellow],
      Color::Purple => vec![Color::Green, Color::Blue, Color::Red, Color::Silver, Color::Yellow],
      Color::Red    => vec![Color::Green, Color::Purple, Color::Blue, Color::Silver, Color::Yellow],
      Color::Silver => vec![Color::Green, Color::Purple, Color::Red, Color::Blue, Color::Yellow],
      Color::Yellow => vec![Color::Green, Color::Purple, Color::Red, Color::Silver, Color::Blue],
    };
    let random_colors_list = (1..6).rev()
                                   .map(|x| other_colors.remove(::rand::random::<usize>() % x) )
                                   .collect::<Vec<Color> >();
    let mut batch = CrystalBatch {
      blue: 0,
      energy: 0,
      green: 0,
      purple: 0,
      red: 0,
      silver: 0,
      yellow: 0,
    };
    {
      let mut add_to_color = |range: (f64, f64), color: Color| {
        let rand_percent = range.0 + ((::rand::random::<f64>() ) * (range.1 - range.0) );
        let amt_crystals = (rand_percent * (crystals as f64) ) as u64;
        batch[color] = amt_crystals;
        crystals -= amt_crystals;
      };
      add_to_color((0.40, 0.75), color);
      add_to_color((0.0, 0.75), Color::Energy);
      for &color in random_colors_list.iter() {
        add_to_color((0.0, 1.0), color);
      }
    }
    batch[color] += crystals;
    batch
  }
}

impl Index<Color> for CrystalBatch {
  type Output = u64;

  fn index(&self, index: Color) -> &u64 {
    match index {
      Color::Blue   => &self.blue,
      Color::Energy => &self.energy,
      Color::Green  => &self.green,
      Color::Purple => &self.purple,
      Color::Red    => &self.red,
      Color::Silver => &self.silver,
      Color::Yellow => &self.yellow,
    }
  }
}

impl IndexMut<Color> for CrystalBatch {
  fn index_mut(&mut self, index: Color) -> &mut u64 {
    match index {
      Color::Blue   => &mut self.blue,
      Color::Energy => &mut self.energy,
      Color::Green  => &mut self.green,
      Color::Purple => &mut self.purple,
      Color::Red    => &mut self.red,
      Color::Silver => &mut self.silver,
      Color::Yellow => &mut self.yellow,
    }
  }
}

impl Add<CrystalBatch> for CrystalBatch {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    CrystalBatch {
      blue:   self[Color::Blue] + rhs[Color::Blue],
      energy: self[Color::Energy] + rhs[Color::Energy],
      green:  self[Color::Green] + rhs[Color::Green],
      purple: self[Color::Purple] + rhs[Color::Purple],
      red:    self[Color::Red] + rhs[Color::Red],
      silver: self[Color::Silver] + rhs[Color::Silver],
      yellow: self[Color::Yellow] + rhs[Color::Yellow],
    }
  }
}

impl PartialEq<CrystalBatch> for CrystalBatch {
  fn eq(&self, rhs: &Self) -> bool {
    (self.blue   == rhs.blue) &&
    (self.energy == rhs.energy) &&
    (self.green  == rhs.green) &&
    (self.purple == rhs.purple) &&
    (self.red    == rhs.red) &&
    (self.silver == rhs.silver) &&
    (self.yellow == rhs.yellow)
  }
}

impl ResourceAccum for CrystalBatch {
  fn new_base() -> Self {
    CrystalBatch {
      blue: 0,
      energy: 0,
      green: 0,
      purple: 0,
      red: 0,
      silver: 0,
      yellow: 0,
    }
  }
}

#[cfg(test)]
mod tests;
