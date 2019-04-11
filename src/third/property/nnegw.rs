use third::property::{ Zero, IsNN, ArbWeight, NNegWeight };

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
                    NNegW::Inf => std::cmp::Ordering::Equal, 
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

impl std::ops::Shl for NNegW<usize> {
    type Output = Self;
    fn shl(self, rhs: Self) -> Self {
        match rhs {
            NNegW::Some(r) => match self {
                NNegW::Some(d) => NNegW::Some(d.shl(r)), 
                other => other, 
            }
            _ => unreachable!(), 
        }
    }
}

impl std::ops::Shr for NNegW<usize> {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self {
        match rhs {
            NNegW::Some(r) => match self {
                NNegW::Some(d) => NNegW::Some(d.shr(r)), 
                other => other, 
            }
            _ => unreachable!(), 
        }
    }
}


impl<W> ArbWeight for NNegW<W> where W: Zero + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    fn zero() -> Self { NNegW::Some(W::zero()) }
    fn inf() -> Self { NNegW::Inf }
}

impl<W> NNegWeight for NNegW<W> where W: Zero + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {}
