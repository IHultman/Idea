use tech::crystalprop::*;
use super::*;

#[test]
fn technode_test_1() {
  let mut t1 = TechNode::new(Tech::T1, true);
  let mut t2 = TechNode::new(Tech::T2, false);

  t1.add_out_edge(Tech::T2).unwrap();
  t2.add_in_edge(Tech::T1).unwrap();
}

#[test]
#[should_panic]
fn technode_test_2() {
// fails with TechNodeErrs::AttemptToLinkToPrereq
  let mut t1 = TechNode::new(Tech::T1, true);

  t1.add_in_edge(Tech::T2).unwrap();
}

#[test]
#[should_panic]
fn technode_test_3() {
// fails with TechNodeErrs::AttemptToLinkToSelf
  let mut t1 = TechNode::new(Tech::T1, true);

  t1.add_out_edge(Tech::T1).unwrap();
}

#[test]
#[should_panic]
fn technode_test_4() {
// fails with TechNodeErrs::AttemptToLinkToSelf
  let mut t1 = TechNode::new(Tech::T1, false);

  t1.add_in_edge(Tech::T1).unwrap();
}

#[test]
#[should_panic]
fn technode_test_5() {
// fails with TechNodeErrs::AttemptToLinkToSelf
  let mut t1 = TechNode::new(Tech::T1, false);

  t1.add_in_edge(Tech::T1).unwrap();
}

#[test]
fn technode_test_6() {
// tests get_unacquired_in_edges() and get_acquired_in_edges()
  let mut t1 = TechNode::new(Tech::T1, false);

  t1.add_in_edge(Tech::T2).unwrap();
  t1.add_in_edge(Tech::T3).unwrap();
  t1.add_in_edge(Tech::T5).unwrap();

  assert_eq!(&**t1.get_unacquired_in_edges().unwrap(), &[Tech::T2, Tech::T3, Tech::T5]);
  assert_eq!(t1.get_acquired_in_edges(), None);
}

#[test]
fn technode_test_7() {
// tests get_unacquired_in_edges(), get_acquired_in_edges(), and move_link_acquired()
  let mut t1 = TechNode::new(Tech::T1, false);

  t1.add_in_edge(Tech::T2).unwrap();
  t1.add_in_edge(Tech::T3).unwrap();
  t1.add_in_edge(Tech::T5).unwrap();

  t1.move_link_acquired(Tech::T2);
  t1.move_link_acquired(Tech::T5);

  assert_eq!(&**t1.get_unacquired_in_edges().unwrap(), &[Tech::T3]);
  assert_eq!(&**t1.get_acquired_in_edges().unwrap(), &[Tech::T2, Tech::T5]);
}

#[test]
fn technode_test_7() {
// tests get_unacquired_in_edges(), get_acquired_in_edges(), and move_link_acquired()
  let mut t1 = TechNode::new(Tech::T1, false);

  t1.add_in_edge(Tech::T2).unwrap();
  t1.add_in_edge(Tech::T3).unwrap();
  t1.add_in_edge(Tech::T5).unwrap();

  t1.move_link_acquired(Tech::T2);
  t1.move_link_acquired(Tech::T5);

  assert_eq!(&**t1.get_unacquired_in_edges().unwrap(), &[Tech::T3]);
  assert_eq!(&**t1.get_acquired_in_edges().unwrap(), &[Tech::T2, Tech::T5]);
}
