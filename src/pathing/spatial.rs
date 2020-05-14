use crate::Node;

pub trait  Heuristic<ID : Eq, T, W> {

    fn get_heuristic_value(&self, node1: &Node<ID, T>, node2: &Node<ID, T>) -> W;
}


impl <F, T, W, ID : Eq> Heuristic<T, W> for F where
    F : Fn(&Node<ID, ()>, &Node<ID, ()>) -> W {
    fn get_heuristic_value(&self, node1: &Node<ID, ()>, node2: &Node<ID, ()>) {
        self(node1, node2)
    }
}


pub trait Location<W> {
    fn distance_to(&self, other: &Self) -> W;
}

pub struct Point {
    x: f64,
    y: f64
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self{ x, y}
    }

    pub fn get_x(&self) -> &f64 {
        &self.x
    }

    pub fn get_y(&self) -> &f64 {
        &self.y
    }

    pub fn get_x_mut(&mut self) -> &mut f64 {
        &mut self.x
    }

    pub fn get_y_mut(&mut self) -> &mut f64 {
        &mut self.y
    }
}

impl Location<f64> for Point {
   
    fn distance_to(&self, other: &Self) -> f64 {
        ((self.get_x()-other.get_x()).powi(2) + (self.get_y() - other.get_y()).powi(2)).sqrt()
    }
    
}

impl <ID : Eq>  Heuristic<Node<ID, Point>, f64> for Point {
    
    fn get_heuristic_value(&self, point1 : &Node<ID, Point>, point2: &Node<ID, Point>) -> f64 {
        *point1.distance_to(*point2)
    }
}

#[mod(test)]
mod test {

    fn distance_heuristic() {
        


    }

}
