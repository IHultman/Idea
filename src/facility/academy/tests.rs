use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use facility::{Facility, EXP};
use facility::location::Loc;
use super::*;
use worker::*;

type Ptr<T> = Arc<Mutex<T> >;


#[test]
fn academy_facility_test_1() {
  let mut academy = Academy::new();

  for i in 0..10 {
    academy.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  assert_eq!(academy.borrow_crew_hash()
                    .values()
                    .fold(0, |count, worker| {
                      assert_eq!(worker.lock().unwrap().get_loc(), Some(Loc::Academy) );
                      count + 1
                    }), 10);
}

#[test]
#[should_panic]
fn academy_facility_test_2() {
  let mut academy = Academy::new();

  for i in 0..10 {
    academy.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  let worker = academy.remove_unit(15).unwrap();
}

#[test]
fn academy_facility_test_3() {
  let mut academy = Academy::new();

  for i in 0..10 {
    academy.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  let worker = academy.remove_unit(2).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 2);

  let worker = academy.remove_unit(7).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 7);

  let worker = academy.remove_unit(9).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 9);

  let worker = academy.remove_unit(4).unwrap();
  assert_eq!(worker.lock().unwrap().get_id(), 4);

  let mut nums_left = academy.borrow_crew_hash()
                             .keys()
                             .map(|&x| x)
                             .collect::<Vec<u8> >();
  nums_left.sort();
  assert_eq!(*nums_left, [0,1,3,5,6,8]);
}

#[test]
#[should_panic]
fn academy_facility_test_4() {
  let mut academy = Academy::new();

  academy.add_unit(Arc::new(Mutex::new(Worker::new(1))) );
  academy.remove_unit(1);
  academy.remove_unit(1).unwrap();
}

#[test]
fn academy_facility_test_5() {
  let mut academy = Academy::new();
  let multiplier = 5;

  for i in 0..10 {
    academy.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  for _ in 0..multiplier {
    academy.exp_up();
  }

  let mut test_unit = Worker::new(0);
  test_unit.set_loc(Loc::Academy);
  test_unit.add_exp(multiplier * EXP);
  let test_unit_lvl = test_unit.get_skill_lvl(Loc::Academy);

  for (_, worker) in academy.borrow_crew_hash() {
    assert_eq!(worker.lock().unwrap().get_skill_lvl(Loc::Academy), test_unit_lvl);
  }
}
