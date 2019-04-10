use std::ops::{ Index, IndexMut };

pub trait ID {
    fn id(&self) -> usize;
}

impl ID for usize {
    fn id(&self) -> usize { *self }
}

pub struct Properties<W: Copy> {
    vec: Vec<W>
}

impl<'a, I: ID, W: Copy> Index<&'a I> for Properties<W> {
    type Output = W;
    fn index(&self, idx: &'a I) -> & Self::Output { &self.vec[idx.id()] }
}

impl<'a, I: ID, W: Copy> IndexMut<&'a I> for Properties<W> {
    fn index_mut(&mut self, idx: &'a I) -> &mut Self::Output { &mut self.vec[idx.id()] }
}

impl<'a, W: Copy> Properties<W> {
    pub fn new(n: usize, init: &W) -> Self {
        Properties {
           vec: vec![*init; n], 
        }
    }
    pub fn iter(&'a self) -> std::slice::Iter<'a, W> { self.vec.iter() }
}