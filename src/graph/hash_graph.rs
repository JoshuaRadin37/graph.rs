use std::hash::Hash;
use std::collections::HashMap;
use crate::{Node, GraphRef};
use crate::graph::{Graph, GraphResult};
use crate::graph::GraphError::{IdExists, IdDoesNotExist, EdgeAlreadyExists};
use std::ops::{Index, IndexMut};
use std::fmt::{Debug, Formatter, Display};
use std::iter::FromIterator;


pub struct HashGraph<ID = usize, W = (), T = ()>
    where
        ID: Eq + Hash + Copy,
{
    adjacency: HashMap<ID, HashMap<ID, W>>,
    nodes: HashMap<ID, Node<ID, T>>,
    edges: Vec<(ID, ID)>,
    num_nodes: usize,
    num_edges: usize,
}



impl<ID, W, T> Display for HashGraph<ID, W, T> where
    ID: Eq + Hash + Copy, {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "HashGraph{{ size = {} }}", self.num_nodes)
    }
}

impl<ID, W, T> Debug for HashGraph<ID, W, T> where
    ID: Eq + Hash + Copy, {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}



impl<ID, W, T> Graph for HashGraph<ID, W, T>
    where
        ID: Eq + Hash + Copy,
{
    type ID = ID;
    type Weight = W;
    type Value = T;


    fn new() -> Self {
        HashGraph {
            adjacency: HashMap::new(),
            nodes: HashMap::new(),
            edges: Vec::new(),
            num_nodes: 0,
            num_edges: 0,
        }
    }




    fn get_node(&self, id: &ID) -> Option<&Node<ID, T>> {
        self.nodes.get(id)
    }

    fn get_node_mut(&mut self, id: &ID) -> Option<&mut Node<ID, T>> {
        self.nodes.get_mut(id)
    }

    fn add_node(&mut self, id: ID, value: T) -> GraphResult {
        let n = Node::new(id.clone(), value);
        if self.nodes.contains_key(n.get_id()) {
            return Err(IdExists);
        }

        self.nodes.insert(id, n);
        self.num_nodes += 1;
        Ok(())
    }

    fn contains_node(&self, id: &ID) -> bool {
        self.nodes.contains_key(&id)
    }

    fn add_edge_with<'a>(&mut self, u: &'a ID, v: &'a ID, weight: W) -> GraphResult {
        if !self.contains_node(u) {
            return Err(IdDoesNotExist);
        } else if !self.contains_node(v) {
            return Err(IdDoesNotExist);
        }
        let map = self.adjacency.entry(*u).or_insert(HashMap::new());
        if map.contains_key(&v) {
            return Err(EdgeAlreadyExists);
        }
        self.edges.push((*u, *v));
        self.num_edges += 1;
        map.insert(*v, weight);
        Ok(())
    }

    fn contains_edge(&self, u: &ID, v: &ID) -> bool {
        if !self.contains_node(u) || !self.contains_node(v) {
            return false;
        }
        match self.adjacency.get(u) {
            None => false,
            Some(map) => map.contains_key(&v),
        }
    }

    fn get_weight(&self, u: &ID, v: &ID) -> Option<&W> {
        if !self.contains_edge(u, v) {
            None
        } else {
            self.adjacency.get(&u).unwrap().get(&v)
        }
    }

    fn get_adjacent(&self, node: &ID) -> Vec<&ID> {
        match self.adjacency.get(&node) {
            None => Vec::new(),
            Some(map) => map.keys().collect(),
        }
    }




    fn nodes(&self) -> Vec<&Node<Self::ID, Self::Value>> {
        Vec::from_iter(self.nodes.values())
    }

    fn edges(&self) -> Vec<(&Self::ID, &Self::ID, &Self::Weight)> {
        Vec::from_iter(self.edges.iter().map(|(id1, id2)| (id1, id2, self.get_weight(id1, id2).unwrap())))
    }


    fn num_nodes(&self) -> usize {
        self.num_edges
    }

    fn num_edges(&self) -> usize {
        self.num_nodes
    }




    /*
    pub fn get_sub_graphs(&self) -> Vec<HashGraph<ID, &W, &T>> {
        unimplemented!()
    }

     */


}



impl<ID, W, T> From<(Vec<(ID, T)>, Vec<(ID, ID, W)>)> for HashGraph<ID, W, T> where
    ID: Eq + Hash + Copy, {
    fn from(input: (Vec<(ID, T)>, Vec<(ID, ID, W)>)) -> Self {
        let (nodes, edges) = input;
        let mut output = HashGraph::new();

        for (id, value) in nodes {
            output.add_node(id, value);
        }

        for (id1, id2, weight) in edges {
            output.add_edge_with(&id1, &id2, weight);
        }

        output
    }
}

impl<ID, W, T> HashGraph<ID, W, T>
    where
        ID: Eq + Hash + Copy,
        T: Copy,
{
    pub fn add_nodes_with<I>(&mut self, id: I, value: T) -> GraphResult
        where
            I: Iterator<Item = ID>,
    {
        for n in id {
            if let Err(e) = self.add_node(n, value) {
                return Err(e);
            }
        }
        Ok(())
    }
}

impl<ID, W, T> HashGraph<ID, W, T>
    where
        ID: Eq + Hash + Copy,
        T: Default,
{
    pub fn add_nodes<I>(&mut self, id: I) -> GraphResult
        where
            I: Iterator<Item = ID>,
    {
        for n in id {
            if let Err(e) = self.add_node(n, T::default()) {
                return Err(e);
            }
        }
        Ok(())
    }
}

impl<ID, W, T> Clone for HashGraph<ID, W, T>
    where
        ID: Eq + Hash + Copy,
        W: Clone,
        T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            adjacency: self.adjacency.clone(),
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
            num_nodes: self.num_nodes,
            num_edges: self.num_edges,
        }
    }
}

impl<'a, ID, W, T> HashGraph<ID, W, T>
    where
        ID: Eq + Hash + Copy,
        W: Default,
{
    ///
    /// If the `W` of the graph has a default value, allows for the adding of edges without a weight specified
    pub fn add_edge(&mut self, u: &'a ID, v: &'a ID) -> GraphResult {
        self.add_edge_with(u, v, Default::default())
    }
}

impl<ID, W, T> Index<ID> for HashGraph<ID, W, T>
    where
        ID: Eq + Hash + Copy,
        T: Copy,
{
    type Output = T;

    fn index(&self, index: ID) -> &Self::Output {
        self.nodes.get(&index).unwrap().get_value()
    }
}

impl<ID, W, T> IndexMut<ID> for HashGraph<ID, W, T>
    where
        ID: Eq + Hash + Copy,
        T: Copy,
{
    fn index_mut(&mut self, index: ID) -> &mut Self::Output {
        self.nodes.get_mut(&index).unwrap().get_value_mut()
    }
}

impl<ID, W, T> Index<(ID, ID)> for HashGraph<ID, W, T>
    where
        ID: Eq + Hash + Copy,
        T: Copy,
{
    type Output = W;

    fn index(&self, index: (ID, ID)) -> &Self::Output {
        &self.adjacency[&index.0][&index.1]
    }
}

impl<'a, ID, W, T>  GraphRef<'a, ID, W, T, HashGraph<&'a ID, &'a W, &'a T>> for HashGraph<ID, W, T> where
    ID: Eq + Hash + Copy, {
    fn to_reference_graph(&'a self) -> HashGraph<&'a ID, &'a W, &'a T> {
        let mut output = HashGraph::new();
        {

            for node in self.nodes() {
                output.add_node(
                    node.get_id(),
                    node.get_value()
                );
            }
        }
        {
            for (ref id1, ref id2, weight) in self.edges() {
                output.add_edge_with(id1, id2, weight);
            }
        }


        output
    }
}