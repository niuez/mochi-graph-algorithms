use third::property::{ Zero, ArbWeight };

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

impl std::ops::Shl for ArbW<usize> {
    type Output = Self;
    fn shl(self, rhs: Self) -> Self {
        match rhs {
            ArbW::Some(r) => match self {
                ArbW::Some(d) => ArbW::Some(d.shl(r)), 
                other => other, 
            }
            _ => unreachable!(), 
        }
    }
}

impl std::ops::Shr for ArbW<usize> {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self {
        match rhs {
            ArbW::Some(r) => match self {
                ArbW::Some(d) => ArbW::Some(d.shr(r)), 
                other => other, 
            }
            _ => unreachable!(), 
        }
    }
}


impl<W> ArbWeight for ArbW<W> where W: Zero + std::ops::Add<Output=W> + std::ops::Sub<Output=W> + std::cmp::Ord + Copy {
    fn zero() -> Self { ArbW::Some(W::zero()) }
    fn inf() -> Self { ArbW::Inf }
    fn neg_inf() -> Self { ArbW::NegInf }
}
