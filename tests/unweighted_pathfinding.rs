use graph_rs::directed::{Undirected, new_hashed_undirected};
use graph_rs::HashGraph;

#[test]
fn small_path() {

    let mut undirected = Undirected::from(HashGraph::<_, ()>::new());

    undirected.add_nodes(0..10).unwrap();




}