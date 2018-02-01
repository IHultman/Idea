use tech::crystalprop::*;
use super::*;
use super::technode::*;


#[test]
fn tech_digraph_test_1() {
//testing add_prereq()

  let mut digraph = TechDiGraph::new();

  assert!(digraph.prereqs.is_empty() );

  digraph.add_prereq(Tech::Flight).unwrap();

  assert!(digraph.advanced.is_empty() );

  let i = digraph.prereqs.len() - 1;
  assert_eq!(digraph.prereqs[i].get_tech_name(), Tech::Flight);
}

#[test]
#[should_panic]
fn tech_digraph_test_2() {
//testing add_prereq()

  let mut digraph = TechDiGraph::new();

  digraph.add_prereq(Tech::Flight).unwrap();
  digraph.add_prereq(Tech::Flight).unwrap();
}

#[test]
#[should_panic]
fn tech_digraph_test_3() {
//testing add_prereq()

  let mut digraph = TechDiGraph::new();

  digraph.advanced.push(TechNode::new(Tech::Flight) );
  digraph.add_prereq(Tech::Flight).unwrap();
}

#[test]
fn tech_digraph_test_4() {
//testing get_node_ref()

  let mut digraph = TechDiGraph::new();

  digraph.prereqs.push(TechNode::new(Tech::Antimatter) );
  TechDiGraph::get_node_ref(&digraph.prereqs, Tech::Antimatter).unwrap();

  digraph.advanced.push(TechNode::new(Tech::Flight) );
  TechDiGraph::get_node_ref(&digraph.advanced, Tech::Flight).unwrap();
}

#[test]
#[should_panic]
fn tech_digraph_test_5() {
//testing get_node_ref()

  let mut digraph = TechDiGraph::new();

  digraph.prereqs.push(TechNode::new(Tech::Antimatter) );
  TechDiGraph::get_node_ref(&digraph.prereqs, Tech::Flight).unwrap();
}

#[test]
#[should_panic]
fn tech_digraph_test_6() {
//testing get_node_ref()

  let mut digraph = TechDiGraph::new();

  digraph.advanced.push(TechNode::new(Tech::Flight) );
  TechDiGraph::get_node_ref(&digraph.advanced, Tech::Antimatter).unwrap();
}

#[test]
fn tech_digraph_test_7() {
//testing add_advanced_link()

  let mut digraph = TechDiGraph::new();

  digraph.add_prereq(Tech::Antimatter).unwrap();
  digraph.add_advanced_link(Tech::Antimatter, Tech::Earthquake).unwrap();

  assert_eq!(digraph.prereqs[0].get_tech_name(), Tech::Antimatter);
  assert_eq!(digraph.advanced[0].get_tech_name(), Tech::Earthquake);
}

#[test]
fn tech_digraph_test_8() {
//testing add_advanced_link()

  let mut digraph = TechDiGraph::new();

  digraph.add_prereq(Tech::Antimatter).unwrap();
  digraph.add_advanced_link(Tech::Antimatter, Tech::Earthquake).unwrap();
  digraph.add_advanced_link(Tech::Earthquake, Tech::EnergyShield).unwrap();

  assert_eq!(digraph.prereqs[0].get_tech_name(), Tech::Antimatter);
  assert_eq!(digraph.advanced[0].get_tech_name(), Tech::Earthquake);
  assert_eq!(digraph.advanced[1].get_tech_name(), Tech::EnergyShield);
}

#[test]
fn tech_digraph_test_9() {
//testing add_advanced_link()

  let mut digraph = TechDiGraph::new();

  digraph.add_prereq(Tech::Antimatter).unwrap();
  digraph.add_advanced_link(Tech::Antimatter, Tech::Earthquake).unwrap();
  digraph.add_advanced_link(Tech::Earthquake, Tech::EnergyShield).unwrap();
  digraph.add_advanced_link(Tech::Antimatter, Tech::EnergyShield).unwrap();

  assert_eq!(digraph.prereqs[0].get_tech_name(), Tech::Antimatter);
  assert_eq!(digraph.advanced[0].get_tech_name(), Tech::Earthquake);
  assert_eq!(digraph.advanced[1].get_tech_name(), Tech::EnergyShield);
}

#[test]
#[should_panic]
fn tech_digraph_test_10() {
//testing add_advanced_link()

  let mut digraph = TechDiGraph::new();

  digraph.add_prereq(Tech::Antimatter).unwrap();
  digraph.add_advanced_link(Tech::Antimatter, Tech::Earthquake).unwrap();
  digraph.add_advanced_link(Tech::Antimatter, Tech::Earthquake).unwrap();
}

#[test]
#[should_panic]
fn tech_digraph_test_11() {
//testing add_advanced_link()

  let mut digraph = TechDiGraph::new();

  digraph.add_prereq(Tech::Antimatter).unwrap();
  digraph.add_prereq(Tech::EnergyShield).unwrap();
  digraph.add_advanced_link(Tech::Antimatter, Tech::EnergyShield).unwrap();
}

#[test]
fn tech_digraph_test_12() {
//testing add_advanced_link()

  let mut digraph = TechDiGraph::new();

  digraph.add_prereq(Tech::Antimatter).unwrap();
  digraph.add_prereq(Tech::EnergyShield).unwrap();
  digraph.add_advanced_link(Tech::Antimatter, Tech::EnergyShield);

  assert_eq!(digraph.prereqs[0].get_tech_name(), Tech::Antimatter);
  assert!(digraph.prereqs[0].get_out_edges().is_none() );

  assert_eq!(digraph.prereqs[1].get_tech_name(), Tech::EnergyShield);
  assert!(digraph.prereqs[1].get_unacquired_in_edges().is_none() );
  assert!(digraph.prereqs[1].get_acquired_in_edges().is_none() );
}

#[test]
#[should_panic]
fn tech_digraph_test_13() {
//testing add_advanced_link()

  let mut digraph = TechDiGraph::new();

  digraph.add_prereq(Tech::Antimatter).unwrap();
  digraph.add_advanced_link(Tech::Antimatter, Tech::Flight).unwrap();
  digraph.add_advanced_link(Tech::Flight, Tech::Flight).unwrap();
}

#[test]
#[should_panic]
fn tech_digraph_test_14() {
//testing add_advanced_link()

  let mut digraph = TechDiGraph::new();

  digraph.add_prereq(Tech::Flight).unwrap();
  digraph.add_advanced_link(Tech::EnergyShield, Tech::Earthquake).unwrap();
}

#[test]
fn tech_digraph_test_15() {
//testing mark_studied()

  let mut digraph = TechDiGraph::new();

  digraph.add_prereq(Tech::Flight).unwrap();
  digraph.mark_studied(Tech::Flight).unwrap();
}

#[test]
fn tech_digraph_test_16() {
//testing mark_studied()

  let mut digraph = TechDiGraph::new();

  digraph.add_prereq(Tech::Flight).unwrap();
  digraph.add_advanced_link(Tech::Flight, Tech::EnergyShield);
  digraph.mark_studied(Tech::Flight).unwrap();
}

#[test]
#[should_panic]
fn tech_digraph_test_17() {
//testing mark_studied()

  let mut digraph = TechDiGraph::new();

  digraph.add_prereq(Tech::Flight).unwrap();
  digraph.add_advanced_link(Tech::Flight, Tech::EnergyShield);
  digraph.mark_studied(Tech::EnergyShield).unwrap();
}

#[test]
fn tech_digraph_test_18() {
//testing mark_studied()

  let mut digraph = TechDiGraph::new();

  digraph.add_prereq(Tech::Flight).unwrap();
  digraph.add_advanced_link(Tech::Flight, Tech::EnergyShield);
  digraph.mark_studied(Tech::Flight).unwrap();
  digraph.mark_studied(Tech::EnergyShield).unwrap();
}

#[test]
#[should_panic]
fn tech_digraph_test_19() {
//testing mark_studied()

  let mut digraph = TechDiGraph::new();

  digraph.add_prereq(Tech::Flight).unwrap();
  digraph.add_advanced_link(Tech::Flight, Tech::EnergyShield);
  digraph.mark_studied(Tech::Flight).unwrap();
  digraph.mark_studied(Tech::Flight).unwrap();
}
