use std::cmp::PartialEq;


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
