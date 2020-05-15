use std::hash::Hash;
use std::collections::HashMap;
use crate::Node;
use crate::graph::GraphError::{IdExists, IdDoesNotExist, EdgeAlreadyExists};
use std::ops::{Index, IndexMut, Add};
use std::fmt::{Debug, Formatter, Result, Display};
pub use crate::graph::hash_graph::HashGraph;
use num_traits::{PrimInt, One, Zero, FromPrimitive};

mod hash_graph;
mod btree_graph;



pub trait Graph :
Debug + Display {
    type ID : Eq;
    type Weight;
    type Value;



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

    fn add_node_with(&mut self, id: Self::ID, value: Self::Value) -> GraphResult;


    fn contains_node(&self, id: &Self::ID) -> bool;



    fn add_edge_with(&mut self, u: &Self::ID, v: &Self::ID, weight: Self::Weight) -> GraphResult;
    fn contains_edge(&self, u: &Self::ID, v: &Self::ID) -> bool;
    fn get_weight(&self, u: &Self::ID, v: &Self::ID) -> Option<&Self::Weight>;
    fn get_adjacent(&self, node: &Self::ID) -> Vec<&Self::ID>;

    fn nodes(&self) -> Vec<&Node<Self::ID, Self::Value>>;
    fn edges(&self) -> Vec<(& Self::ID, &Self::ID, &Self::Weight)>;


    fn num_nodes(&self) -> usize;
    fn num_edges(&self) -> usize;

    fn take_nodes(self) -> Vec<Node<Self::ID, Self::Value>>;

}

pub trait GraphRef<'a, ID : 'a, W : 'a, T : 'a, G>
    where G : 'a + Graph<ID=&'a ID,Weight=&'a W,Value=&'a T>,
          ID: Eq {

    fn to_reference_graph(&'a self) -> G;
}

pub trait GraphReverse<ID, W, T, G = Self>
    where G : Graph<ID=ID, Weight=W, Value=T> ,
          ID: Eq {

    fn into_reverse(self) -> G;

}

pub trait GraphTools<ID> : Graph<ID=ID> where
    ID : PrimInt + Zero + One + Add + Eq + FromPrimitive
{
    fn add_node_auto_id(&mut self, value: Self::Value) -> GraphResult<ID> {
        if let Some(mut next_id) = ID::from_usize(self.num_nodes()) {
            while self.get_node(&next_id).is_some() {
                next_id = next_id + ID::one();
            }

            self.add_node_with(next_id.clone(), value)?;
            Ok(next_id)
        } else {
            panic!("Can't create anymore nodes")
        }
    }

    fn add_nodes_auto_id(&mut self, values: Vec<Self::Value>) -> GraphResult<Vec<ID>> {
        if let Some(mut next_id) = ID::from_usize(self.num_nodes()) {
            let mut output = Vec::with_capacity(values.len());
            for val in values {
                while self.get_node(&next_id).is_some() {
                    next_id = next_id + ID::one();
                }

                self.add_node_with(next_id.clone(), val)?;
                output.push(next_id.clone());
                next_id = next_id + ID::one();
            }

            Ok(output)
        } else {
            panic!("Can't create anymore nodes")
        }
    }


}

impl <ID, G> GraphTools<ID> for G
    where G : Graph<ID=ID>,
          ID : PrimInt + Zero + One + Add + Eq + FromPrimitive {

}





#[derive(Debug)]
pub enum GraphError {
    IdExists,
    IdDoesNotExist,
    EdgeAlreadyExists,
}

pub type GraphResult<T=()> = std::result::Result<T, GraphError>;

