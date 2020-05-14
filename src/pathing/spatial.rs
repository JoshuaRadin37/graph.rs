use crate::{Node, Graph};
use std::ops::{Deref, Sub, Add};

mod astar;
pub use astar::HeuristicPathFinder;
use num_traits::{Num, Float, PrimInt, ToPrimitive};
use num_traits::real::Real;
use crate::directed::Undirected;

pub trait  Heuristic<ID : Eq, T, W> {

    fn get_heuristic_value(&self, node1: &Node<ID, T>, node2: &Node<ID, T>) -> W;
}


impl <F, W, ID : Eq> Heuristic<ID, (), W> for F where
    F : Fn(&Node<ID, ()>, &Node<ID, ()>) -> W {
    fn get_heuristic_value(&self, node1: &Node<ID, ()>, node2: &Node<ID, ()>) -> W {
        self(node1, node2)
    }
}


pub trait Location<W> {
    fn distance_to(&self, other: &Self) -> W;
}

pub struct Point<N : Num + Sub + Add> {
    x: N,
    y: N
}

impl <N : Num> Point<N> {

    pub fn get_x(&self) -> &N {
        &self.x
    }

    pub fn get_y(&self) -> &N {
        &self.y
    }

    pub fn get_x_mut(&mut self) -> &mut N {
        &mut self.x
    }

    pub fn get_y_mut(&mut self) -> &mut N {
        &mut self.y
    }
}


impl <N : Num + Sub + Add + Float> Point<N> {
    pub fn new(x: N, y: N) -> Self {
        Self{ x, y}
    }

}

impl <N : Num + PrimInt + ToPrimitive> Point<N> {
    pub fn newi(x: N, y: N) -> Point<f64> {
        Point{ x: x.to_f64().unwrap(), y: y.to_f64().unwrap() }
    }


}

impl <N : Num + Sub + Add + Real> Location<N> for Point<N> {
   
    fn distance_to(&self, other: &Self) -> N {
        let x = *self.get_x()- *other.get_x();
        let y = *self.get_y() - *other.get_y();
        (x*x + y*y).sqrt()
    }
    
}

impl <ID : Eq, N : Num + Sub + Add + Real>  Heuristic<ID, Point<N>, N> for Point<N> {
    
    fn get_heuristic_value(&self, point1 : &Node<ID, Point<N>>, point2: &Node<ID, Point<N>>) -> N {
        point1.get_value().distance_to(point2.get_value())
    }
}



#[cfg(test)]
mod test {
    use crate::pathing::spatial::{Point, Location};
    use crate::directed::{Undirected, Directed};
    use crate::{HashGraph, Graph};

    #[test]
    fn distance_heuristic() {
        let point1 = Point::newi(0, 0);
        let point2 = Point::newi(4, 3);

        assert_eq!(point1.distance_to(&point2), 5.0, "3 4 5 triangle");

        let mut map = Directed::from(HashGraph::<usize, f64, Point<f64>>::new());
        map.add_node_with(0, point1).unwrap();
        map.add_node_with(1, point2).unwrap();
        map.add_edge_distance(&0, &1);
        assert_eq!(map.get_weight(&0, &1), Some(&5.0), "Weight should carry over");


    }

}
