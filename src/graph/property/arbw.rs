use graph::kernel::property::{ ArbWeight, ToNNegWeight, ToArbWeight };
use graph::kernel::property::literal::{ Zero, ToNNeg, IsNum, IsNN, Integer };
use graph::property::NNegW;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ArbW<W> where W: Zero + IsNum + std::ops::Add<Output=W> + std::cmp::Ord + Copy {
    Inf, 
    Some(W),
    NegInf,
}

impl<W> std::ops::Add for ArbW<W> where W: Zero + IsNum + std::ops::Add<Output=W> + std::cmp::Ord + Copy {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match self {
            ArbW::Inf => {
                match rhs {
                    ArbW::NegInf => unreachable!("can't resolve inf + neginf"),
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
                    ArbW::Inf => unreachable!("can't resolve neginf + inf"),
                    _ => ArbW::NegInf,
                }
            }
        }
    }

}

impl<W> std::ops::Sub for ArbW<W> where W: Zero + IsNum + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        match self {
            ArbW::Inf => {
                match rhs {
                    ArbW::Inf => unreachable!("can't resolve inf - inf"),
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
                    ArbW::NegInf => unreachable!("can't resolve neginf - neginf"),
                    _ => ArbW::NegInf,
                }
            }
        }
    }
}

impl<W,X> std::ops::Mul<ArbW<X>> for ArbW<W> 
where W: Zero + IsNum + std::ops::Add<Output=W> + std::cmp::Ord + Copy + std::ops::Mul<Output=W>,
      X: Zero + IsNum + std::ops::Add<Output=X> + std::cmp::Ord + Copy + Into<W> {
    type Output = Self;
    fn mul(self, rhs: ArbW<X>) -> Self {
        match self {
            ArbW::Inf => {
                match rhs {
                    ArbW::NegInf => ArbW::NegInf,
                    _ => ArbW::Inf,
                }
            }
            ArbW::Some(d) => {
                match rhs {
                    ArbW::Inf => ArbW::Inf,
                    ArbW::Some(d2) => ArbW::Some(d.mul(d2.into())),
                    ArbW::NegInf => ArbW::NegInf,
                }
            }
            ArbW::NegInf => {
                match rhs {
                    ArbW::NegInf => ArbW::Inf,
                    _ => ArbW::NegInf,
                }
            }
        }
    }
}

impl<W,X> std::ops::Mul<NNegW<X>> for ArbW<W> 
where W: Zero + IsNum + std::ops::Add<Output=W> + std::cmp::Ord + Copy + std::ops::Mul<Output=W>,
      X: Zero + IsNum + IsNN + std::ops::Add<Output=X> + std::cmp::Ord + Copy + Into<W> {
    type Output = Self;
    fn mul(self, rhs: NNegW<X>) -> Self {
        match self {
            ArbW::Inf => {
                ArbW::Inf
            }
            ArbW::Some(d) => {
                match rhs {
                    NNegW::Inf => ArbW::Inf,
                    NNegW::Some(d2) => ArbW::Some(d.mul(d2.into())),
                }
            }
            ArbW::NegInf => {
                ArbW::NegInf
            }
        }
    }
}


impl<W> std::cmp::PartialOrd for ArbW<W> where W: Zero + IsNum + std::ops::Add<Output=W> + std::cmp::Ord + Copy {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl<W> std::cmp::Ord for ArbW<W> where W: Zero + IsNum + std::ops::Add<Output=W> + std::cmp::Ord + Copy {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        match self {
            ArbW::Inf => {
                match rhs {
                    ArbW::Inf => std::cmp::Ordering::Equal,
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
                    ArbW::NegInf => std::cmp::Ordering::Equal,
                    _ => std::cmp::Ordering::Less, 
                }
            }
        }
    }
}

impl<W> ToNNegWeight for ArbW<W> where W: Zero + IsNum + std::ops::Add<Output=W> + std::cmp::Ord + Copy {
    type Output = NNegW<<W as ToNNeg>::Output>;
    fn to_nnegw(&self) -> Self::Output {
        match self {
            ArbW::Inf => NNegW::Inf,
            ArbW::Some(ref num) => NNegW::Some(num.to_nneg()),
            ArbW::NegInf => unreachable!("neginf can't convert to non-negative weight"), 
        }
    }
}

impl<W> ToArbWeight for ArbW<W> where W: Zero + IsNum + std::ops::Add<Output=W> + std::cmp::Ord + Copy {
    type Output = Self;
    fn to_arbw(&self) -> Self::Output {
        self.clone()
    }
}

impl<W> std::ops::Shl<usize> for ArbW<W>
where W: Zero + IsNum + Integer + std::ops::Add<Output=W> + std::cmp::Ord + Copy {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self {
        match self {
            ArbW::Some(d) => ArbW::Some(d.shl(rhs)), 
            inf => inf, 
        }
    }
}

impl<W> std::ops::Shr<usize> for ArbW<W> 
where W: Zero + IsNum + Integer + std::ops::Add<Output=W> + std::cmp::Ord + Copy + std::ops::Shr<usize, Output=W> {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self {
        match self {
            ArbW::Some(d) => ArbW::Some(d.shr(rhs)),
            inf => inf,
        }
    }
}


impl<W> ArbWeight for ArbW<W> where W: Zero + IsNum + std::ops::Add<Output=W> + std::cmp::Ord + Copy {
    fn inf() -> Self { ArbW::Inf }
    fn zero() -> Self { ArbW::Some(W::zero()) }
    fn neg_inf() -> Self { ArbW::NegInf }
}
