use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Deref, Mul, Sub};
use std::sync::{Arc, Mutex};

use facility::location::Loc;

use resources::{ResourceAccum, ResourceUpkeep};

use worker::Worker;


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
  fn new_base() -> Water {
    Water::new(0)
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
