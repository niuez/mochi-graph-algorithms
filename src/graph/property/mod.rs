pub mod arbw;
pub mod nnegw;
pub mod properties;

pub use graph::property::arbw::ArbW;
pub use graph::property::nnegw::NNegW;
pub use graph::property::properties::{ ID, Properties };

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

