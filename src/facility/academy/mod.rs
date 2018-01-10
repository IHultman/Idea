use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use facility::{Facility, Loc};

use worker::Worker;


type Ptr<T> = Arc<Mutex<T> >;


#[derive(Debug)]
pub struct Academy {
  crew: HashMap<u8, Ptr<Worker> >,
}

impl Academy {
  pub fn new() -> Self {
    Academy {
      crew: HashMap::new(),
    }
  }
}

impl Facility for Academy {
  fn borrow_crew_hash(&self) -> &HashMap<u8, Ptr<Worker> > {
    &self.crew
  }

  fn borrow_crew_hash_mut(&mut self) -> &mut HashMap<u8, Ptr<Worker> > {
    &mut self.crew
  }

  fn get_loc(&self) -> Loc {
    Loc::Academy
  }
}

#[cfg(test)]
mod tests;
