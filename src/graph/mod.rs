pub mod property;
pub mod directed_graph;
pub mod undirected_graph;
pub mod dynamic_directed_graph;
pub mod dynamic_undirected_graph;
pub mod bipartite_graph;
pub mod matching;
pub mod shortest_path;
pub mod network;

use graph::property::*;

/// Vertex object for graphs. it has the index of the vertex.
#[derive(Clone,Copy,Eq,PartialEq,Debug)]
pub struct Vertex(pub usize);

/// Edge object for graphs.
#[derive(Clone,Copy,Debug)]
pub struct Edge {
    /// index of the edge for edge property.
    pub index : usize,
    /// start vertex of the edge.
    pub from : Vertex,
    /// end vertex of the edge.
    pub to : Vertex
}


impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for Edge {}

pub trait Graph<'a,VP: Property,EP: Property> {
    type EIter: std::iter::Iterator<Item=&'a Edge>;
    
    /// this method return the count of vertices of graph.
    fn vertices_cnt(&self) -> usize;
    /// this method return the count of edges of graph.
    fn edges_cnt(&self) -> usize;
    /// this method add new edge to graph.
    fn add_edge(&mut self , from : &Vertex , to : &Vertex , edge_prop : EP);
    /// this method return mutable reference of v's property.
    fn vprop_mut(&mut self, v : &Vertex) -> &mut VP;
    /// this method return reference of v's property.
    fn vprop(&self, v : &Vertex) -> &VP;
    /// this method return mutable reference of e's property.
    fn eprop_mut(&mut self, e : &Edge) -> &mut EP;
    /// this method return reference of e's property.
    fn eprop(&self, e : &Edge) -> &EP;

    fn delta(&'a self , v : &Vertex) -> Self::EIter;
}

pub trait StaticGraph<'a,VP: Property, EP: Property>: Graph<'a,VP,EP> {
    /// this method create new graph object.
    /// n ... count of vertices.
    /// vp_init ... initial property of vertices.
    fn new(n : usize , vp_init : VP) -> Self;
}
pub trait DynamicGraph<'a,VP: Property, EP: Property>: Graph<'a,VP,EP> {
    fn new() -> Self;
    fn add_vertex(&'a mut self, v: &Vertex,vp: VP) -> bool;
    fn erase_vertex(&'a mut self,v: &Vertex) -> bool;
    fn erase_edge(&'a mut self, e: &Edge) -> bool;
}

pub trait Directed<'a,VP: Property,EP: Property>: Graph<'a,VP,EP>{
}
pub trait Undirected<'a,VP: Property,EP: Property>: Graph<'a,VP,EP>{
}

pub trait Bipartite<'a,VP: Property,EP: Property>: Undirected<'a,VP,EP> {
    fn binew(left: usize, right: usize, vp_init: VP) -> Self;
    fn left_cnt(&self) -> usize;
    fn right_cnt(&self)-> usize; 
    fn left_vertices(&self) -> std::slice::Iter<Vertex>;
    fn right_vertices(&self) -> std::slice::Iter<Vertex>;
}
