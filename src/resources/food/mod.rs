use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Deref, Mul, Sub};
use std::sync::{Arc, Mutex};

use facility::location::Loc;

use resources::{ResourceAccum, ResourceUpkeep};

use worker::Worker;


type Ptr<T> = Arc<Mutex<T> >;


#[derive(Clone, Copy, Debug, Eq)]
pub struct Food {
  food: u64,
}

impl Food {
  pub fn new(food: u64) -> Self {
    Food {
      food: food,
    }
  }

  pub fn food(&self) -> u64 {self.food}
}

impl Add<Food> for Food {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Food {
      food: self.food + rhs.food,
    }
  }
}

impl Deref for Food {
  type Target = u64;

  fn deref(&self) -> &u64 {
    &self.food
  }
}

impl Mul<u64> for Food {
  type Output = Self;

  fn mul(self, rhs: u64) -> Self {
    Food {
      food: self.food * rhs,
    }
  }
}

impl Ord for Food {
  fn cmp(&self, other: &Self) -> Ordering {
    self.food.cmp(&other.food)
  }
}

impl PartialEq<Food> for Food {
  fn eq(&self, other: &Self) -> bool {
    self.food == other.food
  }
}

impl PartialOrd<Food> for Food {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.food.cmp(&other.food) )
  }
}

impl Sub<Food> for Food {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Food {
      food: if self.food > rhs.food {self.food - rhs.food} else {0}
    }
  }
}

impl ResourceAccum for Food {
  fn new_base() -> Food {
    Food::new(0)
  }
}

impl ResourceUpkeep for Food {
  fn get_value(&self) -> u64 {
    self.food
  }

  fn upkeep_per_unit() -> Self {
    Food {
      food: 5,
    }
  }
}

#[cfg(test)]
mod tests;
