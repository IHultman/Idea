use self::crystalprop::{CrystalProperties, CrystalProperty, Tech};
use self::technode::*;


pub mod crystalprop;
mod technode;


pub struct TechDiGraph {
  tech_list: Vec<TechNode>,
}


#[cfg(test)]
mod tests;
