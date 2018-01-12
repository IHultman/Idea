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
fn worker_energy_test() {
  let mut worker = Worker::new(0);

  assert_eq!(worker.get_energy(), 1.0);

  worker.add_energy(0.5);
  assert_eq!(worker.get_energy(), 1.0);

  worker.remove_energy(0.5);
  assert_eq!(worker.get_energy(), 0.5);

  worker.remove_energy(0.5);
  assert_eq!(worker.get_energy(), 0.1);
}

#[test]
fn worker_loc_test() {
  let mut worker = Worker::new(0);

  assert_eq!(worker.get_loc(), None);

  worker.set_loc(Loc::Academy);
  assert_eq!(worker.get_loc(), Some(Loc::Academy) );

  worker.set_loc(Loc::Farm);
  assert_eq!(worker.get_loc(), Some(Loc::Farm) );

  worker.set_loc(Loc::Lab);
  assert_eq!(worker.get_loc(), Some(Loc::Lab) );

  worker.set_loc(Loc::Mine);
  assert_eq!(worker.get_loc(), Some(Loc::Mine) );

  worker.set_loc(Loc::Ship);
  assert_eq!(worker.get_loc(), Some(Loc::Ship) );

  worker.set_loc(Loc::WaterProcessor);
  assert_eq!(worker.get_loc(), Some(Loc::WaterProcessor) );
}

#[test]
#[should_panic(expected = "Cannot add experience without location")]
fn worker_skills_test_1() {
  let mut worker = Worker::new(0);

  worker.add_exp(50);
}

#[test]
fn worker_skills_test_2() {
  let mut worker = Worker::new(0);

  assert_eq!(worker.get_skill_lvl(Loc::Academy), 1);
  assert_eq!(worker.get_skill_lvl(Loc::Farm), 1);
  assert_eq!(worker.get_skill_lvl(Loc::Lab), 1);
  assert_eq!(worker.get_skill_lvl(Loc::Mine), 1);
  assert_eq!(worker.get_skill_lvl(Loc::Ship), 1);
  assert_eq!(worker.get_skill_lvl(Loc::WaterProcessor), 1);

  worker.set_loc(Loc::Academy);
  worker.add_exp(50);

  worker.set_loc(Loc::Farm);
  worker.add_exp(2000);

  worker.set_loc(Loc::Lab);
  worker.add_exp(100);

  worker.set_loc(Loc::Mine);
  worker.add_exp(1500);

  worker.set_loc(Loc::Ship);
  worker.add_exp(350);

  worker.set_loc(Loc::WaterProcessor);
  worker.add_exp(500);

  assert_eq!(worker.get_skill_lvl(Loc::Academy), 2);
  assert_eq!(worker.get_skill_lvl(Loc::Farm), 10);
  assert_eq!(worker.get_skill_lvl(Loc::Lab), 3);
  assert_eq!(worker.get_skill_lvl(Loc::Mine), 9);
  assert_eq!(worker.get_skill_lvl(Loc::Ship), 5);
  assert_eq!(worker.get_skill_lvl(Loc::WaterProcessor), 6);
}
