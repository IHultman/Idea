use facility::Loc;

const LEVELS: [u32; 10] = [0, 50, 100, 200, 350, 500, 750, 1000, 1500, 2000];


#[derive(Debug, Clone, Copy, PartialEq)]
struct Skill {
  level: u8,
  exp: u32,
}

impl Skill {
  fn new() -> Skill {
    Skill {
      level: 1,
      exp: 0,
    }
  }

  fn get_lvl(&self) -> u8 {
    self.level
  }

  fn add_exp(&mut self, exp: u16) {
    if self.level < 10 {
      self.exp += exp as u32;
      while (self.level < 10) && (self.exp >= LEVELS[self.level as usize]) {
        self.level += 1;
      }
    }
  }
}

#[derive(Debug)]
pub struct Worker {
  energy: f64,
  id: u8,
  loc: Option<Loc>,
  skills: [Skill; 6],
}

impl Worker {
  pub fn new(id: u8) -> Worker {
    Worker {
      energy: 1.0,
      id: id,
      loc: None,
      skills: [Skill::new(); 6],
    }
  }

  pub fn get_energy(&self) -> f64 {
    self.energy
  }

  pub fn get_id(&self) -> u8 {
    self.id
  }

  pub fn get_loc(&self) -> Option<Loc> {
    self.loc
  }

  pub fn get_skill_lvl(&self, loc: Loc) -> u8 {
    match loc {
      Loc::Academy        => self.skills[0].get_lvl(),
      Loc::Farm           => self.skills[1].get_lvl(),
      Loc::Lab            => self.skills[2].get_lvl(),
      Loc::Mine           => self.skills[3].get_lvl(),
      Loc::Ship           => self.skills[4].get_lvl(),
      Loc::WaterProcessor => self.skills[5].get_lvl(),
    }
  }

  pub fn set_loc(&mut self, loc: Loc) {
    self.loc = Some(loc);
  }

  pub fn add_energy(&mut self, energy: f64) {
    let new_energy_level = self.energy + energy;
    self.energy = if new_energy_level > 1.0 {1.0} else {new_energy_level};
  }

  pub fn remove_energy(&mut self, energy: f64) {
    let new_energy_level = self.energy - energy;
    self.energy = if new_energy_level < 0.10 {0.10} else {new_energy_level};
  }

  pub fn add_exp(&mut self, exp: u16) {
    match self.loc.expect("Cannot add experience without location") {
      Loc::Academy        => self.skills[0].add_exp(exp),
      Loc::Farm           => self.skills[1].add_exp(exp),
      Loc::Lab            => self.skills[2].add_exp(exp),
      Loc::Mine           => self.skills[3].add_exp(exp),
      Loc::Ship           => self.skills[4].add_exp(exp),
      Loc::WaterProcessor => self.skills[5].add_exp(exp),
    }
  }
}

#[cfg(test)]
mod tests {
  use facility::Loc;
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
}
