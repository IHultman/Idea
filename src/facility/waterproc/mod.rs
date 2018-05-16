use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use facility::{Facility, Producer};
use facility::location::Loc;
use resources::water::Water;
use worker::Worker;


type Ptr<T> = Arc<Mutex<T> >;


const PRODUCT: f64 = 25.0;


#[derive(Debug)]
pub struct WaterProcessor {
  crew: HashMap<u8, Ptr<Worker> >,
}

impl WaterProcessor {
  pub fn new() -> Self {
    WaterProcessor {
      crew: HashMap::new(),
    }
  }
}

impl Facility for WaterProcessor {
  fn borrow_crew_hash(&self) -> &HashMap<u8, Ptr<Worker> > {
    &self.crew
  }

  fn borrow_crew_hash_mut(&mut self) -> &mut HashMap<u8, Ptr<Worker> > {
    &mut self.crew
  }

  fn get_loc(&self) -> Loc {
    Loc::WaterProcessor
  }
}

impl Producer for WaterProcessor {
  type ProduceArgs = ();
  type Resource = Water;

  fn get_produce_args(&self) {()}

  fn produce(worker: Ptr<Worker>, _: &() ) -> Water {
    let (lvl, energy) = {
      let worker = worker.lock().unwrap();
      (worker.get_skill_lvl(Loc::WaterProcessor), worker.get_energy() )
    };
    Water::new((PRODUCT * (lvl as f64) * energy ) as u64)
  }
}

#[cfg(test)]
mod tests;
