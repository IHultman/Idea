use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use facility::{Facility, Loc, ResourceAccum, EXP};
use resources::crystals::{Color, CrystalBatch};
use super::*;
use worker::*;

type Ptr<T> = Arc<Mutex<T> >;


#[test]
fn lab_facility_test_1() {
  let mut lab = Lab::new();

  for i in 0..10 {
    lab.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  assert_eq!(lab.borrow_crew_hash()
                .values()
                .fold(0, |count, worker| {
                  assert_eq!(worker.lock().unwrap().get_loc(), Some(Loc::Lab) );
                  count + 1
                }), 10);
}

#[test]
#[should_panic]
fn lab_facility_test_2() {
  let mut lab = Lab::new();

  for i in 0..10 {
    lab.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  let worker = lab.remove_unit(10).unwrap();
}

#[test]
fn lab_facility_test_3() {
  let mut lab = Lab::new();

  for i in 0..10 {
    lab.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  let worker = lab.remove_unit(2).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 2);

  let worker = lab.remove_unit(0).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 0);

  let worker = lab.remove_unit(8).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 8);

  let worker = lab.remove_unit(9).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 9);

  let worker = lab.remove_unit(5).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 5);

  let mut nums_left = lab.borrow_crew_hash()
                         .keys()
                         .map(|&x| x)
                         .collect::<Vec<u8> >();
  nums_left.sort();
  assert_eq!(*nums_left, [1,3,4,6,7]);
}

#[test]
#[should_panic]
fn lab_facility_test_4() {
  let mut lab = Lab::new();

  lab.add_unit(Arc::new(Mutex::new(Worker::new(63))) );
  lab.remove_unit(63);
  lab.remove_unit(63).unwrap();
}

#[test]
fn lab_facility_test_5() {
  let mut lab = Lab::new();
  let multiplier = 10;

  for i in 0..10 {
    lab.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  for _ in 0..multiplier {
    lab.exp_up();
  }

  let mut test_unit = Worker::new(0);
  test_unit.set_loc(Loc::Lab);
  test_unit.add_exp(multiplier * EXP);
  let test_unit_lvl = test_unit.get_skill_lvl(Loc::Lab);

  for (_, worker) in lab.borrow_crew_hash() {
    assert_eq!(worker.lock().unwrap().get_skill_lvl(Loc::Lab), test_unit_lvl);
  }
}
