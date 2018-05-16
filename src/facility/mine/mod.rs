use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use facility::{Facility, Producer};
use facility::location::Loc;
use resources::crystals::{Color, CrystalBatch};
use worker::Worker;


type Ptr<T> = Arc<Mutex<T> >;


const PRODUCT: f64 = 25.0;


#[derive(Debug)]
pub struct Mine {
  crew: HashMap<u8, Ptr<Worker> >,
  loc_color: Color,
}

impl Mine {
  pub fn new() -> Self {
    Mine {
      crew: HashMap::new(),
      loc_color: Color::Blue,
    }
  }
}

impl Facility for Mine {
  fn borrow_crew_hash(&self) -> &HashMap<u8, Ptr<Worker> > {
    &self.crew
  }

  fn borrow_crew_hash_mut(&mut self) -> &mut HashMap<u8, Ptr<Worker> > {
    &mut self.crew
  }

  fn get_loc(&self) -> Loc {
    Loc::Mine
  }
}

impl Producer for Mine {
  type ProduceArgs = Color;
  type Resource = CrystalBatch;

  fn get_produce_args(&self) -> Color {
    self.loc_color
  }

  fn produce(worker: Ptr<Worker>, color: &Color) -> CrystalBatch {
    let (lvl, energy) = {
      let worker = worker.lock().unwrap();
      (worker.get_skill_lvl(Loc::Mine), worker.get_energy() )
    };
    CrystalBatch::new_random(((PRODUCT * (lvl as f64) * energy) as u64), *color)
  }
}

#[cfg(test)]
mod tests;
