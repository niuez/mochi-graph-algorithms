pub mod property;
mod undirected_graph;
mod directed_graph;

pub mod shortest_path;

use graph::property::*;

use std::slice::Iter;

#[derive(Clone,Eq,PartialEq,Debug)]
pub struct Vertex(pub usize);

#[derive(Clone,Debug)]
pub struct Edge {
    pub index : usize,
    pub from : Vertex,
    pub to : Vertex
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for Edge {}

pub trait Graph {
    type VP;
    type EP;
    fn new(n : usize , vp_init : Self::VP) -> Self;
    fn vertices_cnt(&self) -> usize;
    fn edges_cnt(&self) -> usize;
    fn add_edge(&mut self , from : &Vertex , to : &Vertex , edge_prop : Self::EP);
    fn delta(&self , v : &Vertex) -> Iter<Edge>;
    fn vprop_mut(&mut self, v : &Vertex) -> &mut Self::VP;
    fn vprop(&self, v : &Vertex) -> &Self::VP;
    fn eprop_mut(&mut self, e : &Edge) -> &mut Self::EP;
    fn eprop(&self, e : &Edge) -> &Self::EP;
}

pub struct DirectedGraph<VP: Property,EP: Property> {
    n : usize,
    m : usize,
    g : Vec<Vec<Edge>>,
    es : Vec<EP>,
    vs : Vec<VP>
}

pub struct UndirectedGraph<VP: Property,EP: Property> {
    n : usize,
    m : usize,
    g : Vec<Vec<Edge>>,
    es : Vec<EP>,
    vs : Vec<VP>
}
