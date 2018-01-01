use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use facility::{Facility, Loc, Producer};

use resources::food::Food;

use worker::Worker;


type Ptr<T> = Arc<Mutex<T> >;


#[derive(Debug)]
pub struct Farm {
  crew: HashMap<u8, Ptr<Worker> >,
}

impl Farm {
  pub fn new() -> Self {
    Farm {
      crew: HashMap::new(),
    }
  }
}

impl Facility for Farm {
  fn borrow_crew_hash(&self) -> &HashMap<u8, Ptr<Worker> > {
    &self.crew
  }

  fn borrow_crew_hash_mut(&mut self) -> &mut HashMap<u8, Ptr<Worker> > {
    &mut self.crew
  }

  fn get_loc(&self) -> Loc {
    Loc::Farm
  }
}

impl Producer for Farm {
  type Resource = Food;
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use std::sync::{Arc, Mutex};
  use facility::{Facility, Loc, Producer};
  use resources::food::Food;
  use super::*;
  use worker::*;
  //type Ptr<T> = Arc<Mutex<T> >;

  #[test]
  fn farm_test_1() {
    let mut farm = Farm::new();

    for i in 0..10 {
      farm.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
    }

    assert_eq!(farm.borrow_crew_hash()
                   .values()
                   .fold(0, |count, worker| {
                     assert_eq!(worker.lock().unwrap().get_loc(), Some(Loc::Farm) );
                     count + 1
                   }), 10);
  }

  #[test]
  fn farm_test_2() {

  }
}
