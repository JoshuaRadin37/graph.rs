use graph_rs::directed::{Undirected, new_hashed_undirected};
use graph_rs::HashGraph;
use graph_rs::pathing::{StatelessPathFinder, PathFinder};

#[test]
fn small_path() {

    let mut undirected = Undirected::from(HashGraph::<usize, ()>::new());

    undirected.add_nodes(0..10).unwrap();
    undirected.add_edge(&0, &1);
    undirected.add_edge(&0, &2);
    undirected.add_edge(&2, &3);
    undirected.add_edge(&1, &4);
    undirected.add_edge(&3, &4);

    let pathfinder = StatelessPathFinder::new(undirected);

    let path: Option<(Vec<_>, usize)> = pathfinder.find_path(&0, &4);
    if let Some((path, weight)) = path {
        assert_eq!(path, vec![0, 1, 4]);
        assert_eq!(weight, 2);
    } else {
        panic!("There is a path between the two points")
    }

}