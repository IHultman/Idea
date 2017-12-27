use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use facility::{Facility, Loc, Producer};
use facility::academy::Academy;
use facility::farm::Farm;
use facility::lab::Lab;
use facility::mine::Mine;
use facility::ship::Ship;
use facility::waterproc::WaterProcessor;

use resources::{ResourceAccum, ResourceUpkeep};
use resources::crystals::CrystalBatch;
use resources::food::Food;
use resources::water::Water;

use worker::Worker;

type Ptr<T> = Arc<Mutex<T> >;


#[derive(Debug)]
pub struct WorkSite {
  crew: Vec<Ptr<Worker> >,
  academy: Ptr<Academy>,
  farm: Ptr<Farm>,
  lab: Ptr<Lab>,
  mine: Ptr<Mine>,
  ship: Ptr<Ship>,
  water_proc: Ptr<WaterProcessor>,
  crystals: CrystalBatch,
  food: Food,
  water: Water,
}

impl WorkSite {
  pub fn new() -> WorkSite {
    WorkSite {
      crew: (0..50).map(|id| Arc::new(Mutex::new(Worker::new(id))) ).collect(),
      academy: Arc::new(Mutex::new(Academy::new()) ),
      farm: Arc::new(Mutex::new(Farm::new()) ),
      lab: Arc::new(Mutex::new(Lab::new()) ),
      mine: Arc::new(Mutex::new(Mine::new()) ),
      ship: Arc::new(Mutex::new(Ship::new()) ),
      water_proc: Arc::new(Mutex::new(WaterProcessor::new()) ),
      crystals: CrystalBatch::new_base(),
      food: Food::new_base(),
      water: Water::new_base(),
    }
  }

  pub fn end_turn(&mut self) {
    self.accumulate_resources();
    self.exp_up();
    WorkSite::pay_upkeep(&mut self.food, &self.crew);
    WorkSite::pay_upkeep(&mut self.water, &self.crew);
  }

  pub fn move_unit(&mut self, id: u8, to_loc: Loc) {
    self.remove_unit(id);
    self.add_unit(id, to_loc);
  }

  #[inline(always)]
  fn remove_unit(&mut self, id: u8) {
    let loc = self.crew[id as usize].lock().unwrap().get_loc();
    match loc {
      Some(loc) => {
        match loc {
          Loc::Academy        => self.academy.lock().unwrap().remove_unit(id),
          Loc::Farm           => self.farm.lock().unwrap().remove_unit(id),
          Loc::Lab            => self.lab.lock().unwrap().remove_unit(id),
          Loc::Mine           => self.mine.lock().unwrap().remove_unit(id),
          Loc::Ship           => self.ship.lock().unwrap().remove_unit(id),
          Loc::WaterProcessor => self.water_proc.lock().unwrap().remove_unit(id),
        }
      },
      None => {},
    }
  }

  #[inline(always)]
  fn add_unit(&mut self, id: u8, loc: Loc) {
    let crew_member = self.crew[id as usize].clone();
    match loc {
      Loc::Academy        => self.academy.lock().unwrap().add_unit(crew_member),
      Loc::Farm           => self.farm.lock().unwrap().add_unit(crew_member),
      Loc::Lab            => self.lab.lock().unwrap().add_unit(crew_member),
      Loc::Mine           => self.mine.lock().unwrap().add_unit(crew_member),
      Loc::Ship           => self.ship.lock().unwrap().add_unit(crew_member),
      Loc::WaterProcessor => self.water_proc.lock().unwrap().add_unit(crew_member),
    }
  }

  fn exp_up(&mut self) {
    let mut handles = Vec::new();

    handles.push(Self::spawn_thd_for_exp(self.academy.clone(), self.farm.clone()) );
    handles.push(Self::spawn_thd_for_exp(self.lab.clone(), self.mine.clone()) );
    handles.push(Self::spawn_thd_for_exp(self.ship.clone(), self.water_proc.clone()) );

    for handle in handles {
      handle.join().unwrap();
    }
  }

  #[inline(always)]
  fn spawn_thd_for_exp<T, S>(facility1: Ptr<T>, facility2: Ptr<S>) -> thread::JoinHandle<()>
    where
      T: Facility + Send + 'static,
      S: Facility + Send + 'static,
  {
    thread::spawn(move || {
      facility1.lock().unwrap().exp_up();
      facility2.lock().unwrap().exp_up();
    })
  }

  pub fn accumulate_resources(&mut self) {
    let food_handle = Self::spawn_thd_for_res(self.farm.clone() );
    let crystal_handle = Self::spawn_thd_for_res(self.mine.clone() );
    let water_handle = Self::spawn_thd_for_res(self.water_proc.clone() );

    let food = food_handle.join().unwrap();
    let crystal = crystal_handle.join().unwrap();
    let water = water_handle.join().unwrap();

    self.food = self.food + food;
    self.crystals = self.crystals + crystal;
    self.water = self.water + water;

    println!("{:?}", self.food);
    println!("{:?}", self.crystals);
    println!("{:?}", self.water);
  }

  #[inline(always)]
  fn spawn_thd_for_res<T>(facility: Ptr<T>) -> thread::JoinHandle<T::Resource>
    where
      T: Producer + Send + 'static,
      T::Resource: Send + 'static,
  {
    thread::spawn(move || {
      let resource = facility.lock().unwrap().harvest();
      resource
    })
  }

  fn pay_upkeep<R>(resource: &mut R, crew: &[Ptr<Worker>])
    where
      R: ResourceUpkeep
  {
    let pos_energy: f64 = 0.025;
    let neg_energy_coef: f64 = 0.005;
    let tot_upkeep: R = R::upkeep_per_unit() * (crew.len() as u64);
    // let tot_upkeep = <R as ResourceUpkeep>::upkeep_per_unit();

    if (*resource) > tot_upkeep {
      *resource = (*resource) - tot_upkeep;
      for worker in crew.iter() {
        worker.lock().unwrap().add_energy(pos_energy);
      }
    } else {
      let shortage: R = tot_upkeep - (*resource);
      let shortage_penalty: f64 = (shortage.get_value() as f64) / (crew.len() as f64);
      *resource = R::new_base();
      for worker in crew.iter() {
        worker.lock().unwrap().remove_energy(shortage_penalty * neg_energy_coef);
      }
    }
  }
}
