use graph::kernel::property::{ ArbWeight, NNegWeight, ToNNegWeight, ToArbWeight, IntegerWeight };
use graph::kernel::property::literal::{ Zero, IsNN, IsNum, ToArb, Integer };
use graph::property::ArbW;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NNegW<W> where W: Zero + IsNum + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    Inf,
    Some(W), 
}

impl<W> std::ops::Add for NNegW<W> where W: Zero + IsNum + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
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

impl<W> std::ops::Sub for NNegW<W> where W: Zero + IsNum + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
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


impl<W> std::cmp::PartialOrd for NNegW<W> where W: Zero + IsNum + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl<W> std::cmp::Ord for NNegW<W> where W: Zero + IsNum + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
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

impl<W> IsNN for NNegW<W> where W: Zero + IsNum + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {}

impl<W> ToNNegWeight for NNegW<W> where W: Zero + IsNum + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    type Output = Self;
    fn to_nnegw(&self) -> Self::Output {
        self.clone()
    }
}

impl<W> ToArbWeight for NNegW<W> where W: Zero + IsNum + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    type Output = ArbW<<W as ToArb>::Output>;
    fn to_arbw(&self) -> Self::Output {
        match self {
            NNegW::Inf => ArbW::Inf,
            NNegW::Some(ref num) => ArbW::Some(num.to_arb())
        }
    }
}

impl<W> std::ops::Shl<usize> for NNegW<W>
where W: Zero + IsNum + IsNN + Integer + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self {
        match self {
            NNegW::Some(d) => NNegW::Some(d.shl(rhs)), 
            other => other, 
        }
    }
}


impl<W> std::ops::Shr<usize> for NNegW<W>
where W: Zero + IsNum + IsNN + Integer + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self {
        match self {
            NNegW::Some(d) => NNegW::Some(d.shr(rhs)), 
            other => other, 
        }
    }
}



impl<W> ArbWeight for NNegW<W> where W: Zero + IsNum + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    fn inf() -> Self { NNegW::Inf }
    fn zero() -> Self { NNegW::Some(W::zero()) }
}

impl<W> NNegWeight for NNegW<W> where W: Zero + IsNum + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {}


impl<W> IntegerWeight for NNegW<W> where W: Zero + IsNum + IsNN + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy + Integer {}
