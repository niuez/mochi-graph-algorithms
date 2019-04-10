use std::ops::{Index, IndexMut};

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for usize { fn zero() -> Self { 0 } }
impl Zero for u8 { fn zero() -> Self { 0 } }
impl Zero for u16 { fn zero() -> Self { 0 } }
impl Zero for u32 { fn zero() -> Self { 0 } } impl Zero for u64 { fn zero() -> Self { 0 } } impl Zero for isize { fn zero() -> Self { 0 } }
impl Zero for i8 { fn zero() -> Self { 0 } }
impl Zero for i16 { fn zero() -> Self { 0 } }
impl Zero for i32 { fn zero() -> Self { 0 } }
impl Zero for i64 { fn zero() -> Self { 0 } }

pub trait IsNN {}

impl IsNN for usize {}
impl IsNN for u8 {}
impl IsNN for u16 {}
impl IsNN for u32 {}
impl IsNN for u64 {}

pub trait Property: Copy {}

impl<P> Property for P where P: Copy {}

pub trait ArbWeight: Property + std::ops::Add<Output=Self> + std::ops::Sub<Output=Self> + std::cmp::Ord {
    fn zero() -> Self;
    fn inf() -> Self;
    fn neg_inf() -> Self { unreachable!() }
}

pub trait NNegWeight: ArbWeight {}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ArbW<W> where W: Zero + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    Inf, 
    Some(W),
    NegInf,
}

impl<W> std::ops::Add for ArbW<W> where W: Zero + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match self {
            ArbW::Inf => {
                match rhs {
                    ArbW::NegInf => unreachable!(),
                    _ => ArbW::Inf,
                }
            }
            ArbW::Some(d) => {
                match rhs {
                    ArbW::Inf => ArbW::Inf,
                    ArbW::Some(d2) => ArbW::Some(d + d2),
                    ArbW::NegInf => ArbW::NegInf,
                }
            }
            ArbW::NegInf => {
                match rhs {
                    ArbW::Inf => unreachable!(),
                    _ => ArbW::NegInf,
                }
            }
        }
    }

}

impl<W> std::ops::Sub for ArbW<W> where W: Zero + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        match self {
            ArbW::Inf => {
                match rhs {
                    ArbW::Inf => unreachable!(),
                    _ => ArbW::Inf,
                }
            }
            ArbW::Some(d) => {
                match rhs {
                    ArbW::Inf => ArbW::NegInf,
                    ArbW::Some(d2) => ArbW::Some(d - d2),
                    ArbW::NegInf => ArbW::Inf,
                }
            }
            ArbW::NegInf => {
                match rhs {
                    ArbW::NegInf => unreachable!(),
                    _ => ArbW::NegInf,
                }
            }
        }
    }
}


impl<W> std::cmp::PartialOrd for ArbW<W> where W: Zero + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl<W> std::cmp::Ord for ArbW<W> where W: Zero + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        match self {
            ArbW::Inf => {
                match rhs {
                    ArbW::Inf => unreachable!(),
                    _ => std::cmp::Ordering::Greater,
                }
            }
            ArbW::Some(d) => {
                match rhs {
                    ArbW::Inf => std::cmp::Ordering::Less,
                    ArbW::Some(d2) => d.cmp(d2), 
                    ArbW::NegInf => std::cmp::Ordering::Greater,
                }
            }
            ArbW::NegInf => {
                match rhs {
                    ArbW::NegInf => unreachable!(),
                    _ => std::cmp::Ordering::Less, 
                }
            }
        }
    }
}


impl<W> ArbWeight for ArbW<W> where W: Zero + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    fn zero() -> Self { ArbW::Some(W::zero()) }
    fn inf() -> Self { ArbW::Inf }
    fn neg_inf() -> Self { ArbW::NegInf }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NNegW<W> where W: Zero + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    Inf,
    Some(W), 
}

impl<W> std::ops::Add for NNegW<W> where W: Zero + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match self {
            NNegW::Inf => {
                NNegW::Inf
            }
            NNegW::Some(d) => {
                match rhs {
                    NNegW::Inf => NNegW::Inf,
                    NNegW::Some(d2) => NNegW::Some(d + d2),
                }
            }
        }
    }

}

impl<W> std::ops::Sub for NNegW<W> where W: Zero + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        match self {
            NNegW::Inf => {
                match rhs {
                    NNegW::Inf => unreachable!(),
                    _ => NNegW::Inf,
                }
            }
            NNegW::Some(d) => {
                match rhs {
                    NNegW::Inf => unreachable!(), 
                    NNegW::Some(d2) => NNegW::Some(d - d2),
                }
            }
        }
    }
}


impl<W> std::cmp::PartialOrd for NNegW<W> where W: Zero + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl<W> std::cmp::Ord for NNegW<W> where W: Zero + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        match self {
            NNegW::Inf => {
                match rhs {
                    NNegW::Inf => unreachable!(),
                    _ => std::cmp::Ordering::Greater,
                }
            }
            NNegW::Some(d) => {
                match rhs {
                    NNegW::Inf => std::cmp::Ordering::Less,
                    NNegW::Some(d2) => d.cmp(d2), 
                }
            }
        }
    }
}


impl<W> ArbWeight for NNegW<W> where W: Zero + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    fn zero() -> Self { NNegW::Some(W::zero()) }
    fn inf() -> Self { NNegW::Inf }
}

impl<W> NNegWeight for NNegW<W> where W: Zero + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {}

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
