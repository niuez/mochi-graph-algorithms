use std::ops::{Index, IndexMut};

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for usize { fn zero() -> Self { 0 } }
impl Zero for u8 { fn zero() -> Self { 0 } }
impl Zero for u16 { fn zero() -> Self { 0 } }
impl Zero for u32 { fn zero() -> Self { 0 } }
impl Zero for u64 { fn zero() -> Self { 0 } }
impl Zero for isize { fn zero() -> Self { 0 } }
impl Zero for i8 { fn zero() -> Self { 0 } }
impl Zero for i16 { fn zero() -> Self { 0 } }
impl Zero for i32 { fn zero() -> Self { 0 } }
impl Zero for i64 { fn zero() -> Self { 0 } }

pub trait Property: Copy {}

impl<P> Property for P where P: Copy {}

pub trait Weight: Property + std::ops::Add<Output=Self> + std::cmp::Ord + Zero {}

impl<W> Weight for W where W: Property + std::ops::Add<Output=W> + std::cmp::Ord + Zero {}

pub trait NNWeight: Weight {}

impl NNWeight for usize {}

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
