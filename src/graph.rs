use std::hash::Hash;
use std::collections::HashMap;
use crate::Node;
use crate::graph::GraphError::{IdExists, IdDoesNotExist, EdgeAlreadyExists};
use std::ops::{Index, IndexMut};
use std::fmt::{Debug, Formatter, Result, Display};
pub use crate::graph::hash_graph::HashGraph;

mod hash_graph;
mod btree_graph;



pub trait Graph :
    Debug + Display {
    type ID;
    type Weight;
    type Value;

    fn new() -> Self;

    fn get(&self, id: &Self::ID) -> Option<&Self::Value> {
        match self.get_node(id) {
            None => None,
            Some(node) => Some(&node.value),
        }
    }

    fn get_mut(&mut self, id: &Self::ID) -> Option<&mut Self::Value> {
        match self.get_node_mut(id) {
            None => None,
            Some(node) => Some(&mut node.value),
        }
    }

    fn get_node(&self, id: &Self::ID) -> Option<&Node<Self::ID, Self::Value>>;
    fn get_node_mut(&mut self, id: &Self::ID) -> Option<&mut Node<Self::ID, Self::Value>>;

    fn add_node(&mut self, id: Self::ID, value: Self::Value) -> GraphResult;
    fn contains_node(&self, id: &Self::ID) -> bool;



    fn add_edge_with(&mut self, u: &Self::ID, v: &Self::ID, weight: Self::Weight) -> GraphResult;
    fn contains_edge(&self, u: &Self::ID, v: &Self::ID) -> bool;
    fn get_weight(&self, u: &Self::ID, v: &Self::ID) -> Option<&Self::Weight>;
    fn get_adjacent(&self, node: &Self::ID) -> Vec<&Self::ID>;

    fn nodes(&self) -> Vec<&Node<Self::ID, Self::Value>>;
    fn edges(&self) -> Vec<(& Self::ID, &Self::ID, &Self::Weight)>;


    fn num_nodes(&self) -> usize;
    fn num_edges(&self) -> usize;


}

pub trait GraphRef<'a, ID : 'a, W : 'a, T : 'a, G>
where G : 'a + Graph<ID=&'a ID,Weight=&'a W,Value=&'a T>{

    fn to_reference_graph(&'a self) -> G;
}








#[derive(Debug)]
pub enum GraphError {
    IdExists,
    IdDoesNotExist,
    EdgeAlreadyExists,
}

pub type GraphResult = std::result::Result<(), GraphError>;

