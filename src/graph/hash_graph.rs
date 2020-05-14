use std::hash::Hash;
use std::collections::{HashMap, HashSet};
use crate::{Node, GraphRef, GraphReverse};
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

impl<ID, W, T> Default for HashGraph<ID, W, T>
    where ID: Eq + Hash + Copy{
    fn default() -> Self {
        Self {
            adjacency: Default::default(),
            nodes: Default::default(),
            edges: vec![],
            num_nodes: 0,
            num_edges: 0
        }
    }
}


impl<ID, W, T> Graph for HashGraph<ID, W, T>
    where
        ID: Eq + Hash + Copy,
{
    type ID = ID;
    type Weight = W;
    type Value = T;






    fn get_node(&self, id: &ID) -> Option<&Node<ID, T>> {
        self.nodes.get(id)
    }

    fn get_node_mut(&mut self, id: &ID) -> Option<&mut Node<ID, T>> {
        self.nodes.get_mut(id)
    }

    fn add_node_with(&mut self, id: ID, value: T) -> GraphResult {
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
        self.num_nodes
    }

    fn num_edges(&self) -> usize {
        self.num_edges
    }

    fn take_nodes(self) -> Vec<Node<Self::ID, Self::Value>> {
        self.nodes.into_iter().map(|(_, node)|
            node
        ).collect()
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
            output.add_node_with(id, value);
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
            if let Err(e) = self.add_node_with(n, value) {
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
            if let Err(e) = self.add_node_with(n, T::default()) {
                return Err(e);
            }
        }
        Ok(())
    }

    pub fn add_node(&mut self, id: ID) -> GraphResult {
        self.add_node_with(id, T::default())
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

impl<ID, W, T> HashGraph<ID, W, T>
    where
        ID: Eq + Hash + Copy {
    pub fn disassemble(mut self) -> (Vec<Node<ID, T>>, Vec<(ID, ID, W)>){
        let adj = std::mem::replace(&mut self.adjacency, HashMap::new());
        let edges =
            adj.into_iter().map(
                |(id1, map)| {
                    map.into_iter().map(
                        move |(id2, weight)| {
                            (id1, id2, weight)
                        }
                    )
                }
            ).flatten().collect();

        let nodes = self.take_nodes();

        (nodes, edges)
    }

    pub fn as_reverse(&self) -> HashGraph<&ID, &W, &T> {
        self.to_reference_graph().into_reverse()
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl<ID, W, T> HashGraph<ID, W, Option<T>>
    where
        ID: Eq + Hash + Copy
{


    pub fn unwrap(mut self) -> Option<HashGraph<ID,W,T>> {
        let mut output = HashGraph::new();
        let (nodes, edges) = self.disassemble();
        for node in nodes {
            let id = *node.get_id();
            match node.value {
                None => { return None; },
                Some(val) => {
                    output.add_node_with(id, val);
                },
            }
        }

        for (u, v, weight) in edges {
            output.add_edge_with(&u, &v, weight);
        }


        Some(output)
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
                output.add_node_with(
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

fn compare_vectors_for_element_equality<T, R>(vec1: &Vec<T>, vec2: &Vec<R>) -> bool
    where T : PartialEq<R>
{
    if vec1.len() != vec2.len() {
        false
    } else {
        let mut vec2_copy = vec2.iter().map(|val| val).collect::<Vec<&R>>();

        for vec1_value in vec1 {
            let exists = vec2_copy.iter().enumerate().find(|(pos, vec2_val)| {
                vec1_value == **vec2_val
            }).map(|(pos, _)| pos);

            match exists {
                None => {},
                Some(vec2_match) => {
                    vec2_copy.remove(vec2_match);
                },
            }

        }



        vec2_copy.is_empty()
    }
}

impl <ID, W, T, G, IDO, WO, TO> PartialEq<G> for HashGraph<ID,W,T>
    where ID : Eq + Hash + Copy,
          IDO: Eq,
          W : PartialEq<WO>,
          T : PartialEq<TO>,
          G: Graph<ID=IDO, Weight=WO, Value=TO>{
    fn eq(&self, other: &G) -> bool {
        let this_nodes = self.nodes();
        let other_nodes = other.nodes();
        let mut nodes_map: HashMap<ID, IDO> = HashMap::new(); // self(ID) -> other(ID)

        let this_nodes_set: Vec<&T> = this_nodes.iter().map(|n| n.get_value()).collect();
        let other_nodes_set: Vec<&TO> = other_nodes.iter().map(|n| n.get_value()).collect();

        if !compare_vectors_for_element_equality(&this_nodes_set, &other_nodes_set) {
            return false;
        }



        unimplemented!()
    }
}




impl<ID, W, T> GraphReverse<ID, W, T> for HashGraph<ID, W, T> where
    ID: Eq + Hash + Copy, {
    fn into_reverse(self) -> Self {
        let mut output = Self::new();
        let (nodes, edges) = self.disassemble();
        for node in nodes {
            output.add_node_with(*node.get_id(), node.value);
        }

        for (u, v, weight) in edges {
            output.add_edge_with(&v, &u, weight);
        }


        output
    }
}


#[cfg(test)]
mod test {
    use crate::graph::hash_graph::compare_vectors_for_element_equality;

    #[test]
    fn vector_element_equality() {
        let vec1 = vec![3, 5, 10];
        let vec2 = vec![10, 3, 5];

        assert!(compare_vectors_for_element_equality(&vec1, &vec2));
    }
}