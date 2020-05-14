use crate::Graph;
use crate::pathing::spatial::Heuristic;

pub struct HeuristicPathFinder<ID : Eq, W, T : Heuristic<ID, T, W>, G : Graph<ID=ID,Weight=W,Value=T>>(G);