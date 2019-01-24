pub mod property;
pub mod directed_graph;
pub mod undirected_graph;
pub mod bipartite_directed_graph;
pub mod bipartite_undirected_graph;

pub mod single_source_shortest_path;
pub mod maxflow;
pub mod cardinality_bipartite_maching;
pub mod cardinality_nonbipartite_matching;

use graph::property::*;

#[derive(Clone,Copy,Eq,PartialEq,Debug)]
pub struct Vite(pub usize);

#[derive(Clone,Copy,Eq,PartialEq,Debug)]
pub struct Eite(pub usize);

pub trait Edge {
    fn from(&self) -> Vite;
    fn to(&self) -> Vite;
}

pub trait Vertex {
    fn new(id : usize) -> Self;
    fn id(&self) -> usize;
}

impl Vertex for usize {
    fn new(id: usize) -> Self { id }
    fn id(&self) -> usize { *self }
}

impl Edge for (usize,usize) { 
    fn from(&self) -> Vite { Vite(self.0) }
    fn to(&self) -> Vite { Vite(self.1) }
}

impl<P> Edge for (usize,usize,P) where P: Property { 
    fn from(&self) -> Vite { Vite(self.0) }
    fn to(&self) -> Vite { Vite(self.1) }
}

pub trait Graph<'a, V: Vertex, E: Edge> {
    type EsIter: std::iter::Iterator<Item=&'a Eite>;
    fn add_edge(&mut self, e: E);
    fn delta(&'a self, v: &Vite) -> Self::EsIter;
    fn edge(&self, e: &Eite) -> &E;
    fn vertex(&self, v: &Vite) -> &V;
    fn v_size(&self) -> usize;
    fn e_size(&self) -> usize;
}

pub fn from<E: Edge>(f: Vite, e: &E) -> Vite {
    if e.from() == f { e.from() }
    else { e.to() }
}

pub fn to<E: Edge>(f: Vite, e: &E) -> Vite {
    if e.from() == f { e.to() }
    else { e.from() }
}

pub trait Directed<'a,V: Vertex, E: Edge>: Graph<'a,V,E> {  }
pub trait Undirected<'a,V: Vertex, E: Edge>: Graph<'a,V,E> {  }
pub trait Bipartite<'a,V: Vertex, E: Edge>: Graph<'a,V,E> { 
    fn left_size(&self) -> usize;
    fn right_size(&self) -> usize;
    fn left_vs(&self) -> std::slice::Iter<Vite>;
    fn right_vs(&self) -> std::slice::Iter<Vite>;
}

