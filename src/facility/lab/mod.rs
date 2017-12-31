use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use facility::{Facility, Loc, ResourceAccum};

use resources::crystals::{Color, CrystalBatch};

use worker::Worker;


type Ptr<T> = Arc<Mutex<T> >;

#[derive(Debug)]
pub struct Lab {
  crew: HashMap<u8, Ptr<Worker> >,
  crystals: CrystalBatch,
}

impl Lab {
  pub fn new() -> Self {
    Lab {
      crew: HashMap::new(),
      crystals: CrystalBatch::new_base(),
    }
  }

  pub fn add_crystals(&mut self, crystals: CrystalBatch) {
    self.crystals = self.crystals + crystals;
  }
/*
  pub fn set_study_color(&mut self, crystal: Color) {
    self.studying = Some(crystal);
  }

  pub fn study(&mut self) {
    let progress: f64 = 0.0;

    for (_,crew) in self.crew.iter() {
      let energy = crew.lock().unwrap().get_energy();
      let level = crew.lock().unwrap().get_skill_lvl(Loc::Lab);
      progress += 0.25 * energy * ((level as f64) / 10.0);
    }

    if let Some(color) = self.studying {
      let mut crystals_used = (progress * 1000.0) as u64;
      if self.crystals[color] < crystals_used {
        crystals_used = self.crystals[color];
        self.crystals[color] = 0;
      } else {
        self.crystals[color] -= crystals_used;
      }

      self.increase_progress(crystals_used);
    }
  }

  fn increase_progress(&mut self, crystals: u64) {
    match self.studying.expect("Cannot increase progress without studying something") {
      Color::Blue => {
        self.progress_list[0].use_crystals(crystals);
      },
      Color::Energy => {
        self.progress_list[1].use_crystals(crystals);
      },
      Color::Green => {
        self.progress_list[2].use_crystals(crystals);
      },
      Color::Purple => {
        self.progress_list[3].use_crystals(crystals);
      },
      Color::Red => {
        self.progress_list[4].use_crystals(crystals);
      },
      Color::Silver => {
        self.progress_list[5].use_crystals(crystals);
      },
      Color::Yellow => {
        self.progress_list[6].use_crystals(crystals);
      },
    }
  }
*/
}


impl Facility for Lab {
  fn borrow_crew_hash(&self) -> &HashMap<u8, Ptr<Worker> > {
    &self.crew
  }

  fn borrow_crew_hash_mut(&mut self) -> &mut HashMap<u8, Ptr<Worker> > {
    &mut self.crew
  }

  fn get_loc(&self) -> Loc {
    Loc::Lab
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn lab_test() {
      
  }
}
