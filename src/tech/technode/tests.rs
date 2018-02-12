use tech::crystalprop::*;
use super::*;

#[test]
fn technode_test_1() {
// tests add_in_edge() and add_out_edge()
  let mut t1 = TechNode::new(Tech::T1, true);
  let mut t2 = TechNode::new(Tech::T2, false);

  assert!(t1.available);
  assert!(t2.available);

  t1.add_out_edge(Tech::T2).unwrap();
  t2.add_in_edge(Tech::T1).unwrap();

  assert!(t1.available);
  assert!(!t2.available);
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
fn technode_test_8() {
// tests get_unacquired_in_edges(), get_acquired_in_edges(), and move_link_acquired()
  let mut t1 = TechNode::new(Tech::T1, false);

  t1.add_in_edge(Tech::T3).unwrap();
  t1.add_in_edge(Tech::T5).unwrap();

  t1.move_link_acquired(Tech::T3);
  t1.move_link_acquired(Tech::T5);

  assert_eq!(t1.get_unacquired_in_edges(), None);
  assert_eq!(&**t1.get_acquired_in_edges().unwrap(), &[Tech::T3, Tech::T5]);
}

#[test]
fn technode_test_9() {
// tests get_all_in_edges()
  let mut t1 = TechNode::new(Tech::T1, false);

  assert_eq!(t1.get_all_in_edges(), None);
}

#[test]
fn technode_test_10() {
// tests get_all_in_edges() and move_link_acquired()
  let mut t1 = TechNode::new(Tech::T1, false);

  t1.add_in_edge(Tech::T3).unwrap();
  t1.add_in_edge(Tech::T5).unwrap();
  t1.add_in_edge(Tech::T6).unwrap();
  t1.add_in_edge(Tech::T8).unwrap();
  t1.add_in_edge(Tech::T9).unwrap();

  t1.move_link_acquired(Tech::T3);
  t1.move_link_acquired(Tech::T6);
  t1.move_link_acquired(Tech::T8);

  assert_eq!(&**t1.get_unacquired_in_edges().unwrap(), &[Tech::T5, Tech::T9]);
  assert_eq!(&**t1.get_acquired_in_edges().unwrap(), &[Tech::T3, Tech::T6, Tech::T8]);
  assert_eq!(*t1.get_all_in_edges().unwrap(), [Tech::T3, Tech::T5, Tech::T6, Tech::T8, Tech::T9]);
}

#[test]
#[should_panic]
fn technode_test_11() {
// fails with TechNodeErrs::TechNotFound
  let mut t1 = TechNode::new(Tech::T1, false);

  t1.add_in_edge(Tech::T2).unwrap();

  t1.move_link_acquired(Tech::T3).unwrap();
}

#[test]
fn technode_test_12() {
// tests move_link_acquired()
  let mut t1 = TechNode::new(Tech::T1, false);

  t1.add_in_edge(Tech::T2).unwrap();

  t1.move_link_acquired(Tech::T2).unwrap();

  assert_eq!(t1.get_unacquired_in_edges(), None);
}

#[test]
#[should_panic]
fn technode_test_13() {
// fails with TechNodeErrs::TechNotFound
  let mut t1 = TechNode::new(Tech::T1, false);

  t1.add_in_edge(Tech::T2).unwrap();

  t1.move_link_acquired(Tech::T2).unwrap();
  t1.move_link_acquired(Tech::T3).unwrap();
}

#[test]
#[should_panic]
fn technode_test_14() {
// fails with TechNodeErrs::TechAlreadyMarkedAcquired
  let mut t1 = TechNode::new(Tech::T1, false);

  t1.add_in_edge(Tech::T2).unwrap();

  t1.move_link_acquired(Tech::T2).unwrap();
  t1.move_link_acquired(Tech::T2).unwrap();
}

#[test]
#[should_panic]
fn technode_test_15() {
// fails with TechNodeErrs::TechNotFound
  let mut t1 = TechNode::new(Tech::T1, false);

  t1.add_in_edge(Tech::T2).unwrap();
  t1.add_in_edge(Tech::T3).unwrap();

  t1.move_link_acquired(Tech::T2).unwrap();
  t1.move_link_acquired(Tech::T2).unwrap();
}

#[test]
#[should_panic]
fn technode_test_16() {
// fails with TechNodeErrs::InEdgesTechAlreadyExists
  let mut t1 = TechNode::new(Tech::T1, false);

  t1.add_in_edge(Tech::T2).unwrap();
  t1.add_in_edge(Tech::T2).unwrap();
}

#[test]
#[should_panic]
fn technode_test_17() {
// fails with TechNodeErrs::InEdgesTechAlreadyExists
  let mut t1 = TechNode::new(Tech::T1, false);

  {
    let rmv = t1.acquired_in_edges.get_or_insert(Vec::new() );
    rmv.push(Tech::T2);
  }

  assert_eq!(t1.get_unacquired_in_edges(), None);
  assert_eq!(&**t1.get_acquired_in_edges().unwrap(), &[Tech::T2]);

  t1.add_in_edge(Tech::T2).unwrap();
}

#[test]
#[should_panic]
fn technode_test_18() {
// fails with TechNodeErrs::OutEdgesTechAlreadyExists
  let mut t1 = TechNode::new(Tech::T1, true);

  t1.add_out_edge(Tech::T2).unwrap();
  t1.add_out_edge(Tech::T2).unwrap();
}

#[test]
fn technode_test_19() {
// tests move_link_acquired()
  let mut t1 = TechNode::new(Tech::T1, false);

  {
    let rmv = t1.acquired_in_edges.get_or_insert(Vec::new() );
    rmv.push(Tech::T2);

    let rmv = t1.unacquired_in_edges.get_or_insert(Vec::new() );
    rmv.push(Tech::T2);
  }

  assert_eq!(&**t1.get_unacquired_in_edges().unwrap(), &[Tech::T2]);
  assert_eq!(&**t1.get_acquired_in_edges().unwrap(), &[Tech::T2]);

  // produces Err(TechNodeErrs::TechAlreadyMarkedAcquired)
  t1.move_link_acquired(Tech::T2);

  assert_eq!(t1.get_unacquired_in_edges(), None);
  assert_eq!(&**t1.get_acquired_in_edges().unwrap(), &[Tech::T2]);
}

#[test]
fn technode_test_20() {
// tests mark_researched()
  let mut t1 = TechNode::new(Tech::T1, true);

  assert!(t1.available);
  assert!(!t1.researched);

  t1.mark_researched().unwrap();

  assert!(t1.researched);
}

#[test]
#[should_panic]
fn technode_test_21() {
// fails with TechNodeErrs::AlreadyResearched
  let mut t1 = TechNode::new(Tech::T1, true);

  t1.mark_researched().unwrap();
  t1.mark_researched().unwrap();
}

#[test]
#[should_panic]
fn technode_test_22() {
// fails with TechNodeErrs::TechNotAvailable
  let mut t1 = TechNode::new(Tech::T1, false);

  t1.add_in_edge(Tech::T2).unwrap();
  t1.mark_researched().unwrap();
}

#[test]
fn technode_test_23() {
// tests mark_researched()
  let mut t1 = TechNode::new(Tech::T1, false);

  t1.add_in_edge(Tech::T2).unwrap();
  t1.move_link_acquired(Tech::T2).unwrap();
  t1.mark_researched().unwrap();
}

#[test]
fn technode_test_24() {
// tests mark_researched()
  let mut t1 = TechNode::new(Tech::T1, false);
  {
    let rmv = t1.unacquired_in_edges.get_or_insert(Vec::new() );
  }

  assert_eq!(&**t1.get_unacquired_in_edges().unwrap(), &[]);

  t1.mark_researched();

  assert_eq!(t1.get_unacquired_in_edges(), None);
}

#[test]
fn technode_test_25() {
// tests mark_researched()
  let mut t1 = TechNode::new(Tech::T1, false);
  {
    let rmv = t1.unacquired_in_edges.get_or_insert(Vec::new() );
    rmv.push(Tech::T2);
  }

  assert_eq!(&**t1.get_unacquired_in_edges().unwrap(), &[Tech::T2]);
  assert!(t1.available);

  t1.mark_researched();

  assert_eq!(&**t1.get_unacquired_in_edges().unwrap(), &[Tech::T2]);
  assert!(!t1.available);
}

#[test]
#[should_panic]
fn technode_test_26() {
// fails with TechNodeErrs::IllegallyMarkedAvailable
  let mut t1 = TechNode::new(Tech::T1, false);
  {
    let rmv = t1.unacquired_in_edges.get_or_insert(Vec::new() );
  }

  t1.mark_researched().unwrap();
}

#[test]
fn technode_test_27() {
// tests mark_researched()
  let mut t1 = TechNode::new(Tech::T1, false);
  {
    let rmv = t1.unacquired_in_edges.get_or_insert(Vec::new() );
  }

  t1.mark_researched();
  t1.mark_researched().unwrap();
}
