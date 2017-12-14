use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use facility::{Facility, Loc, Producer};

use resources::water::Water;

use worker::Worker;


type Ptr<T> = Arc<Mutex<T> >;


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
  type Resource = resource::Water;
}
