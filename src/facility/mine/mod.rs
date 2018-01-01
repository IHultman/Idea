use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use facility::{Facility, Loc, Producer};

use resources::crystals::CrystalBatch;

use worker::Worker;


type Ptr<T> = Arc<Mutex<T> >;


#[derive(Debug)]
pub struct Mine {
  crew: HashMap<u8, Ptr<Worker> >,
}

impl Mine {
  pub fn new() -> Self {
    Mine {
      crew: HashMap::new(),
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
  type Resource = CrystalBatch;
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use std::sync::{Arc, Mutex};
  use facility::{Facility, Loc, Producer};
  use resources::crystals::CrystalBatch;
  use super::*;
  use worker::*;
  //type Ptr<T> = Arc<Mutex<T> >;

  #[test]
  fn mine_test_1() {
    let mut mine = Mine::new();

    for i in 0..10 {
      mine.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
    }

    assert_eq!(mine.borrow_crew_hash()
                   .values()
                   .fold(0, |count, worker| {
                     assert_eq!(worker.lock().unwrap().get_loc(), Some(Loc::Mine) );
                     count + 1
                   }), 10);
  }

  #[test]
  fn mine_test_2() {

  }
}
