pub mod arbw;
pub mod nnegw;
pub mod properties;
pub mod literal;

pub use graph::property::arbw::ArbW;
pub use graph::property::nnegw::NNegW;
pub use graph::property::properties::{ ID, Properties };

pub trait Property: Copy {}

impl<P> Property for P where P: Copy {}

pub trait ToNNegWeight {
    type Output: NNegWeight;
    fn to_nnegw(&self) -> Self::Output;
}

pub trait ToArbWeight {
    type Output: ArbWeight;
    fn to_arbw(&self) -> Self::Output;
}

pub trait ArbWeight where Self: ToNNegWeight + ToArbWeight + Property + std::ops::Add<Output=Self> + std::ops::Sub<Output=Self> + std::cmp::Ord {
    fn inf() -> Self;
    fn zero() -> Self;
    fn neg_inf() -> Self { unreachable!() }
}

pub trait NNegWeight where Self: ArbWeight {}

pub trait IntegerWeight: ArbWeight {}

