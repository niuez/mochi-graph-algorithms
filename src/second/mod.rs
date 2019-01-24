pub mod property;
pub mod directed_graph;
pub mod undirected_graph;
pub mod maxflow;

pub mod single_source_shortest_path;

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

