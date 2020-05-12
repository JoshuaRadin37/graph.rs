use graph_rs::{HashGraph, Graph};

#[test]
fn create_unweighted() {
    let mut graph: HashGraph = HashGraph::new();
    graph.add_nodes(0..10).unwrap();
    graph.add_edge(&0, &1).unwrap();



}