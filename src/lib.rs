use std::collections::{HashMap, VecDeque, HashSet};
use std::fmt::{Debug, Formatter, Result, Display};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, Index, IndexMut};


use std::iter::FromIterator;


mod node;
mod graph;

pub use crate::node::Node;
pub use crate::graph::*;

pub mod directed;
pub mod pathing;

///
/// The base structure of the HashGraph



/// Represents a HashGraph
///
/// `ID` must be impl `Hash`, `Eq`, and `Copy`


#[cfg(test)]
mod test {
    use crate::graph::{HashGraph, Graph};
    use crate::Node;

    #[test]
    fn is_key_works() {
        let n: Node = Node::new(1, ());

        assert!(n.is_id(&1))
    }

    #[test]
    fn create_HashGraph() {
        let mut g: HashGraph<u32> = HashGraph::new();
        assert_eq!(g.num_edges(), 0);
        assert_eq!(g.num_nodes(), 0);

        g.add_node_with(0, ()).unwrap();
        assert_eq!(g.num_nodes(), 1);
    }

    #[test]
    fn add_range_of_ids() {
        let mut g: HashGraph = HashGraph::new();

        g.add_nodes_with(0..10, ()).unwrap();
        assert_eq!(g.num_nodes(), 10);
    }

    #[test]
    fn set_weight() {
        let mut g: HashGraph<_, f64> = HashGraph::new();

        g.add_nodes_with(0..10, ()).unwrap();
        assert!(!g.contains_edge(&1, &2));
        assert!(g.add_edge_with(&1, &2, 10.0).is_ok());
        assert!(g.contains_edge(&1, &2));
        assert_eq!(g.get_weight(&1, &2).unwrap(), &10.0);
        assert!(g.get_weight(&4, &5).is_none());
        assert_eq!(g[(1, 2)], 10.0)
    }

    #[test]
    fn change_value() {
        let mut g: HashGraph<i32, f64, i32> = HashGraph::new();
        g.add_nodes_with(0..10, 10).unwrap();
        assert_eq!(g[3], 10);
        g[3] = 15;
        assert_eq!(g[3], 15);
    }

    #[test]
    fn get_adjacent() {
        let mut g: HashGraph = HashGraph::new();

        g.add_nodes_with(0..10, ()).unwrap();
        g.add_edge(&0, &1).unwrap();
        g.add_edge(&0, &3).unwrap();
        g.add_edge(&0, &7).unwrap();
        let mut v = g.get_adjacent(&0);
        v.sort();

        assert_eq!(v, vec![&1, &3, &7]);
    }

    #[derive(Clone, Copy)]
    struct Wrapper<T>(T);

    #[test]
    fn cloned_HashGraphs_independent() {
        let mut g: HashGraph<usize, f64, Wrapper<usize>> = HashGraph::new();

        g.add_nodes_with(0..10, Wrapper(5)).unwrap();
        g.add_edge_with(&3, &5, 10.0).unwrap();
        let mut g_prime = g.clone();
        g.add_edge_with(&5, &7, 11.0).unwrap();
        assert_eq!(g_prime.get_weight(&3, &5), g.get_weight(&3, &5));
    }

    /*
    #[test]
    fn test_sub_HashGraphs() {
        let mut g: HashGraph<usize, (), ()> = HashGraph::new();
        g.add_nodes(0..6).unwrap();
        g.add_edge(&0, &1).unwrap();
        g.add_edge(&1, &2).unwrap();
        g.add_edge(&2, &3).unwrap();
        g.add_edge(&4,&5).unwrap();

        let sub_HashGraphs = g.get_sub_HashGraphs();
        assert_eq!(sub_HashGraphs.len(), 3);
    }

     */
}
