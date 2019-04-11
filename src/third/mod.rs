pub mod property;
pub mod directed_graph;
pub mod undirected_graph;

pub mod single_source_shortest_path;
pub mod all_pairs_shortest_path;

use third::property::*;


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

pub trait AdjEdge<V, E>: ID where V: Vertex, E: Edge<VType=V> {
    fn from(&self) -> &V;
    fn to(&self) -> &V;
    fn edge(&self) -> &E;
}

pub trait Graph<'a, V, E, AE> where V: Vertex + 'a, E: Edge<VType=V> + 'a, AE: AdjEdge<V, E> {
    type AdjIter: std::iter::Iterator<Item=AE>;
    type EIter: std::iter::Iterator<Item=AE>;
    type VIter: std::iter::Iterator<Item=&'a V>;
    fn delta(&'a self, v: &V) -> Self::AdjIter;
    fn edges(&'a self) -> Self::EIter;
    fn vertices(&'a self) -> Self::VIter;
    fn v_size(&self) -> usize;
    fn e_size(&self) -> usize;
}

pub trait Directed<'a, V, E, AE>: Graph<'a, V, E, AE> where V: Vertex + 'a, E: Edge<VType=V> + 'a, AE: AdjEdge<V, E> {}
pub trait Undirected<'a, V, E, AE>: Graph<'a, V, E, AE> where V: Vertex + 'a, E: Edge<VType=V> + 'a, AE: AdjEdge<V, E> {}
