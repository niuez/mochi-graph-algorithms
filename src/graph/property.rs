pub trait Property: Copy {}

impl<P> Property for P where P: Copy {}

pub trait Weighted: Property + std::ops::Add<Output=Self> + std::cmp::Ord{}

impl<W> Weighted for W where W: Property + std::ops::Add<Output=W> + std::cmp::Ord{}

pub trait NonNegativeWeighted: Weighted {}

impl NonNegativeWeighted for usize {}
