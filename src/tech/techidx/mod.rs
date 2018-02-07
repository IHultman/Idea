use super::crystalprop::*;


pub struct TechIdx {
  idx: usize,
  tech: Tech
}

impl TechIdx {
  pub fn new(idx: usize, tech: Tech) -> TechIdx {
    TechIdx {
      idx: idx,
      tech: tech,
    }
  }

  pub fn get_idx(&self) -> usize {
    self.idx
  }

  pub fn get_tech(&self) -> Tech {
    self.tech
  }

  pub fn set_idx(&mut self, idx: usize) {
    self.idx = idx;
  }

  pub fn set_tech(&mut self, tech: Tech) {
    self.tech = tech;
  }
}
