use std::fmt::{Display, Formatter, Debug, Result};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

pub struct Node<ID = usize, T = ()>
{
    id: ID,
    pub value: T,
}

impl<ID, T> Display for Node<ID, T> where
    ID: PartialEq + Copy + Display,
    T : Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{{}:{}}}", self.id, self.value)
    }
}

impl<ID, T> Debug for Node<ID, T> where
    ID: PartialEq + Copy + Debug,
    T : Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{{:?}:{:?}}}", self.id, self.value)
    }
}

impl<ID, T> Clone for Node<ID, T>
    where
        ID: PartialEq + Copy,
        T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            value: self.value.clone(),
        }
    }
}

impl <ID,T> Hash for Node<ID, T>
    where ID : Hash + PartialEq + Copy {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<ID, T> PartialEq for Node<ID, T>
    where ID : PartialEq + Copy {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<ID, T> Eq for Node<ID, T>
    where ID : PartialEq + Copy {

}



impl<ID, T> Deref for Node<ID, T>
    where
        ID: PartialEq + Copy,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<ID, T> Node<ID, T> {
    pub fn new(id: ID, val: T) -> Self {
        Node { id, value: val }
    }


    pub fn get_id(&self) -> &ID {
        &self.id
    }

    pub fn into_tuple(self) -> (ID, T){
        (self.id, self.value)
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl <ID : PartialEq, T> Node<ID, T>{
    pub fn is_id(&self, k: &ID) -> bool {
        &self.id == k
    }

}
