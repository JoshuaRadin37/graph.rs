use crate::Node;

pub trait Heuristic<T, W> {

    fn get_heuristic_value(&self, node1: &Node<T>, node2: &Node<T>) -> W;
}


impl <F, T, W> Heuristic<T, W> for F where
    F : Fn(&Node<T>, &Node<T>) -> W {
    fn get_heuristic_value(&self, node1: &Node<T, ()>, node2: &Node<T, ()>) -> W {
        self(node1, node2)
    }
}