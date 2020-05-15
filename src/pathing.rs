use crate::Graph;
use std::ops::{Add, Sub};
use num_traits::Num;
use map_vec::{Map, Set};
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::thread::current;

pub mod spatial;

pub trait PathFinder<ID : PartialEq + Clone, W : Num> {

    fn find_path(&self, from: &ID, to: &ID) -> Option<(Vec<ID>, W)>;
}

pub trait WeightRepr<W> where
W : PartialOrd + Add + Sub {
    fn into_weight(&self) -> W;
}

impl WeightRepr<usize> for () {
    fn into_weight(&self) -> usize {
        1
    }
}

pub struct StatelessPathFinder<ID : Eq, W, T, G : Graph<ID=ID,Weight=W,Value=T>>(G);

impl<ID: Eq, W, T, G: Graph<ID=ID, Weight=W, Value=T>> StatelessPathFinder<ID, W, T, G> {

    pub fn new(graph: G) -> Self {
        Self(graph)
    }
}

/*
impl<ID: Eq, W: PartialOrd + Add + Sub , T, G: Graph<ID=ID, Weight=W, Value=T>> PathFinder<ID, W> for StatelessPathFinder<ID, W, T, G> {
    fn find_path(&self, from: &ID, to: &ID) -> Option<(Vec<&ID>, W)> {
        unimplemented!()
    }
}

 */

impl <T : Num+ PartialOrd + Add + Sub + Clone> WeightRepr<T> for T {
    fn into_weight(&self) -> T {
        self.clone()
    }
}

struct IdWrapper<'a, ID : Eq, W : PartialOrd>(&'a ID, Option<W>);

impl<'a, 'b, ID : Eq, W : PartialOrd> PartialEq for IdWrapper<'a, ID, W> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.1, &other.1) {
            (Some(this), Some(other)) => {
                this.eq(other)
            },
            (Some(_), None) => {
                false
            },
            (None, Some(_)) => {
                false
            },
            (None, None) => {
                true
            }
        }
    }
}

impl<'a, 'b, ID : Eq, W : PartialOrd> PartialOrd for IdWrapper<'a, ID, W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self.1, &other.1) {
            (Some(this), Some(other)) => {
                other.partial_cmp(this)
            },
            (Some(_), None) => {
                Some(Greater)
            },
            (None, Some(_)) => {
                Some(Less)
            },
            (None, None) => {
                Some(Equal)
            }
        }
    }
}

impl<'a, 'b, ID : Eq, W : PartialOrd> Ord for IdWrapper<'a, ID, W> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a, 'b, ID : Eq, W : PartialOrd> Eq for IdWrapper<'a, ID, W> {

}


impl<ID: Eq + Clone, WI : Num + PartialOrd + Add + Sub + Clone, W : WeightRepr<WI>, T, G: Graph<ID=ID, Weight=W, Value=T>> PathFinder<ID, WI> for StatelessPathFinder<ID, W, T, G> {


    fn find_path(&self, from: &ID, to: &ID) -> Option<(Vec<ID>, WI)> {
        let mut visited = Set::new();
        let mut prev = Map::new();


        let mut distance = Map::new();
        distance.insert(from, WI::zero());

        let long_life_time_borrow = self.0.nodes().iter().map(|node| {
            (node.get_id(), node.get_id())
        }).collect::<Map<&ID, &ID>>();


        let mut queue = BinaryHeap::new();
        queue.push(IdWrapper(from, distance.get(from).map(|w| w.clone())));

        loop {
            let current =
                loop {
                    let node = queue.pop();
                    if let Some(node) = node {
                        if !visited.contains(node.0) {
                            break Some(node.0);
                        }
                    } else {
                        break None;
                    }

                };
            if current.is_none() {
                break;
            }

            let current = current.unwrap();
            visited.insert(current);

            if current == to {
                break;
            }

            let current_distance = distance.get(current).expect("Distance should exist").clone();
            for adj in self.0.get_adjacent(current) {

                if !visited.contains(adj) {

                    let path_length = match self.0.get_weight(current, adj) {
                        None => { panic!("This souldn't happen, weight should always exist")},
                        Some(weight) => {
                            weight
                        },
                    }.into_weight();


                    let fixed = long_life_time_borrow[adj];
                    let new_distance: WI = path_length + current_distance.clone();

                     match distance.get(adj) {
                        None => {
                            distance.insert(adj, new_distance.clone());
                            prev.insert(adj, current);
                        },
                        Some(old_distance) => {
                            if new_distance < *old_distance {
                                *distance.get_mut(adj).unwrap() = new_distance.clone();
                                *prev.get_mut(adj).unwrap() = current;


                            }
                        },
                    }

                    queue.push(IdWrapper(fixed, Some(new_distance)));


                }
            }
        }

        if visited.contains(to) {

            let mut output = vec![];
            let mut total_weight: WI = WI::zero();
            let mut ptr = Some(&to);

            while ptr.is_some() {
                let current = *ptr.unwrap();
                output.insert(0, current.clone());
                let prev = prev.get(current);

                match prev {
                    None => {},
                    Some(prev) => {
                        let weight = self.0.get_weight(prev, current).unwrap().into_weight();
                        total_weight = total_weight + weight;
                    },
                }

                ptr = prev;

            }

            // output.push(from.clone());

            Some((output, total_weight))
        } else {
            None
        }
    }
}



