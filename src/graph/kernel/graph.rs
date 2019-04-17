use graph::kernel::property::*;

pub trait ID {
    fn id(&self) -> usize;
}

impl ID for usize {
    fn id(&self) -> usize { *self }
}

pub trait Vertex: ID + Eq + Clone { }

impl<V: ID + Eq + Clone> Vertex for V { }

pub trait Edge {
    type VType: Vertex;
    fn from(&self) -> &Self::VType;
    fn to(&self) -> &Self::VType;
}

impl<V> Edge for (V, V) where V: Vertex { 
    type VType = V;
    fn from(&self) -> &Self::VType { &self.0 }
    fn to(&self) -> &Self::VType { &self.1 }
}

impl<V, P> Edge for (V, V, P) where V: Vertex, P: Property { 
    type VType = V;
    fn from(&self) -> &Self::VType { &self.0 }
    fn to(&self) -> &Self::VType { &self.1 }
}

pub trait AdjEdge: ID {
    type VType: Vertex;
    type EType: Edge<VType=Self::VType>;
    fn from(&self) -> &Self::VType;
    fn to(&self) -> &Self::VType;
    fn edge(&self) -> &Self::EType;
}

pub trait ResidualEdge: AdjEdge {
    fn rev(&self) -> Self;
}

pub trait Graph<'a> {
    type VType: Vertex + 'a;
    type EType: Edge<VType=Self::VType>;
    type AEType: AdjEdge<VType=Self::VType, EType=Self::EType>;
    type AdjIter: std::iter::Iterator<Item=Self::AEType>;
    type EIter: std::iter::Iterator<Item=Self::AEType>;
    type VIter: std::iter::Iterator<Item=&'a Self::VType>;
    fn delta(&'a self, v: &Self::VType) -> Self::AdjIter;
    fn edges(&'a self) -> Self::EIter;
    fn vertices(&'a self) -> Self::VIter;
    fn v_size(&self) -> usize;
    fn e_size(&self) -> usize;
}

pub trait Directed<'a>: Graph<'a> {}
pub trait Undirected<'a>: Graph<'a> {}
pub trait Bipartite<'a>: Undirected<'a> {
    type BVIter: std::iter::Iterator<Item=&'a Self::VType>;
    fn left_vertices(&'a self) -> Self::BVIter;
    fn right_vertices(&'a self) -> Self::BVIter;
}
pub trait Residual<'a>: Directed<'a> where <Self as Graph<'a>>::AEType: ResidualEdge {}
