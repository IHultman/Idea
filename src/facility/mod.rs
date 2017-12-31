use std::collections::HashMap;
use std::cmp::{Ord, PartialEq};
use std::fmt::Debug;
use std::ops::{Add, Mul, Sub};
use std::sync::{Arc, Mutex};
use std::thread;

use resources::ResourceAccum;

use worker::Worker;


pub mod academy;
pub mod farm;
pub mod lab;
pub mod mine;
pub mod ship;
pub mod waterproc;


const EXP: u16 = 25;

type Ptr<T> = Arc<Mutex<T> >;


#[derive(Clone, Copy, Debug)]
pub enum Loc {
  Academy,
  Farm,
  Lab,
  Mine,
  Ship,
  WaterProcessor,
}

impl PartialEq<Loc> for Loc {
  fn eq(&self, other: &Self) -> bool {
    match *self {
      Loc::Academy        => if let Loc::Academy = *other {true} else {false},
      Loc::Farm           => if let Loc::Farm = *other {true} else {false},
      Loc::Lab            => if let Loc::Lab = *other {true} else {false},
      Loc::Mine           => if let Loc::Mine = *other {true} else {false},
      Loc::Ship           => if let Loc::Ship = *other {true} else {false},
      Loc::WaterProcessor => if let Loc::WaterProcessor = *other {true} else {false},
    }
  }
}

pub trait Facility where
  Self: Send + 'static
{
  fn borrow_crew_hash(&self) -> &HashMap<u8, Ptr<Worker> >;
  fn borrow_crew_hash_mut(&mut self) -> &mut HashMap<u8, Ptr<Worker> >;
  fn get_loc(&self) -> Loc;

  fn borrow_crew_vec(&self) -> Vec<&Ptr<Worker> > {
    self.borrow_crew_hash().into_iter().map(|(_, unit)| unit).collect()
    //self.borrow_crew_hash().iter().map(|(_, unit)| unit).collect()
  }

  fn add_unit(&mut self, unit: Ptr<Worker>) {
    let loc = self.get_loc();
    let id = {
      let mut unit = unit.lock().unwrap();
      unit.set_loc(loc);
      unit.get_id()
    };
    self.borrow_crew_hash_mut().insert(id, unit);
    println!("Unit {} just arrived at {:?}!", id, loc);
  }

  fn remove_unit(&mut self, id: u8) {
    println!("Unit {} is leaving {:?}!", id, self.get_loc() );
    self.borrow_crew_hash_mut().remove(&id);
  }

  fn exp_up(&mut self) {
    let mut handles = self.borrow_crew_hash()
                          .iter()
                          .map(|(_, worker)| {
                            let worker = (*worker).clone();
                            thread::spawn(move || {worker.lock().unwrap().add_exp(EXP);})
                          })
                          .collect::<Vec<thread::JoinHandle<()>> >();

    for handle in handles {
      handle.join();
    }
  }
}

pub trait Producer where
  Self: Facility,
  Self::Resource: ResourceAccum + Debug,
{
  type Resource;

  fn harvest(&self) -> Self::Resource {
    self.borrow_crew_hash()
        .iter()
        .fold(
          Self::Resource::new_base(),
          |acc, (_,worker)| {
            acc + Self::Resource::produced((*worker).clone() )
          })
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn facility_test() {
      
  }
}
