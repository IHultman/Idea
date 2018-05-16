use facility::location::Loc;
use super::*;

#[test]
fn skill_level_test() {
  let mut skill = Skill::new();

  assert_eq!(skill.get_lvl(), 1);

  skill.add_exp(50);
  assert_eq!(skill.get_lvl(), 2);

  skill.add_exp(450);
  assert_eq!(skill.get_lvl(), 6);

  skill.add_exp(40000);
  assert_eq!(skill.get_lvl(), 10);
}

#[test]
fn worker_test_1() {
  let mut worker = Worker::new(0);

  assert_eq!(worker.get_id(), 0);
  assert_eq!(worker.get_energy(), 1.0);
  assert!(worker.get_loc().is_none() );
  assert_eq!(worker.get_skill_lvl(Loc::Farm), 1);

  worker.set_loc(Loc::Farm);

  assert_eq!(worker.get_loc(), Some(Loc::Farm) );

  worker.add_exp(100);

  assert_eq!(worker.get_skill_lvl(Loc::Farm), 3);

  worker.set_loc(Loc::Mine);

  assert_eq!(worker.get_loc(), Some(Loc::Mine) );

  worker.add_exp(20000);

  assert_eq!(worker.get_skill_lvl(Loc::Mine), 10);

  worker.add_energy(10.0);

  assert_eq!(worker.get_energy(), 1.0);

  worker.remove_energy(10.0);

  assert_eq!(worker.get_energy(), 0.1);

  worker.add_energy(0.4);

  assert_eq!(worker.get_energy(), 0.5);
}

#[test]
#[should_panic]
fn worker_test_2() {
  let mut worker = Worker::new(1);

  worker.add_exp(1);
}

#[test]
#[should_panic]
fn worker_test_3() {
  let mut worker = Worker::new(2);

  worker.set_loc(Loc::Lab);
  worker.add_energy(-1.0);
}

#[test]
#[should_panic]
fn worker_test_4() {
  let mut worker = Worker::new(3);

  worker.set_loc(Loc::Lab);
  worker.remove_energy(-1.0);
}

#[test]
fn worker_test_5() {
  let mut worker = Worker::new(57);

  worker.set_loc(Loc::WaterProcessor);
  worker.add_exp(65536);

  assert_eq!(worker.get_skill_lvl(Loc::WaterProcessor), 1);
}

#[test]
fn worker_test_6() {
  let mut worker = Worker::new(57);

  worker.set_loc(Loc::WaterProcessor);
  worker.add_exp(65535);

  assert_eq!(worker.get_skill_lvl(Loc::WaterProcessor), 10);
}
