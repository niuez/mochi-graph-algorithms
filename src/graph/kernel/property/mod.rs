pub mod literal;

/// Trait for properties.
pub trait Property: Copy {}

impl<P> Property for P where P: Copy {}

/// Types implementing `ToNNegWeight` are able to convert to non-negative weights.
/// This trait use the algorithms with potentials (`dijkstra_with_potential`, etc...).
pub trait ToNNegWeight {

    /// converting type.
    type Output: NNegWeight;

    /// convert to non-negative weights.
    fn to_nnegw(&self) -> Self::Output;
}

/// Types implementing `ToARbWeight` are able to convert to arbitrary weights.
/// This trait use to reverse from non-negative weight after converting weight.
pub trait ToArbWeight {

    /// converting type.
    type Output: ArbWeight;

    /// convert to non-negative weights.
    fn to_arbw(&self) -> Self::Output;
}

/// Trait of arbitrary weights.
/// the arbirary weight has infinity, zero and negative infinity.
pub trait ArbWeight where Self: ToNNegWeight + ToArbWeight + Property + std::ops::Add<Output=Self> + std::ops::Sub<Output=Self> + std::cmp::Ord {
    fn inf() -> Self;
    fn zero() -> Self;
    fn neg_inf() -> Self { unreachable!() }
}

/// Trait of non-negative weights.
pub trait NNegWeight where Self: ArbWeight {}

/// Trait of weights of integer. 
/// types implementing this use the scaling algorithms.
pub trait IntegerWeight: ArbWeight + std::ops::Shl<usize, Output=Self> + std::ops::Shr<usize, Output=Self> {}

/// Trait of capacity for maxflow, mcf, and so on.
pub trait Capacity: NNegWeight + IntegerWeight {}

impl<W> Capacity for W where W: NNegWeight + IntegerWeight {}

