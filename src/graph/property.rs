/// `Property` trait is for properties of edges.
pub trait Property: Copy {}

impl<P> Property for P where P: Copy {}

pub trait ZeroP {
    fn zero() -> Self;
}

impl ZeroP for usize { fn zero() -> Self { 0 } }
impl ZeroP for u8 { fn zero() -> Self { 0 } }
impl ZeroP for u16 { fn zero() -> Self { 0 } }
impl ZeroP for u32 { fn zero() -> Self { 0 } }
impl ZeroP for u64 { fn zero() -> Self { 0 } }
impl ZeroP for isize { fn zero() -> Self { 0 } }
impl ZeroP for i8 { fn zero() -> Self { 0 } }
impl ZeroP for i16 { fn zero() -> Self { 0 } }
impl ZeroP for i32 { fn zero() -> Self { 0 } }
impl ZeroP for i64 { fn zero() -> Self { 0 } }

/// `Weighted` trait is for properties of edges such as distance,cost and so on.
pub trait Weighted: Property + std::ops::Add<Output=Self> + std::cmp::Ord + ZeroP {}

impl<W> Weighted for W where W: Property + std::ops::Add<Output=W> + std::cmp::Ord + ZeroP {}

/// `NonNegativeWeighted` trait is for algorithm such Dijkstra's Algorithm.
pub trait NonNegativeWeighted: Weighted {}

impl NonNegativeWeighted for usize {}

pub trait Capacity: Weighted + std::ops::Sub<Output=Self>{}

impl Capacity for usize {}

pub trait Cost: Capacity + std::ops::Mul<Output=Self> {}
