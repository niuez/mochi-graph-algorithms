/// `Property` trait is for properties of edges.
pub trait Property: Copy {}

impl<P> Property for P where P: Copy {}

/// `Weighted` trait is for properties of edges such as distance,cost and so on.
pub trait Weighted: Property + std::ops::Add<Output=Self> + std::cmp::Ord{}

impl<W> Weighted for W where W: Property + std::ops::Add<Output=W> + std::cmp::Ord{}

/// `NonNegativeWeighted` trait is for algorithm such Dijkstra's Algorithm.
pub trait NonNegativeWeighted: Weighted {}

impl NonNegativeWeighted for usize {}

pub trait Capacity: Property + std::ops::Add<Output=Self> + std::ops::Sub<Output=Self> + std::cmp::Ord {}

impl Capacity for usize {}
