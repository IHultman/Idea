use std::collections::HashMap;
use std::cmp::{Ord, PartialEq};
use std::fmt::Debug;
use std::ops::{Add, Mul, Sub};
use std::sync::{Arc, Mutex};
use std::thread;
use resources::ResourceAccum;
use super::*;
use worker::Worker;

type Ptr<T> = Arc<Mutex<T> >;


#[test]
fn facility_loc_test() {
  assert_eq!(Loc::Academy, Loc::Academy);
  assert_ne!(Loc::Academy, Loc::Farm);
  assert_ne!(Loc::Academy, Loc::Lab);
  assert_ne!(Loc::Academy, Loc::Mine);
  assert_ne!(Loc::Academy, Loc::Ship);
  assert_ne!(Loc::Academy, Loc::WaterProcessor);

  assert_ne!(Loc::Farm, Loc::Academy);
  assert_eq!(Loc::Farm, Loc::Farm);
  assert_ne!(Loc::Farm, Loc::Lab);
  assert_ne!(Loc::Farm, Loc::Mine);
  assert_ne!(Loc::Farm, Loc::Ship);
  assert_ne!(Loc::Farm, Loc::WaterProcessor);

  assert_ne!(Loc::Lab, Loc::Academy);
  assert_ne!(Loc::Lab, Loc::Farm);
  assert_eq!(Loc::Lab, Loc::Lab);
  assert_ne!(Loc::Lab, Loc::Mine);
  assert_ne!(Loc::Lab, Loc::Ship);
  assert_ne!(Loc::Lab, Loc::WaterProcessor);

  assert_ne!(Loc::Mine, Loc::Academy);
  assert_ne!(Loc::Mine, Loc::Farm);
  assert_ne!(Loc::Mine, Loc::Lab);
  assert_eq!(Loc::Mine, Loc::Mine);
  assert_ne!(Loc::Mine, Loc::Ship);
  assert_ne!(Loc::Mine, Loc::WaterProcessor);

  assert_ne!(Loc::Ship, Loc::Academy);
  assert_ne!(Loc::Ship, Loc::Farm);
  assert_ne!(Loc::Ship, Loc::Lab);
  assert_ne!(Loc::Ship, Loc::Mine);
  assert_eq!(Loc::Ship, Loc::Ship);
  assert_ne!(Loc::Ship, Loc::WaterProcessor);

  assert_ne!(Loc::WaterProcessor, Loc::Academy);
  assert_ne!(Loc::WaterProcessor, Loc::Farm);
  assert_ne!(Loc::WaterProcessor, Loc::Lab);
  assert_ne!(Loc::WaterProcessor, Loc::Mine);
  assert_ne!(Loc::WaterProcessor, Loc::Ship);
  assert_eq!(Loc::WaterProcessor, Loc::WaterProcessor);
}
