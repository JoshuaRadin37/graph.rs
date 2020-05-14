use crate::{Graph, HashGraph, GraphResult, Node};
use std::ops::{DerefMut, Deref};
use std::fmt::{Display, Formatter, Result, Debug};
use std::hash::Hash;
use std::collections::HashSet;
use map_vec::Set;


#[derive(Debug)]
pub struct Directed<ID, W, T, G>(G)
    where G : Graph<ID=ID, Weight=W, Value=T>,
          ID: Eq;

impl<ID, W, T, G> From<G> for Directed<ID, W, T, G> where G: Graph<ID=ID, Weight=W, Value=T>,
                                              ID: Eq {
    fn from(g: G) -> Self {
        Directed::new_from(g)
    }
}

impl<ID, W, T, G> Deref for Directed<ID, W, T, G> where
    G: Graph<ID=ID, Weight=W, Value=T>,
    ID: Eq {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<ID, W, T, G> DerefMut for Directed<ID, W, T, G> where
    G: Graph<ID=ID, Weight=W, Value=T>,
    ID: Eq {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn new_hashed_directed<ID, W, T>() -> Directed<ID, W, T, HashGraph<ID, W, T>> where
    ID : Eq + Hash + Copy {
    Directed(HashGraph::new())
}

impl<ID, W, T, G> Directed<ID, W, T, G> where G: Graph<ID=ID, Weight=W, Value=T>,
                                              ID: Eq {

    pub fn new_from(graph: G) -> Self {
        Self(graph)
    }


}





pub struct Undirected<ID, W, T, G>(G)
    where G : Graph<ID=ID, Weight=W, Value=T>,
          ID: Eq,
          W : Clone;

impl<ID, W, T, G> From<G> for Undirected<ID, W, T, G> where G: Graph<ID=ID, Weight=W, Value=T>,
                                                ID: Eq,
                                                W: Clone {
    fn from(g: G) -> Self {
        Undirected::new_from(g)
    }
}

impl<ID, W, T, G> Undirected<ID, W, T, G> where G: Graph<ID=ID, Weight=W, Value=T>,
                                                ID: Eq,
                                                W: Clone,
                                                T : Default {

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


impl<'a, ID, W, T, G> Undirected<ID, W, T, G> where G: Graph<ID=ID, Weight=W, Value=T>,
                                                ID: Eq,
                                                W: Clone + Default {

    pub fn add_edge(&mut self, u: &'a ID, v: &'a ID) -> GraphResult {
        self.add_edge_with(u, v, Default::default())
    }

}

impl<ID, W, T, G> Debug for Undirected<ID, W, T, G> where G: Graph<ID=ID, Weight=W, Value=T>,
                                                          W: Clone,
                                                          ID: Eq {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}


impl<ID, W, T, G> Display for Undirected<ID, W, T, G> where G: Graph<ID=ID, Weight=W, Value=T>,
                                                            W: Clone,
                                                            ID: Eq {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.0)
    }
}


impl<ID, W, T, G> Undirected<ID, W, T, G> where G: Graph<ID=ID, Weight=W, Value=T>,
                                                W: Clone,
                                                ID: Eq {

    pub fn new_from(graph: G) -> Self {
        Self(graph)
    }
}

pub fn new_hashed_undirected<ID, W, T>() -> Undirected<ID, W, T, HashGraph<ID, W, T>> where
    ID : Eq + Hash + Copy,
    W : Clone {
    Undirected(HashGraph::new())
}


impl<ID, W, T, G> Graph for Undirected<ID, W, T, G> where
    G: Graph<ID=ID, Weight=W, Value=T>,
    W: Clone,
    ID : Eq
{
    type ID = ID;
    type Weight = W;
    type Value = T;



    fn get_node(&self, id: &Self::ID) -> Option<&Node<Self::ID, Self::Value>> {
        self.0.get_node(id)
    }

    fn get_node_mut(&mut self, id: &Self::ID) -> Option<&mut Node<Self::ID, Self::Value>> {
        self.0.get_node_mut(id)
    }

    fn add_node_with(&mut self, id: Self::ID, value: Self::Value) -> GraphResult {
        self.0.add_node_with(id, value)
    }

    fn contains_node(&self, id: &Self::ID) -> bool {
        self.0.contains_node(id)
    }

    fn add_edge_with(&mut self, u: &Self::ID, v: &Self::ID, weight: Self::Weight) -> GraphResult {
        self.0.add_edge_with(u, v, weight.clone())?;
        self.0.add_edge_with(v, u, weight)
    }

    fn contains_edge(&self, u: &Self::ID, v: &Self::ID) -> bool {
        self.0.contains_edge(u, v)
    }

    fn get_weight(&self, u: &Self::ID, v: &Self::ID) -> Option<&Self::Weight> {
        self.0.get_weight(u, v)
    }

    fn get_adjacent(&self, node: &Self::ID) -> Vec<&Self::ID> {
        self.0.get_adjacent(node)
    }

    fn nodes(&self) -> Vec<&Node<Self::ID, Self::Value>> {
        self.0.nodes()
    }

    fn edges(&self) -> Vec<(&Self::ID, &Self::ID, &Self::Weight)> {
        let mut edges = Set::new();
        self.0.edges().into_iter().filter(|edge| {
            let (u, v, _) = edge;
            let alter = (*v, *u);
            if edges.contains(&alter) {
                false
            }  else {
                edges.insert((*u, *v));
                true
            }
        }).collect()
    }

    fn num_nodes(&self) -> usize {
        self.0.num_edges()
    }

    fn num_edges(&self) -> usize {
        self.0.num_edges() / 2
    }

    fn take_nodes(self) -> Vec<Node<Self::ID, Self::Value>> {
        self.0.take_nodes()
    }
}





#[cfg(test)]
mod test {
    use crate::directed::{Directed, new_hashed_directed, new_hashed_undirected};
    use crate::Graph;

    struct Wrapper(u32);

    #[test]
    fn transparency() {

        let mut d = new_hashed_directed();
        d.add_node_with(32, Wrapper(15));
        d.add_edge_with(&32, &44, ());

        let mut d= new_hashed_undirected();
        d.add_node_with(32, Wrapper(15));
        d.add_edge_with(&32, &44, ());

    }

    #[test]
    fn test_undirected_behavior() {
        let mut undirected = new_hashed_undirected::<usize, usize, ()>();
        undirected.add_nodes(0..2).unwrap();
        undirected.add_edge_with(&0, &1, 15).expect("Should use Undirected implementation");
        assert_eq!(undirected.num_edges(), 1, "Undirected should only show 1 edge");
        assert_eq!(undirected.0.num_edges(), 2, "Backing graph has double the amount of edges");
    }
}