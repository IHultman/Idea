use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use facility::Facility;
use facility::location::Loc;

use worker::Worker;


type Ptr<T> = Arc<Mutex<T> >;


#[derive(Debug)]
pub struct Ship {
  crew: HashMap<u8, Ptr<Worker> >,
}

impl Ship {
  pub fn new() -> Self {
    Ship {
      crew: HashMap::new(),
    }
  }
}

impl Facility for Ship {
  fn borrow_crew_hash(&self) -> &HashMap<u8, Ptr<Worker> > {
    &self.crew
  }

  fn borrow_crew_hash_mut(&mut self) -> &mut HashMap<u8, Ptr<Worker> > {
    &mut self.crew
  }

  fn get_loc(&self) -> Loc {
    Loc::Ship
  }
}

#[cfg(test)]
mod tests;
