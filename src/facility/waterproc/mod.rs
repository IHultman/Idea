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
  type Resource = Water;
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use std::sync::{Arc, Mutex};
  use facility::{Facility, Loc, Producer};
  use resources::water::Water;
  use super::*;
  use worker::*;
  //type Ptr<T> = Arc<Mutex<T> >;

  #[test]
  fn waterproc_test_1() {
    let mut waterproc = WaterProcessor::new();

    for i in 0..10 {
      waterproc.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
    }

    assert_eq!(waterproc.borrow_crew_hash()
                        .values()
                        .fold(0, |count, worker| {
                          assert_eq!(worker.lock().unwrap().get_loc(), Some(Loc::WaterProcessor) );
                          count + 1
                        }), 10);
  }

  #[test]
  fn waterproc_test_2() {
      
  }
}
