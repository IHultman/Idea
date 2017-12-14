use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use facility::{Facility, Loc, ResourceAccum};

use resources::crystals::{Color, CrystalBatch};

use worker::Worker;


type Ptr<T> = Arc<Mutex<T> >;

const TECH_STAGE_COST: [u64; 10] = [0, 200, 450, 900, 2000, 4000, 9000, 20000, 50000, 100000];


pub struct CrystalProperties {
  properties: [Option<PropertyLevel>; 12]
}

#[derive(Debug, Clone, Copy)]
pub struct tech_progress {
  level: u8,
  crystals_used: u64,
}

impl tech_progress {
  pub fn new() -> Self {
    tech_progress {
      level: 1,
      crystals_used: 0,
    }
  }

  pub fn get_lvl(&self) -> u8 {
    self.level
  }

  pub fn use_crystals(&mut self, crystals: u64) {
    if self.level < 10 {
      self.crystals_used += crystals;
      if self.crystals_used > TECH_STAGE_COST[self.level as usize] {
        self.level += 1;
      }
    }
  }
}

#[derive(Debug)]
pub struct Lab {
  crew: HashMap<u8, Ptr<Worker> >,
  crystals: CrystalBatch,
  progress_list: [tech_progress; 7],
  searching_property: Option<Color>,
  studying_tech: Option<>,
}

impl Lab {
  pub fn new() -> Self {
    Lab {
      crew: HashMap::new(),
      crystals: CrystalBatch::new_base(),
      progress_list: [tech_progress::new(); 7],
      searching_property: None,
      studying_tech: None,
    }
  }

  pub fn add_crystals(&mut self, crystals: CrystalBatch) {
    self.crystals = self.crystals + crystals;
  }

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
