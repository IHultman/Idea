use resources::crystals::Color;


pub enum CrystalPropErrs {
  AllPropertiesSet,
  IllegalColorChoice,
}


pub struct CrystalProperties {
  properties: [[bool; 10]; 7]
}

impl CrystalProperties {
  pub fn new() -> Self {
    CrystalProperties {
      properties: [[false; 10]; 7],
    }
  }

  pub fn set_new_property_rand(&mut self, color: Color) -> Result<usize, CrystalPropErrs> {
    if color == Color::Energy {
      return Err(CrystalPropErrs::IllegalColorChoice);
    }

    let prop_list = self.get_prop_list_mut(color);
    let i_vec = (*prop_list).iter().
      enumerate().
      filter(|&(_, &p)| !p).
      map(|(i, _)| i).
      collect::<Vec<usize> >();
    let i_remaining = i_vec.len();

    if i_remaining == 0 {
      Err(CrystalPropErrs::AllPropertiesSet)
    } else {
      let i_to_set = i_vec[::rand::random::<usize>() % i_remaining];
      prop_list[i_to_set] = true;
      Ok(i_to_set)
    }
  }

  fn get_prop_list_ref(&self, color: Color) -> &[bool; 10] {
    match color {
      Color::Blue   => &self.properties[0],
      Color::Energy => &self.properties[1],
      Color::Green  => &self.properties[2],
      Color::Purple => &self.properties[3],
      Color::Red    => &self.properties[4],
      Color::Silver => &self.properties[5],
      Color::Yellow => &self.properties[6],
    }
  }

  fn get_prop_list_mut(&mut self, color: Color) -> &mut [bool; 10] {
    match color {
      Color::Blue   => &mut self.properties[0],
      Color::Energy => &mut self.properties[1],
      Color::Green  => &mut self.properties[2],
      Color::Purple => &mut self.properties[3],
      Color::Red    => &mut self.properties[4],
      Color::Silver => &mut self.properties[5],
      Color::Yellow => &mut self.properties[6],
    }
  }
}


#[cfg(test)]
mod tests;
