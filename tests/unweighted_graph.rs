use graph_rs::{HashGraph, Graph, GraphReverse, GraphRef};

#[test]
fn create_unweighted() {
    let mut graph: HashGraph = HashGraph::new();
    graph.add_nodes(0..10).unwrap();
    graph.add_edge(&0, &1).unwrap();

    let weight = graph[(0, 1)];
    assert_eq!(weight, ());

}

#[test]
fn reverse() {
    let mut graph : HashGraph<usize, i32, ()> = HashGraph::new();
    

    graph.add_nodes(0..2).unwrap();
    graph.add_edge_with(&0, &1, 15).unwrap();

    let graph_r = graph.into_reverse();

    assert_eq!(graph_r.num_nodes(), 2, "Number of nodes should stay the same");
    assert_eq!(graph_r.num_edges(), 1, "Number of edges should stay the same");

    assert!(graph_r.contains_edge(&1, &0), "This should now exist");
    assert!(!graph_r.contains_edge(&0, &1), "This should not exist");
    assert_eq!(graph_r.get_weight(&1, &0), Some(&15), "Weight should remain");


}