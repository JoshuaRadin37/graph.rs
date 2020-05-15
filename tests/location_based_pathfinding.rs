use graph_rs::{HashGraph, GraphTools};
use graph_rs::directed::Directed;
use graph_rs::pathing::spatial::Point;

#[test]
fn small_test() {

    let mut map = HashGraph::<usize, f64, Point<f64>>::new();

    let point_ids = map.add_nodes_auto_id(
        vec![
            Point::newi(3, 5),
            Point::newi(0, 0),
            Point::newi(5, 8)
        ]
    ).unwrap();

    assert_eq!(point_ids, vec![0, 1, 2]);


}