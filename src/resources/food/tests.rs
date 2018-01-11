use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Deref, Mul, Sub};
use std::sync::{Arc, Mutex};
use facility::Loc;
use resources::{ResourceAccum, ResourceUpkeep};
use super::*;
use worker::*;

type Ptr<T> = Arc<Mutex<T> >;


#[test]
fn food_ordering_test_1() {
  let f1 = Food::new(0);
  let f2 = Food::new(1);

  if f1 < f2 {} else {panic!()}
}

#[test]
fn food_ordering_test_2() {
  let f1 = Food::new(5);
  let f2 = Food::new(5);

  assert_eq!(f1, f2);
}

#[test]
#[should_panic]
fn food_ordering_test_3() {
  let f1 = Food::new(0);
  let f2 = Food::new(1);

  if f1 >= f2 {} else {panic!()}
}

#[test]
fn food_add_test() {
  let f1 = Food::new(7);
  let f2 = Food::new(8);

  assert_eq!(f1 + f2, Food::new(15) );
}

#[test]
fn food_mul_test_1() {
  let f1 = Food::new(4);

  assert_eq!(f1 * 7, Food::new(28) );
}

#[test]
fn food_mul_test_2() {
  let f1 = Food::new(0);

  assert_eq!(f1 * 92, Food::new_base() );
}

#[test]
fn food_mul_test_3() {
  let f1 = Food::new(249);

  assert_eq!(f1 * 0, Food::new(0) );
}

#[test]
fn food_sub_test_1() {
  let f1 = Food::new(73);
  let f2 = Food::new(56);

  assert_eq!(f1 - f2, Food::new(17) );
}

#[test]
fn food_sub_test_2() {
  let f1 = Food::new(73);
  let f2 = Food::new(56);

  assert_eq!(f2 - f1, Food::new(0) );
}

#[test]
fn food_sub_test_3() {
  let f1 = Food::new(0);
  let f2 = Food::new(0);

  assert_eq!(f1 - f2, f2 - f1);
}

/*
#[test]
fn food_resorceaccum_test() {
  let mut food = Food::new_base();

  let mut worker = Arc::new(Mutex::new(Worker::new(0)) );
  food = food + Food::produced(worker.clone(), () );
  assert_eq!(food, Food::new(PRODUCT as u64) );
  {
    worker.lock().unwrap().remove_energy(0.5);
  }
  food = food + Food::produced(worker.clone(), () );
  assert_eq!(food, Food::new((PRODUCT * 1.5) as u64) );
  {
    let mut worker = worker.lock().unwrap();
    worker.set_loc(Loc::Farm);
    worker.add_exp(100);
  }
  food = food + Food::produced(worker.clone(), () );
  assert_eq!(food, Food::new((PRODUCT * 3.0 - 1.0) as u64) )
}
*/
