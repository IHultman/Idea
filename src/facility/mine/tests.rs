use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use facility::{Facility, Loc, Producer, EXP};
use resources::crystals::*;
use super::*;
use worker::*;

type Ptr<T> = Arc<Mutex<T> >;


#[test]
fn mine_facility_test_1() {
  let mut mine = Mine::new();

  for i in 0..10 {
    mine.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  assert_eq!(mine.borrow_crew_hash()
                 .values()
                 .fold(0, |count, worker| {
                   assert_eq!(worker.lock().unwrap().get_loc(), Some(Loc::Mine) );
                   count + 1
                 }), 10);
}

#[test]
#[should_panic]
fn mine_facility_test_2() {
  let mut mine = Mine::new();

  for i in 1..11 {
    mine.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  let worker = mine.remove_unit(0).unwrap();
}

#[test]
fn mine_facility_test_3() {
  let mut mine = Mine::new();

  for i in 1..11 {
    mine.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  let worker = mine.remove_unit(2).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 2);

  let worker = mine.remove_unit(1).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 1);

  let worker = mine.remove_unit(8).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 8);

  let worker = mine.remove_unit(9).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 9);

  let worker = mine.remove_unit(5).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 5);

  let worker = mine.remove_unit(10).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 10);

  let mut nums_left = mine.borrow_crew_hash()
                          .keys()
                          .map(|&x| x)
                          .collect::<Vec<u8> >();
  nums_left.sort();
  assert_eq!(*nums_left, [3,4,6,7]);
}

#[test]
#[should_panic]
fn mine_facility_test_4() {
  let mut mine = Mine::new();

  mine.add_unit(Arc::new(Mutex::new(Worker::new(2))) );
  mine.remove_unit(2);
  mine.remove_unit(2).unwrap();
}

#[test]
fn mine_facility_test_5() {
  let mut mine = Mine::new();
  let multiplier = 10;

  for i in 0..10 {
    mine.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  for _ in 0..multiplier {
    mine.exp_up();
  }

  let mut test_unit = Worker::new(0);
  test_unit.set_loc(Loc::Mine);
  test_unit.add_exp(multiplier * EXP);
  let test_unit_lvl = test_unit.get_skill_lvl(Loc::Mine);

  for (_, worker) in mine.borrow_crew_hash() {
    assert_eq!(worker.lock().unwrap().get_skill_lvl(Loc::Mine), test_unit_lvl);
  }
}

#[test]
fn mine_producer_test_1() {

}
