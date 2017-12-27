use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Deref, Mul, Sub};
use std::sync::{Arc, Mutex};

use worker::Worker;


pub mod crystals;
pub mod food;
pub mod water;


type Ptr<T> = Arc<Mutex<T> >;


pub trait ResourceAccum where
  Self: Add<Self, Output=Self> + Sized
{
  fn new_base() -> Self;
  fn produced(Ptr<Worker>) -> Self;
}

pub trait ResourceUpkeep where
  Self: ResourceAccum +
        Sub<Self, Output=Self> +
        Mul<u64, Output=Self> +
        Copy +
        Ord
{
  fn get_value(&self) -> u64;
  fn upkeep_per_unit() -> Self;
}
