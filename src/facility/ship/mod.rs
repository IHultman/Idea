use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use facility::{Facility, Loc};

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
mod tests {
  use std::collections::HashMap;
  use std::sync::{Arc, Mutex};
  use facility::{Facility, Loc};
  use super::*;
  use worker::*;
  //type Ptr<T> = Arc<Mutex<T> >;

  #[test]
  fn ship_test_1() {
    let mut ship = Ship::new();

    for i in 0..10 {
      ship.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
    }

    assert_eq!(ship.borrow_crew_hash()
                   .values()
                   .fold(0, |count, worker| {
                     assert_eq!(worker.lock().unwrap().get_loc(), Some(Loc::Ship) );
                     count + 1
                   }), 10);
  }

  #[test]
  fn ship_test_2() {

  }
}
