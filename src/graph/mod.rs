pub mod property;
pub mod directed_graph;
pub mod undirected_graph;
pub mod dynamic_directed_graph;
pub mod dynamic_undirected_graph;
pub mod matching;
pub mod shortest_path;

use std::slice::Iter;
use std::collections::btree_map::*;

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

pub trait Graph {
    type VP;
    type EP;
    
    /// this method return the count of vertices of graph.
    fn vertices_cnt(&self) -> usize;
    /// this method return the count of edges of graph.
    fn edges_cnt(&self) -> usize;
    /// this method add new edge to graph.
    fn add_edge(&mut self , from : &Vertex , to : &Vertex , edge_prop : Self::EP);
    /// this method return mutable reference of v's property.
    fn vprop_mut(&mut self, v : &Vertex) -> &mut Self::VP;
    /// this method return reference of v's property.
    fn vprop(&self, v : &Vertex) -> &Self::VP;
    /// this method return mutable reference of e's property.
    fn eprop_mut(&mut self, e : &Edge) -> &mut Self::EP;
    /// this method return reference of e's property.
    fn eprop(&self, e : &Edge) -> &Self::EP;
}

pub trait StaticGraph: Graph {
    /// this method create new graph object.
    /// n ... count of vertices.
    /// vp_init ... initial property of vertices.
    fn new(n : usize , vp_init : Self::VP) -> Self;
    fn delta(&self , v : &Vertex) -> Iter<Edge>;
}
pub trait DynamicGraph: Graph {
    fn delta(&self , v : &Vertex) -> Values<usize,Edge>;
}
