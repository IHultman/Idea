use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Deref, Mul, Sub};
use std::sync::{Arc, Mutex};

use facility::Loc;

use resources::{ResourceAccum, ResourceUpkeep};

use worker::Worker;


const PRODUCT: f64 = 25.0;

type Ptr<T> = Arc<Mutex<T> >;


#[derive(Clone, Copy, Debug, Eq)]
pub struct Water {
  water: u64,
}

impl Water {
  pub fn new(water: u64) -> Self {
    Water {
      water: water,
    }
  }

  pub fn water(&self) -> u64 {self.water}
}

impl Add<Water> for Water {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Water {
      water: self.water + rhs.water,
    }
  }
}

impl Deref for Water {
  type Target = u64;

  fn deref(&self) -> &u64 {
    &self.water
  }
}

impl Mul<u64> for Water {
  type Output = Self;

  fn mul(self, rhs: u64) -> Self {
    Water {
      water: self.water * rhs,
    }
  }
}

impl Ord for Water {
  fn cmp(&self, other: &Self) -> Ordering {
    self.water.cmp(&other.water)
  }
}

impl PartialEq<Water> for Water {
  fn eq(&self, other: &Self) -> bool {
    self.water == other.water
  }
}

impl PartialOrd<Water> for Water {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.water.cmp(&other.water) )
  }
}

impl Sub<Water> for Water {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Water {
      water: if self.water > rhs.water {self.water - rhs.water} else {0}
    }
  }
}

impl ResourceAccum for Water {
  type Args = ();

  fn new_base() -> Water {
    Water::new(0)
  }

  fn produced(worker: Ptr<Worker>, args: () ) -> Self {
    let (lvl, energy) = {
      let worker = worker.lock().unwrap();
      (worker.get_skill_lvl(Loc::WaterProcessor), worker.get_energy() )
    };
    Water::new((PRODUCT * (lvl as f64) * energy ) as u64)
  }
}

impl ResourceUpkeep for Water {
  fn get_value(&self) -> u64 {
    self.water
  }

  fn upkeep_per_unit() -> Self {
    Water {
      water: 5,
    }
  }
}

#[cfg(test)]
mod tests;
