use super::*;
use super::technode::*;

#[test]
fn techdigraph_test_1() {
// tests add_prereq()
  let mut techdigraph = TechDiGraph::new();

  techdigraph.add_prereq(Tech::T1).unwrap();
  techdigraph.add_prereq(Tech::T2).unwrap();

  for t in &techdigraph.tech_list[0..8] {
    assert!(t.is_none() );
  }

  assert!(techdigraph.get_node_ref(Tech::T1).is_some() );
  assert!(techdigraph.get_node_ref(Tech::T2).is_some() );

  for t in &techdigraph.tech_list[10..18] {
    assert!(t.is_none() );
  }
}

#[test]
#[should_panic]
fn techdigraph_test_2() {
// fails with TechDiGraphErrs::PrereqAlreadyInsertedToGraph
  let mut techdigraph = TechDiGraph::new();

  techdigraph.add_prereq(Tech::T1).unwrap();
  techdigraph.add_prereq(Tech::T1).unwrap();
}

#[test]
fn techdigraph_test_3() {
// tests add_advanced_link()
  let mut techdigraph = TechDiGraph::new();

  techdigraph.add_prereq(Tech::T1).unwrap();
  techdigraph.add_advanced_link(Tech::T1, Tech::T2).unwrap();

  assert_eq!(
    &**techdigraph.get_node_ref(Tech::T1).as_ref().
    unwrap().
    get_out_edges().
    unwrap(), &[Tech::T2]);

  assert_eq!(
    &**techdigraph.get_node_ref(Tech::T2).as_ref().
    unwrap().
    get_unacquired_in_edges().
    unwrap(), &[Tech::T1]);
}

#[test]
#[should_panic]
fn techdigraph_test_4() {
// fails with TechDiGraphErrs::IllegalLink
  let mut techdigraph = TechDiGraph::new();

  techdigraph.add_prereq(Tech::T1).unwrap();
  techdigraph.add_prereq(Tech::T2).unwrap();
  techdigraph.add_advanced_link(Tech::T1, Tech::T2).unwrap();
}

#[test]
#[should_panic]
fn techdigraph_test_5() {
// fails with TechDiGraphErrs::TechNotFound
  let mut techdigraph = TechDiGraph::new();

  techdigraph.add_prereq(Tech::T1).unwrap();
  techdigraph.add_advanced_link(Tech::T2, Tech::T3).unwrap()
}

#[test]
fn techdigraph_test_6() {
//
  let mut techdigraph = TechDiGraph::new();

  techdigraph.add_prereq(Tech::T1).unwrap();
  techdigraph.add_advanced_link(Tech::T1, Tech::T2).unwrap();
  techdigraph.add_advanced_link(Tech::T2, Tech::T3).unwrap();

  for t in &techdigraph.tech_list[0..8] {
    assert!(t.is_none() );
  }

  assert_eq!(
    &**techdigraph.get_node_ref(Tech::T1).as_ref().
    unwrap().
    get_out_edges().
    unwrap(), &[Tech::T2]);

  assert_eq!(
    &**techdigraph.get_node_ref(Tech::T2).as_ref().
    unwrap().
    get_unacquired_in_edges().
    unwrap(), &[Tech::T1]);

  assert_eq!(
    &**techdigraph.get_node_ref(Tech::T3).as_ref().
    unwrap().
    get_unacquired_in_edges().
    unwrap(), &[Tech::T2]);

  for t in &techdigraph.tech_list[11..18] {
    assert!(t.is_none() );
  }
}

#[test]
#[should_panic]
fn techdigraph_test_7() {
// fails with TechDiGraphErrs::IllegalLink
  let mut techdigraph = TechDiGraph::new();

  techdigraph.add_prereq(Tech::T1).unwrap();
  techdigraph.add_advanced_link(Tech::T1, Tech::T1).unwrap();
}

#[test]
#[should_panic]
fn techdigraph_test_8() {
// fails with TechDiGraphErrs::LinkToTechAlreadyInsertedToGraph
  let mut techdigraph = TechDiGraph::new();

  techdigraph.add_prereq(Tech::T1).unwrap();
  techdigraph.add_advanced_link(Tech::T1, Tech::T2).unwrap();
  techdigraph.add_advanced_link(Tech::T1, Tech::T2).unwrap();
}

#[test]
#[should_panic]
fn techdigraph_test_9() {
// fails with TechDiGraphErrs::LinkToTechAlreadyInsertedToGraph
  let mut techdigraph = TechDiGraph::new();

  techdigraph.add_prereq(Tech::T1).unwrap();
  techdigraph.add_advanced_link(Tech::T1, Tech::T2).unwrap();
  techdigraph.add_advanced_link(Tech::T2, Tech::T3).unwrap();
  techdigraph.add_advanced_link(Tech::T2, Tech::T3).unwrap();
}

#[test]
fn techdigraph_test_10() {
// tests add_advanced_link()
  let mut techdigraph = TechDiGraph::new();

  techdigraph.add_prereq(Tech::T1).unwrap();
  techdigraph.add_prereq(Tech::T2).unwrap();
  techdigraph.add_prereq(Tech::T3).unwrap();

  techdigraph.add_advanced_link(Tech::T1, Tech::T4).unwrap();
  techdigraph.add_advanced_link(Tech::T2, Tech::T4).unwrap();
  techdigraph.add_advanced_link(Tech::T3, Tech::T4).unwrap();

  for t in &techdigraph.tech_list[8..11] {
    assert_eq!(
      &**t.as_ref().
      unwrap().
      get_out_edges().
      unwrap(), &[Tech::T4]);
  }

  assert_eq!(
    &**techdigraph.tech_list[11].as_ref().
    unwrap().
    get_unacquired_in_edges().
    unwrap(), &[Tech::T1, Tech::T2, Tech::T3]);
}

#[test]
fn techdigraph_test_11() {
// tests mark_researched()
  let mut techdigraph = TechDiGraph::new();

  assert!(techdigraph.get_node_ref(Tech::T1).is_none() );

  techdigraph.add_prereq(Tech::T1).unwrap();
  assert!(!techdigraph.get_node_ref(Tech::T1).as_ref().unwrap().is_researched() );

  techdigraph.mark_researched(Tech::T1).unwrap();
  assert!(techdigraph.get_node_ref(Tech::T1).as_ref().unwrap().is_researched() );
}

#[test]
#[should_panic]
fn techdigraph_test_12() {
// fails with TechDiGraphErrs::TechNotFound
  let mut techdigraph = TechDiGraph::new();

  techdigraph.mark_researched(Tech::T1).unwrap();
}

#[test]
fn techdigraph_test_13() {
// tests mark_researched()
  let mut techdigraph = TechDiGraph::new();

  techdigraph.add_prereq(Tech::T1).unwrap();
  techdigraph.add_advanced_link(Tech::T1, Tech::T2).unwrap();

  assert!(!techdigraph.get_node_ref(Tech::T1).as_ref().unwrap().is_researched() );
  assert!(!techdigraph.get_node_ref(Tech::T2).as_ref().unwrap().is_researched() );

  assert_eq!(
    &**techdigraph.get_node_ref(Tech::T1).as_ref().
    unwrap().
    get_out_edges().
    unwrap(), &[Tech::T2]);
  assert_eq!(
    &**techdigraph.get_node_ref(Tech::T2).as_ref().
    unwrap().
    get_unacquired_in_edges().
    unwrap(), &[Tech::T1]);

  techdigraph.mark_researched(Tech::T1).unwrap();

  assert!(techdigraph.get_node_ref(Tech::T1).as_ref().unwrap().is_researched() );
  assert!(!techdigraph.get_node_ref(Tech::T2).as_ref().unwrap().is_researched() );

  assert_eq!(
    &**techdigraph.get_node_ref(Tech::T1).as_ref().
    unwrap().
    get_out_edges().
    unwrap(), &[Tech::T2]);
  assert_eq!(
    &**techdigraph.get_node_ref(Tech::T2).as_ref().
    unwrap().
    get_acquired_in_edges().
    unwrap(), &[Tech::T1]);

  assert!(techdigraph.get_node_ref(Tech::T2).as_ref().unwrap().get_unacquired_in_edges().is_none() );
}

#[test]
#[should_panic]
fn techdigraph_test_14() {
// fails with TechDiGraphErrs::TechAlreadyResearched
  let mut techdigraph = TechDiGraph::new();

  techdigraph.add_prereq(Tech::T1).unwrap();

  techdigraph.mark_researched(Tech::T1).unwrap();
  techdigraph.mark_researched(Tech::T1).unwrap();
}

#[test]
#[should_panic]
fn techdigraph_test_15() {
// fails with TechDiGraphErrs::MultipleErrs([TechAlreadyResearched, LinkToTechAlreadyAcquired])
  let mut techdigraph = TechDiGraph::new();

  techdigraph.add_prereq(Tech::T1).unwrap();
  techdigraph.add_advanced_link(Tech::T1, Tech::T2).unwrap();

  techdigraph.mark_researched(Tech::T1).unwrap();
  techdigraph.mark_researched(Tech::T1).unwrap();
}

#[test]
fn techdigraph_test_16() {
// tests mark_researched()
  let mut techdigraph = TechDiGraph::new();

  techdigraph.add_prereq(Tech::T1).unwrap();
  techdigraph.add_advanced_link(Tech::T1, Tech::T2).unwrap();

  assert_eq!(
    &**techdigraph.get_node_ref(Tech::T1).as_ref().
    unwrap().
    get_out_edges().
    unwrap(), &[Tech::T2]);
  assert_eq!(
    &**techdigraph.get_node_ref(Tech::T2).as_ref().
    unwrap().
    get_unacquired_in_edges().
    unwrap(), &[Tech::T1]);

  techdigraph.mark_researched(Tech::T1).unwrap();

  assert_eq!(
    &**techdigraph.get_node_ref(Tech::T1).as_ref().
    unwrap().
    get_out_edges().
    unwrap(), &[Tech::T2]);
  assert_eq!(
    &**techdigraph.get_node_ref(Tech::T2).as_ref().
    unwrap().
    get_acquired_in_edges().
    unwrap(), &[Tech::T1]);

  techdigraph.add_advanced_link(Tech::T1, Tech::T3).unwrap();

  assert_eq!(
    &**techdigraph.get_node_ref(Tech::T1).as_ref().
    unwrap().
    get_out_edges().
    unwrap(), &[Tech::T2, Tech::T3]);
  assert_eq!(
    &**techdigraph.get_node_ref(Tech::T3).as_ref().
    unwrap().
    get_unacquired_in_edges().
    unwrap(), &[Tech::T1]);

  // returns Err(TechDiGraphErrs::MultipleErrs([TechAlreadyResearched, LinkToTechAlreadyAcquired]) )
  // but also moves T1 to acquired_in_edges of T3
  techdigraph.mark_researched(Tech::T1);

  assert_eq!(
    &**techdigraph.get_node_ref(Tech::T1).as_ref().
    unwrap().
    get_out_edges().
    unwrap(), &[Tech::T2, Tech::T3]);
  assert_eq!(
    &**techdigraph.get_node_ref(Tech::T3).as_ref().
    unwrap().
    get_acquired_in_edges().
    unwrap(), &[Tech::T1]);

  assert!(techdigraph.get_node_ref(Tech::T2).as_ref().unwrap().get_unacquired_in_edges().is_none() );
  assert!(techdigraph.get_node_ref(Tech::T3).as_ref().unwrap().get_unacquired_in_edges().is_none() );
}

#[test]
#[should_panic]
fn techdigraph_test_17() {
// fails with TechDiGraphErrs::TechNotAvailable
  let mut techdigraph = TechDiGraph::new();

  techdigraph.add_prereq(Tech::T1).unwrap();
  techdigraph.add_advanced_link(Tech::T1, Tech::T2).unwrap();

  techdigraph.mark_researched(Tech::T2).unwrap();
}

#[test]
fn techdigraph_test_18() {
// tests mark_researched()
  let mut techdigraph = TechDiGraph::new();

  techdigraph.add_prereq(Tech::T1).unwrap();
  techdigraph.add_advanced_link(Tech::T1, Tech::T2).unwrap();

  techdigraph.mark_researched(Tech::T1).unwrap();
  techdigraph.mark_researched(Tech::T2).unwrap();
}
