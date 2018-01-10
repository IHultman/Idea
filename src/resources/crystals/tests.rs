use std::ops::{Add, Index, IndexMut};
use std::sync::{Arc, Mutex};
use facility::Loc;
use resources::ResourceAccum;
use super::*;
use worker::*;

type Ptr<T> = Arc<Mutex<T> >;


#[test]
fn crystals_add_test() {
  let c1 = CrystalBatch {
    blue: 8,
    energy: 97,
    green: 44,
    purple: 10,
    red: 0,
    silver: 70,
    yellow: 400,
  };

  let c2 = CrystalBatch {
    blue: 92,
    energy: 103,
    green: 256,
    purple: 390,
    red: 500,
    silver: 530,
    yellow: 300,
  };

  let expected_c = CrystalBatch {
    blue: 100,
    energy: 200,
    green: 300,
    purple: 400,
    red: 500,
    silver: 600,
    yellow: 700,
};

  assert_eq!(c1 + c2, expected_c);
}
