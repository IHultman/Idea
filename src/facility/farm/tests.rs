use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use facility::{Facility, Loc, Producer, EXP};
use resources::food::*;
use super::*;
use worker::*;

type Ptr<T> = Arc<Mutex<T> >;


#[test]
fn farm_facility_test_1() {
  let mut farm = Farm::new();

  for i in 0..10 {
    farm.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  assert_eq!(farm.borrow_crew_hash()
                 .values()
                 .fold(0, |count, worker| {
                   assert_eq!(worker.lock().unwrap().get_loc(), Some(Loc::Farm) );
                   count + 1
                 }), 10);
}

#[test]
#[should_panic]
fn farm_facility_test_2() {
  let mut farm = Farm::new();

  for i in 0..10 {
    farm.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  let worker = farm.remove_unit(15).unwrap();
}

#[test]
fn farm_facility_test_3() {
  let mut farm = Farm::new();

  for i in 0..10 {
    farm.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  let worker = farm.remove_unit(2).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 2);

  let worker = farm.remove_unit(0).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 0);

  let worker = farm.remove_unit(9).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 9);

  let worker = farm.remove_unit(5).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 5);

  let mut nums_left = farm.borrow_crew_hash()
                          .keys()
                          .map(|&x| x)
                          .collect::<Vec<u8> >();
  nums_left.sort();
  assert_eq!(*nums_left, [1,3,4,6,7,8]);
}

#[test]
#[should_panic]
fn farm_facility_test_4() {
  let mut farm = Farm::new();

  farm.add_unit(Arc::new(Mutex::new(Worker::new(17))) );
  farm.remove_unit(17);
  farm.remove_unit(17).unwrap();
}

#[test]
fn farm_facility_test_5() {
  let mut farm = Farm::new();
  let multiplier = 8;

  for i in 0..10 {
    farm.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  for _ in 0..multiplier {
    farm.exp_up();
  }

  let mut test_unit = Worker::new(0);
  test_unit.set_loc(Loc::Farm);
  test_unit.add_exp(multiplier * EXP);
  let test_unit_lvl = test_unit.get_skill_lvl(Loc::Farm);

  for (_, worker) in farm.borrow_crew_hash() {
    assert_eq!(worker.lock().unwrap().get_skill_lvl(Loc::Farm), test_unit_lvl);
  }
}

#[test]
fn farm_producer_test_1() {
  let mut farm = Farm::new();

  for i in 0..10 {
    farm.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  //assert_eq!(farm.harvest(), )
}
