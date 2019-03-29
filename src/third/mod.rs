pub mod property;
pub mod directed_graph;

pub mod single_source_shortest_path;

use third::property::*;

#[derive(Clone,Copy,Eq,PartialEq,Debug)]
pub struct Eite(pub usize);

pub trait Vertex: ID + Clone { }

impl<V: ID + Clone> Vertex for V { }

pub trait Edge {
    type VType: Vertex;
    fn from(&self) -> &Self::VType;
    fn to(&self) -> &Self::VType;
}

impl Edge for (usize,usize) { 
    type VType = usize;
    fn from(&self) -> &usize { &self.0 }
    fn to(&self) -> &usize { &self.1 }
}

impl<P> Edge for (usize,usize,P) where P: Property { 
    type VType = usize;
    fn from(&self) -> &usize { &self.0 }
    fn to(&self) -> &usize { &self.1 }
}

pub struct IEdge<E: Edge>(E, usize);

impl<E: Edge> ID for IEdge<E> {
    fn id(&self) -> usize { self.1 }
}

impl<E: Edge> IEdge<E> {
    pub fn from(&self) -> &E::VType { self.0.from() }
    pub fn to(&self) -> &E::VType { self.0.to() }
    pub fn edge(&self) -> &E { &self.0 }
}

pub struct AdjIter<'a, E: Edge + 'a> {
    iter: std::slice::Iter<'a, Eite>,
    edges: &'a Vec<IEdge<E>>,
}

impl<'a, E: Edge + 'a> std::iter::Iterator for AdjIter<'a, E> {
    type Item = &'a IEdge<E>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(ei) => {
                Some( &self.edges[ei.0] )
            }
            None => {
                None
            }
        }
    }
}

pub trait Graph<'a, V, E>: where V: Vertex, E: Edge<VType=V> + 'a {
    type EIter: std::iter::Iterator<Item=&'a IEdge<E>>;
    fn add_edge(&mut self, e: E);
    fn delta(&'a self, v: &V) -> Self::EIter;
    fn v_size(&self) -> usize;
    fn e_size(&self) -> usize;
}
