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

pub trait Property: Copy + Zero {}

impl<P> Property for P where P: Copy + Zero {}


pub trait Weighted: Property + std::ops::Add<Output=Self> + std::cmp::Ord {}

impl<W> Weighted for W where W: Property + std::ops::Add<Output=W> + std::cmp::Ord {}

pub trait NonNegativeWeighted: Weighted {}

impl NonNegativeWeighted for usize {}

pub trait Capacity: Weighted + std::ops::Sub<Output=Self>{}

impl Capacity for usize {}

pub trait Cost: Capacity + std::ops::Mul<Output=Self> {}


pub trait ID {
    fn id(&self) -> usize;
}

impl ID for usize {
    fn id(&self) -> usize { *self }
}

pub struct Properties<W: Property> {
    vec: Vec<W>
}

impl<'a, I: ID, W: Property> Index<&'a I> for Properties<W> {
    type Output = W;
    fn index(&self, idx: &'a I) -> & Self::Output { &self.vec[idx.id()] }
}

impl<'a, I: ID, W: Property> IndexMut<&'a I> for Properties<W> {
    fn index_mut(&mut self, idx: &'a I) -> &mut Self::Output { &mut self.vec[idx.id()] }
}

impl<W: Property> Properties<W> {
    pub fn new(n: usize) -> Self {
        Properties {
           vec: vec![W::zero();n], 
        }
    }
}
