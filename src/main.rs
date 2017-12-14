extern crate game;

use game::facility::*;
use game::facility::mine::resource::*;
use game::worker::Worker;
use game::worksite::WorkSite;


fn main() {
  let mut worksite = WorkSite::new();
  worksite.move_unit(1, Loc::Farm);
  worksite.move_unit(2, Loc::Farm);
  worksite.move_unit(3, Loc::Farm);
  worksite.move_unit(4, Loc::Farm);
  worksite.move_unit(5, Loc::Farm);
  worksite.move_unit(16, Loc::Farm);
  worksite.move_unit(17, Loc::Farm);
  worksite.move_unit(18, Loc::Farm);
  worksite.move_unit(19, Loc::Farm);
  worksite.move_unit(20, Loc::Farm);
  worksite.move_unit(6, Loc::WaterProcessor);
  worksite.move_unit(7, Loc::WaterProcessor);
  worksite.move_unit(8, Loc::WaterProcessor);
  worksite.move_unit(9, Loc::WaterProcessor);
  worksite.move_unit(10, Loc::WaterProcessor);
  worksite.move_unit(21, Loc::WaterProcessor);
  worksite.move_unit(22, Loc::WaterProcessor);
  worksite.move_unit(23, Loc::WaterProcessor);
  worksite.move_unit(24, Loc::WaterProcessor);
  worksite.move_unit(25, Loc::WaterProcessor);
  worksite.move_unit(11, Loc::Mine);
  worksite.move_unit(12, Loc::Mine);
  worksite.move_unit(13, Loc::Mine);
  worksite.move_unit(14, Loc::Mine);
  worksite.move_unit(15, Loc::Mine);
  worksite.move_unit(26, Loc::Mine);
  worksite.move_unit(27, Loc::Mine);
  worksite.move_unit(28, Loc::Mine);
  worksite.move_unit(29, Loc::Mine);
  worksite.move_unit(30, Loc::Mine);
  worksite.accumulate_resources();
  worksite.accumulate_resources();
  worksite.accumulate_resources();
  worksite.accumulate_resources();
}
