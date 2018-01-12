use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use facility::{Facility, EXP};
use facility::location::Loc;
use super::*;
use worker::*;

type Ptr<T> = Arc<Mutex<T> >;


#[test]
fn ship_test_1() {
  let mut ship = Ship::new();

  for i in 0..10 {
    ship.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  assert_eq!(ship.borrow_crew_hash()
                 .values()
                 .fold(0, |count, worker| {
                   assert_eq!(worker.lock().unwrap().get_loc(), Some(Loc::Ship) );
                   count + 1
                 }), 10);
}

#[test]
#[should_panic]
fn ship_facility_test_2() {
  let mut ship = Ship::new();

  for i in 5..15 {
    ship.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  let worker = ship.remove_unit(15).unwrap();
}

#[test]
fn ship_facility_test_3() {
  let mut ship = Ship::new();

  for i in 5..15 {
    ship.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  let worker = ship.remove_unit(6).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 6);

  let worker = ship.remove_unit(5).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 5);

  let worker = ship.remove_unit(13).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 13);

  let worker = ship.remove_unit(14).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 14);

  let worker = ship.remove_unit(8).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 8);

  let worker = ship.remove_unit(10).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 10);

  let worker = ship.remove_unit(12).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 12);

  let mut nums_left = ship.borrow_crew_hash()
                          .keys()
                          .map(|&x| x)
                          .collect::<Vec<u8> >();
  nums_left.sort();
  assert_eq!(*nums_left, [7,9,11]);
}

#[test]
#[should_panic]
fn ship_facility_test_4() {
  let mut ship = Ship::new();

  ship.add_unit(Arc::new(Mutex::new(Worker::new(9))) );
  ship.remove_unit(9);
  ship.remove_unit(9).unwrap();
}

#[test]
fn ship_facility_test_5() {
  let mut ship = Ship::new();
  let multiplier = 15;
  let test_loc = Loc::Ship;

  for i in 0..10 {
    ship.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  for _ in 0..multiplier {
    ship.exp_up();
  }

  let mut test_unit = Worker::new(0);
  test_unit.set_loc(test_loc);
  test_unit.add_exp(multiplier * EXP);
  let test_unit_lvl = test_unit.get_skill_lvl(test_loc);

  for (_, worker) in ship.borrow_crew_hash() {
    assert_eq!(worker.lock().unwrap().get_skill_lvl(test_loc), test_unit_lvl);
  }
}
