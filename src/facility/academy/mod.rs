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
mod tests {
  use std::collections::HashMap;
  use std::sync::{Arc, Mutex};
  use facility::{Facility, Loc};
  use super::*;
  use worker::*;
  //type Ptr<T> = Arc<Mutex<T> >;

  #[test]
  fn academy_test_1() {
    let mut academy = Academy::new();

    for i in 0..10 {
      academy.add_unit(Arc::new(Mutex::new(Worker::new(i))) );
    }

    assert_eq!(academy.borrow_crew_hash()
                      .values()
                      .fold(0, |count, worker| {
                        assert_eq!(worker.lock().unwrap().get_loc(), Some(Loc::Academy) );
                        count + 1
                      }), 10);
  }

  #[test]
  fn academy_test_2() {

  }
}
