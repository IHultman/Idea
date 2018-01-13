use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use facility::{Facility, Producer, EXP};
use facility::location::Loc;
use resources::water::*;
use super::*;
use worker::*;

type Ptr<T> = Arc<Mutex<T> >;


#[test]
fn waterproc_facility_test_1() {
  let mut waterproc = WaterProcessor::new();

  for i in 0..10 {
    waterproc.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  assert_eq!(waterproc.borrow_crew_hash()
                      .values()
                      .fold(0, |count, worker| {
                        assert_eq!(worker.lock().unwrap().get_loc(), Some(Loc::WaterProcessor) );
                        count + 1
                      }), 10);
}

#[test]
#[should_panic]
fn waterproc_facility_test_2() {
  let mut waterproc = WaterProcessor::new();

  for i in 0..10 {
    waterproc.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  let worker = waterproc.remove_unit(15).unwrap();
}

#[test]
fn waterproc_facility_test_3() {
  let mut waterproc = WaterProcessor::new();

  for i in 0..10 {
    waterproc.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  for i in 0..10 {
    let worker = waterproc.remove_unit(i).unwrap();
    assert_eq!(worker.lock().unwrap().get_id(), i);
  }

  let mut nums_left = waterproc.borrow_crew_hash()
                               .keys()
                               .map(|&x| x)
                               .collect::<Vec<u8> >();
  nums_left.sort();
  assert_eq!(*nums_left, []);
}

#[test]
#[should_panic]
fn waterproc_facility_test_4() {
  let mut waterproc = WaterProcessor::new();

  waterproc.add_unit(Arc::new(Mutex::new(Worker::new(0))) );
  waterproc.remove_unit(0);
  waterproc.remove_unit(0).unwrap();
}

#[test]
fn waterproc_facility_test_5() {
  let mut waterproc = WaterProcessor::new();
  let multiplier = 1000;
  let test_loc = Loc::WaterProcessor;

  for i in 0..10 {
    waterproc.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  for _ in 0..multiplier {
    waterproc.exp_up();
  }

  let mut test_unit = Worker::new(0);
  test_unit.set_loc(test_loc);
  test_unit.add_exp(multiplier * EXP);
  let test_unit_lvl = test_unit.get_skill_lvl(test_loc);

  for (_, worker) in waterproc.borrow_crew_hash() {
    assert_eq!(worker.lock().unwrap().get_skill_lvl(test_loc), test_unit_lvl);
  }
}

#[test]
fn waterproc_producer_test() {
  let mut waterproc = WaterProcessor::new();

  for i in 0..10 {
    waterproc.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  assert_eq!(waterproc.harvest(), Water::new((PRODUCT as u64) * 10) );

  for _ in 0..2 {
    waterproc.exp_up();
  }

  assert_eq!(waterproc.harvest(), Water::new((PRODUCT as u64) * 20) );

  waterproc.borrow_crew_hash()
      .into_iter()
      .map(|(_,worker)|
        worker.lock().unwrap().remove_energy(1.0)
      )
      .collect::<Vec<()> >();

  assert_eq!(waterproc.harvest(), Water::new((PRODUCT as u64) * 2) );

  for i in 10..15 {
    waterproc.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
  }

  assert_eq!(waterproc.harvest(), Water::new((PRODUCT as u64) * 7) );

  for i in 0..10 {
    waterproc.remove_unit(i);
  }

  assert_eq!(waterproc.harvest(), Water::new((PRODUCT as u64) * 5) );
}
