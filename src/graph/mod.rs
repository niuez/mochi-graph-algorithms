pub mod property;
mod undirected_graph;
mod directed_graph;

pub mod shortest_path;

use graph::property::*;

use std::slice::Iter;

/// Vertex object for graphs. it has the index of the vertex.
#[derive(Clone,Eq,PartialEq,Debug)]
pub struct Vertex(pub usize);

/// Edge object for graphs.
#[derive(Clone,Debug)]
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
    /// this method create new graph object.
    /// n ... count of vertices.
    /// vp_init ... initial property of vertices.
    fn new(n : usize , vp_init : Self::VP) -> Self;
    /// this method return the count of vertices of graph.
    fn vertices_cnt(&self) -> usize;
    /// this method return the count of edges of graph.
    fn edges_cnt(&self) -> usize;
    /// this method add new edge to graph.
    fn add_edge(&mut self , from : &Vertex , to : &Vertex , edge_prop : Self::EP);
    /// this method return the edges whose start vertex is v.
    fn delta(&self , v : &Vertex) -> Iter<Edge>;
    /// this method return mutable reference of v's property.
    fn vprop_mut(&mut self, v : &Vertex) -> &mut Self::VP;
    /// this method return reference of v's property.
    fn vprop(&self, v : &Vertex) -> &Self::VP;
    /// this method return mutable reference of e's property.
    fn eprop_mut(&mut self, e : &Edge) -> &mut Self::EP;
    /// this method return reference of e's property.
    fn eprop(&self, e : &Edge) -> &Self::EP;
}

/// struct for Directed Graph.
pub struct DirectedGraph<VP: Property,EP: Property> {
    n : usize,
    m : usize,
    g : Vec<Vec<Edge>>,
    es : Vec<EP>,
    vs : Vec<VP>
}

///struct for UndirectedGraph.
pub struct UndirectedGraph<VP: Property,EP: Property> {
    n : usize,
    m : usize,
    g : Vec<Vec<Edge>>,
    es : Vec<EP>,
    vs : Vec<VP>
}
