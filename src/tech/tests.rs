use tech::crystalprop::*;
use super::*;
use super::technode::*;


#[test]
fn tech_digraph_test_1() {
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
  let mut digraph = TechDiGraph::new();

  digraph.add_prereq(Tech::Flight).unwrap();
  digraph.add_prereq(Tech::Flight).unwrap();
}

#[test]
#[should_panic]
fn tech_digraph_test_3() {
  let mut digraph = TechDiGraph::new();

  digraph.advanced.push(TechNode::new(Tech::Flight) );
  digraph.add_prereq(Tech::Flight).unwrap();
}

#[test]
fn tech_digraph_test_4() {
  let mut digraph = TechDiGraph::new();

  digraph.prereqs.push(TechNode::new(Tech::Antimatter) );
  TechDiGraph::get_node_ref(&digraph.prereqs, Tech::Antimatter).unwrap();

  digraph.advanced.push(TechNode::new(Tech::Flight) );
  TechDiGraph::get_node_ref(&digraph.advanced, Tech::Flight).unwrap();
}

#[test]
#[should_panic]
fn tech_digraph_test_5() {
  let mut digraph = TechDiGraph::new();

  digraph.prereqs.push(TechNode::new(Tech::Antimatter) );
  TechDiGraph::get_node_ref(&digraph.prereqs, Tech::Flight).unwrap();
}

#[test]
#[should_panic]
fn tech_digraph_test_6() {
  let mut digraph = TechDiGraph::new();

  digraph.advanced.push(TechNode::new(Tech::Flight) );
  TechDiGraph::get_node_ref(&digraph.advanced, Tech::Antimatter).unwrap();
}

#[test]
fn tech_digraph_test_7() {
  let mut digraph = TechDiGraph::new();

  
}
