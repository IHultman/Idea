extern crate colored;
extern crate rand;


use colored::*;
use rand::Rng;


pub mod facility;
pub mod resources;
pub mod worker;
pub mod worksite;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
