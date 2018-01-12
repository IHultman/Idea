use facility::location::Loc;

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
mod tests;
