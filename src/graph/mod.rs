pub mod property;
pub mod directed_graph;
pub mod undirected_graph;

pub mod single_source_shortest_path;
pub mod all_pairs_shortest_path;

use graph::property::*;

pub trait Vertex: ID + Clone { }

impl<V: ID + Clone> Vertex for V { }

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

pub trait AdjEdge {
    type VType: Vertex;
    type EType: Edge<VType=Self::VType>;
    fn from(&self) -> &Self::VType;
    fn to(&self) -> &Self::VType;
    fn edge(&self) -> &Self::EType;
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
