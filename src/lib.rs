extern crate rand;

use self::rand::Rng;

pub mod facility;
pub mod resources;
pub mod tech;
pub mod worker;
pub mod worksite;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
