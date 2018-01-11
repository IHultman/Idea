use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Deref, Mul, Sub};
use std::sync::{Arc, Mutex};
use facility::Loc;
use resources::{ResourceAccum, ResourceUpkeep};
use super::*;
use worker::*;

type Ptr<T> = Arc<Mutex<T> >;


#[test]
fn water_ordering_test_1() {
  let w1 = Water::new(0);
  let w2 = Water::new(1);

  if w1 < w2 {} else {panic!()}
}

#[test]
fn water_ordering_test_2() {
  let w1 = Water::new(5);
  let w2 = Water::new(5);

  assert_eq!(w1, w2);
}

#[test]
#[should_panic]
fn water_ordering_test_3() {
  let w1 = Water::new(0);
  let w2 = Water::new(1);

  if w1 >= w2 {} else {panic!()}
}

#[test]
fn water_add_test() {
  let w1 = Water::new(1000);
  let w2 = Water::new(74);

  assert_eq!(w1 + w2, Water::new(1074) );
}

#[test]
fn water_mul_test_1() {
  let w1 = Water::new(0);

  assert_eq!(w1 * 9, Water::new_base() );
}

#[test]
fn water_mul_test_2() {
  let w1 = Water::new(5);

  assert_eq!(w1 * 0, Water::new(0) );
}

#[test]
fn water_mul_test_3() {
  let w1 = Water::new(3);

  assert_eq!(w1 * 17, Water::new(51) );
}

#[test]
fn water_sub_test_1() {
  let w1 = Water::new(0);
  let w2 = Water::new(0);

  assert_eq!(w1 - w2, w2 - w1);
}

#[test]
fn water_sub_test_2() {
  let w1 = Water::new(9);
  let w2 = Water::new(7);

  assert_eq!(w1 - w2, Water::new(2) );
}

#[test]
fn water_sub_test_3() {
  let w1 = Water::new(9);
  let w2 = Water::new(7);

  assert_eq!(w2 - w1, Water::new(0) );
}

/*
#[test]
fn water_resorceaccum_test() {
  let mut water = Water::new_base();

  let mut worker = Arc::new(Mutex::new(Worker::new(0)) );
  water = water + Water::produced(worker.clone(), () );
  assert_eq!(water, Water::new(PRODUCT as u64) );
  {
    worker.lock().unwrap().remove_energy(0.5);
  }
  water = water + Water::produced(worker.clone(), () );
  assert_eq!(water, Water::new((PRODUCT * 1.5) as u64) );
  {
    let mut worker = worker.lock().unwrap();
    worker.set_loc(Loc::WaterProcessor);
    worker.add_exp(100);
  }
  water = water + Water::produced(worker.clone(), () );
  assert_eq!(water, Water::new((PRODUCT * 3.0 - 1.0) as u64) )
}
*/
