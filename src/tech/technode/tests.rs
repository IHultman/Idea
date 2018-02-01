use tech::crystalprop::*;
use super::*;

#[test]
fn technode_test_1() {
// testing adding out and in edges

  let mut tech_vec = Vec::with_capacity(5);
  tech_vec.push(TechNode::new(Tech::Antimatter) );
  tech_vec.push(TechNode::new(Tech::Earthquake) );
  tech_vec.push(TechNode::new(Tech::Flight) );
  tech_vec.push(TechNode::new(Tech::Teleport) );
  tech_vec.push(TechNode::new(Tech::Walls) );

  test_out(&mut tech_vec, 0, 1);
  test_in(&mut tech_vec, 1, 0);
}

#[test]
#[should_panic]
fn technode_test_2() {
// testing for failure when adding an edge to a node such that
// the edge already exists

  let mut tech_vec = Vec::with_capacity(5);
  tech_vec.push(TechNode::new(Tech::Antimatter) );
  tech_vec.push(TechNode::new(Tech::Earthquake) );
  tech_vec.push(TechNode::new(Tech::Flight) );
  tech_vec.push(TechNode::new(Tech::Teleport) );
  tech_vec.push(TechNode::new(Tech::Walls) );

  test_out(&mut tech_vec, 0, 1);
  test_out(&mut tech_vec, 0, 1);
}

#[test]
#[should_panic]
fn technode_test_3() {
// testing for failure when adding an edge from a node such that
// the edge already exists

  let mut tech_vec = Vec::with_capacity(5);
  tech_vec.push(TechNode::new(Tech::Antimatter) );
  tech_vec.push(TechNode::new(Tech::Earthquake) );
  tech_vec.push(TechNode::new(Tech::Flight) );
  tech_vec.push(TechNode::new(Tech::Teleport) );
  tech_vec.push(TechNode::new(Tech::Walls) );

  test_out(&mut tech_vec, 0, 1);
  test_in(&mut tech_vec, 1, 0);
  test_in(&mut tech_vec, 1, 0);
}

#[test]
#[should_panic]
fn technode_test_4() {
// testing for failure when trying to add an out_edge to a node
// such that an edge to a node with the same name already exists

  let mut tech_vec = Vec::with_capacity(5);
  tech_vec.push(TechNode::new(Tech::Antimatter) );
  tech_vec.push(TechNode::new(Tech::Earthquake) );
  tech_vec.push(TechNode::new(Tech::Flight) );
  tech_vec.push(TechNode::new(Tech::Teleport) );
  tech_vec.push(TechNode::new(Tech::Walls) );
  tech_vec.push(TechNode::new(Tech::Teleport) );

  test_out(&mut tech_vec, 1, 3);
  test_in(&mut tech_vec, 3, 1);
  test_out(&mut tech_vec, 1, 5);
}

#[test]
fn technode_test_5() {
// this test demonstrates that the reverse of the previous test isn't true
// thus care needs to be taken when adding nodes to a large structure

  let mut tech_vec = Vec::with_capacity(5);
  tech_vec.push(TechNode::new(Tech::Antimatter) );
  tech_vec.push(TechNode::new(Tech::Earthquake) );
  tech_vec.push(TechNode::new(Tech::Flight) );
  tech_vec.push(TechNode::new(Tech::Teleport) );
  tech_vec.push(TechNode::new(Tech::Walls) );
  tech_vec.push(TechNode::new(Tech::Teleport) );

  test_out(&mut tech_vec, 1, 3);
  test_in(&mut tech_vec, 3, 1);
  test_in(&mut tech_vec, 5, 1);
}

#[test]
fn technode_test_6() {
// testing for correctness of TechNode state when adding out and in edges
// and testing the move_link_acquired() function

  let mut tech_vec = Vec::with_capacity(5);
  tech_vec.push(TechNode::new(Tech::Antimatter) );
  tech_vec.push(TechNode::new(Tech::Earthquake) );
  tech_vec.push(TechNode::new(Tech::Flight) );
  tech_vec.push(TechNode::new(Tech::Teleport) );
  tech_vec.push(TechNode::new(Tech::Walls) );

  test_out(&mut tech_vec, 0, 2);
  test_out(&mut tech_vec, 0, 3);
  test_out(&mut tech_vec, 0, 4);
  test_in(&mut tech_vec, 2, 0);
  test_in(&mut tech_vec, 3, 0);
  test_in(&mut tech_vec, 4, 0);

  assert!(tech_vec[0].available);
  assert!(!tech_vec[2].available);
  assert!(!tech_vec[3].available);
  assert!(!tech_vec[4].available);

  assert!(tech_vec[2].unacquired_in_edges.is_some() );
  assert!(tech_vec[3].unacquired_in_edges.is_some() );
  assert!(tech_vec[4].unacquired_in_edges.is_some() );

  let addr = &tech_vec[0] as *const TechNode;
  unsafe {
    tech_vec[2].move_link_acquired(addr).unwrap();
    tech_vec[3].move_link_acquired(addr).unwrap();
    tech_vec[4].move_link_acquired(addr).unwrap();
  }

  assert!(tech_vec[2].unacquired_in_edges.is_none() );
  assert!(tech_vec[3].unacquired_in_edges.is_none() );
  assert!(tech_vec[4].unacquired_in_edges.is_none() );

  assert!(tech_vec[2].available);
  assert!(tech_vec[3].available);
  assert!(tech_vec[4].available);
}

#[test]
#[should_panic]
fn technode_test_7() {
// testing for failure of move_link_acquired()

  let mut tech_vec = Vec::with_capacity(5);
  tech_vec.push(TechNode::new(Tech::Antimatter) );
  tech_vec.push(TechNode::new(Tech::Earthquake) );
  tech_vec.push(TechNode::new(Tech::Flight) );
  tech_vec.push(TechNode::new(Tech::Teleport) );
  tech_vec.push(TechNode::new(Tech::Walls) );

  test_out(&mut tech_vec, 2, 4);
  test_in(&mut tech_vec, 4, 2);

  let addr = &tech_vec[0] as *const TechNode;
  unsafe {
    tech_vec[4].move_link_acquired(addr).unwrap();
  }
}

#[test]
#[should_panic]
fn technode_test_8() {
// testing for failure of move_link_acquired()

  let mut tech_vec = Vec::with_capacity(5);
  tech_vec.push(TechNode::new(Tech::Antimatter) );
  tech_vec.push(TechNode::new(Tech::Earthquake) );
  tech_vec.push(TechNode::new(Tech::Flight) );
  tech_vec.push(TechNode::new(Tech::Teleport) );
  tech_vec.push(TechNode::new(Tech::Walls) );

  test_out(&mut tech_vec, 1, 4);
  test_in(&mut tech_vec, 4, 1);

  let addr = &tech_vec[1] as *const TechNode;
  unsafe {
    tech_vec[4].move_link_acquired(addr).unwrap();
    tech_vec[4].move_link_acquired(addr).unwrap();
  }
}

#[test]
fn technode_test_9() {
// testing for correctness of TechNode state after calling mark_researched()

  let mut tech_vec = Vec::with_capacity(5);
  tech_vec.push(TechNode::new(Tech::Antimatter) );
  tech_vec.push(TechNode::new(Tech::Earthquake) );
  tech_vec.push(TechNode::new(Tech::Flight) );
  tech_vec.push(TechNode::new(Tech::Teleport) );
  tech_vec.push(TechNode::new(Tech::Walls) );

  test_out(&mut tech_vec, 0, 2);
  test_out(&mut tech_vec, 0, 3);
  test_out(&mut tech_vec, 0, 4);
  test_in(&mut tech_vec, 2, 0);
  test_in(&mut tech_vec, 3, 0);
  test_in(&mut tech_vec, 4, 0);

  for tech in &tech_vec {
    assert!(!tech.researched);
  }

  assert!(tech_vec[0].available);

  for tech in &tech_vec[2..5] {
    assert!(!tech.available);
  }

  tech_vec[0].mark_researched().unwrap();

  for tech in &tech_vec[2..5] {
    assert!(tech.available);
  }

  assert!(tech_vec[0].researched);
}

#[test]
#[should_panic]
fn technode_test_10() {
// testing for failure with respect to mark_researched() as a result of
// a link to an out_edge node having already moved to acquired in that node

  let mut tech_vec = Vec::with_capacity(5);
  tech_vec.push(TechNode::new(Tech::Antimatter) );
  tech_vec.push(TechNode::new(Tech::Earthquake) );
  tech_vec.push(TechNode::new(Tech::Flight) );
  tech_vec.push(TechNode::new(Tech::Teleport) );
  tech_vec.push(TechNode::new(Tech::Walls) );

  test_out(&mut tech_vec, 0, 2);
  test_out(&mut tech_vec, 0, 3);
  test_out(&mut tech_vec, 0, 4);
  test_in(&mut tech_vec, 2, 0);
  test_in(&mut tech_vec, 3, 0);
  test_in(&mut tech_vec, 4, 0);

  for tech in &tech_vec {
    assert!(!tech.researched);
  }

  assert!(tech_vec[0].available);
  for tech in &tech_vec[2..5] {
    assert!(!tech.available);
  }

  let addr = &tech_vec[0] as *const TechNode;
  unsafe {
    tech_vec[2].move_link_acquired(addr).unwrap();
  }

  // failure occurs here
  tech_vec[0].mark_researched().unwrap();
}

#[test]
#[should_panic]
fn technode_test_11() {
// testing for failure with respect to mark_researched() given that the
// node is not marked available

  let mut tech_vec = Vec::with_capacity(5);
  tech_vec.push(TechNode::new(Tech::Antimatter) );
  tech_vec.push(TechNode::new(Tech::Earthquake) );
  tech_vec.push(TechNode::new(Tech::Flight) );
  tech_vec.push(TechNode::new(Tech::Teleport) );
  tech_vec.push(TechNode::new(Tech::Walls) );

  test_out(&mut tech_vec, 2, 4);
  test_in(&mut tech_vec, 4, 2);

  tech_vec[4].mark_researched().unwrap();
}

#[test]
#[should_panic]
fn technode_test_12() {
// testing for failure with respect to mark_researched() as a result
// of a tech already having been researched previously

  let mut tech_vec = Vec::with_capacity(5);
  tech_vec.push(TechNode::new(Tech::Antimatter) );
  tech_vec.push(TechNode::new(Tech::Earthquake) );
  tech_vec.push(TechNode::new(Tech::Flight) );
  tech_vec.push(TechNode::new(Tech::Teleport) );
  tech_vec.push(TechNode::new(Tech::Walls) );

  test_out(&mut tech_vec, 2, 4);
  test_in(&mut tech_vec, 4, 2);

  tech_vec[2].mark_researched().unwrap();
  tech_vec[2].mark_researched().unwrap();
}

#[test]
#[should_panic]
fn technode_test_13() {
// testing for failure with respect to mark_researched() as a result
// of an out_edge node not having a link in from a node

  let mut tech_vec = Vec::with_capacity(5);
  tech_vec.push(TechNode::new(Tech::Antimatter) );
  tech_vec.push(TechNode::new(Tech::Earthquake) );
  tech_vec.push(TechNode::new(Tech::Flight) );
  tech_vec.push(TechNode::new(Tech::Teleport) );
  tech_vec.push(TechNode::new(Tech::Walls) );

  test_out(&mut tech_vec, 2, 4);

  tech_vec[2].mark_researched().unwrap();
}

#[test]
fn technode_test_14() {
// testing get_all_in_edges()

  let mut tech_vec = Vec::with_capacity(5);
  tech_vec.push(TechNode::new(Tech::Antimatter) );
  tech_vec.push(TechNode::new(Tech::Earthquake) );
  tech_vec.push(TechNode::new(Tech::Flight) );
  tech_vec.push(TechNode::new(Tech::Teleport) );
  tech_vec.push(TechNode::new(Tech::Walls) );

  test_out(&mut tech_vec, 0, 1);
  test_out(&mut tech_vec, 2, 1);
  test_out(&mut tech_vec, 3, 1);
  test_out(&mut tech_vec, 4, 1);
  test_in(&mut tech_vec, 1, 0);
  test_in(&mut tech_vec, 1, 2);
  test_in(&mut tech_vec, 1, 3);
  test_in(&mut tech_vec, 1, 4);

  tech_vec[0].mark_researched().unwrap();
  tech_vec[2].mark_researched().unwrap();

  let rv1 = tech_vec[1].get_all_in_edges().
    unwrap().
    into_iter().
    map(|p| {
      unsafe {(*p).get_tech_name()}
    }).
    collect::<Vec<Tech> >();

  let rv2 = tech_vec[1].get_unacquired_in_edges().
    unwrap().
    into_iter().
    map(|&p| {
      unsafe {(*p).get_tech_name()}
    }).
    collect::<Vec<Tech> >();

  let rv3 = tech_vec[1].get_acquired_in_edges().
    unwrap().
    into_iter().
    map(|&p| {
      unsafe {(*p).get_tech_name()}
    }).
    collect::<Vec<Tech> >();

  assert_eq!(*rv1, [Tech::Antimatter, Tech::Flight, Tech::Teleport, Tech::Walls]);
  assert_eq!(*rv2, [Tech::Teleport, Tech::Walls]);
  assert_eq!(*rv3, [Tech::Antimatter, Tech::Flight]);
}


fn test_out(vec: &mut Vec<TechNode>, node: usize, out_to: usize) {
  let addr = &mut vec[out_to] as *mut TechNode;
  vec[node].add_out_edge(addr).unwrap();
  assert!(vec[node].out_edges.is_some() );
  if let Some(ref o_edges) = vec[node].out_edges {
    let i = o_edges.len() - 1;
    assert_eq!(o_edges[i], addr);
  }
}

fn test_in(vec: &mut Vec<TechNode>, node: usize, in_from: usize) {
  let addr = &vec[in_from] as *const TechNode;
  vec[node].add_in_edge(addr).unwrap();
  assert!(vec[node].unacquired_in_edges.is_some() );
  if let Some(ref i_edges) = vec[node].unacquired_in_edges {
    let i = i_edges.len() - 1;
    assert_eq!(i_edges[i], addr);
  }
}
