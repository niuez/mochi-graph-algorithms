use graph::kernel::graph::ID;

use std::ops::{ Index, IndexMut };

pub struct Properties<W> {
    vec: Vec<W>
}

impl<'a, I: ID, W> Index<&'a I> for Properties<W> {
    type Output = W;
    fn index(&self, idx: &'a I) -> & Self::Output { &self.vec[idx.id()] }
}

impl<'a, I: ID, W> IndexMut<&'a I> for Properties<W> {
    fn index_mut(&mut self, idx: &'a I) -> &mut Self::Output { &mut self.vec[idx.id()] }
}

impl<W> std::iter::FromIterator<W> for Properties<W> {
    fn from_iter<T>(iter: T) -> Self
    where T: IntoIterator<Item=W> {
        let mut vec = Vec::new();
        for item in iter { vec.push(item); }
        Properties { vec: vec }
    }
}

impl<'a, W: Clone> Properties<W> {
    pub fn new(n: usize, init: &W) -> Self {
        Properties {
           vec: vec![init.clone(); n], 
        }
    }
}

impl<W: Clone> Clone for Properties<W> {
    fn clone(&self) -> Self {
        Properties { vec: self.vec.clone() }
    }
}

impl<'a, W> Properties<W> {
    pub fn new_empty() -> Self {
        Properties {
            vec: Vec::new(), 
        }
    }
    pub fn push(&mut self, elem: W) {
        self.vec.push(elem);
    }
    pub fn iter(&'a self) -> std::slice::Iter<'a, W> { self.vec.iter() }
}

