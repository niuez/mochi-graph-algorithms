pub mod property;
pub mod matching;

pub mod shortest_path;

use graph::property::*;

use std::slice::Iter;

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

impl<VP : Property ,EP : Property> Graph for DirectedGraph<VP,EP> {
    type VP = VP;
    type EP = EP;
    fn new(n : usize , vp_init: VP) -> Self {
        DirectedGraph {
            n: n,
            m: 0,
            g: vec![Vec::<Edge>::new(); n],
            es: Vec::<EP>::new(),
            vs: vec![vp_init; n]
        }
    }
    fn add_edge(&mut self , from : &Vertex , to : &Vertex , edge_prop : Self::EP) {
        self.g[from.0].push(Edge{index : self.m, from : from.clone() , to : to.clone()});
        self.es.push(edge_prop);
        self.m += 1;
    }
    fn vertices_cnt(&self) -> usize { self.n }
    fn edges_cnt(&self) -> usize { self.m }
    fn delta(&self , v : &Vertex) -> Iter<Edge> {
        self.g[v.0].iter()
    }
    fn vprop_mut(&mut self, v : &Vertex) -> &mut Self::VP {
        &mut self.vs[v.0]
    }
    fn vprop(&self, v : &Vertex) -> &Self::VP {
        & self.vs[v.0]
    }
    fn eprop_mut(&mut self, e : &Edge) -> &mut Self::EP {
        &mut self.es[e.index]
    }
    fn eprop(&self, e : &Edge) -> &Self::EP {
        & self.es[e.index]
    }
}

impl<VP : Property ,EP : Property> Graph for UndirectedGraph<VP,EP> {
    type VP = VP;
    type EP = EP;
    fn new(n : usize , vp_init: VP) -> Self {
        UndirectedGraph {
            n: n,
            m: 0,
            g: vec![Vec::<Edge>::new(); n],
            es: Vec::<EP>::new(),
            vs: vec![vp_init; n]
        }
    }
    fn add_edge(&mut self , from : &Vertex , to : &Vertex , edge_prop : Self::EP) {
        self.g[from.0].push(Edge{index : self.m , from : from.clone() , to : to.clone()});
        self.g[to.0].push(Edge{index : self.m, from : to.clone(), to : from.clone()});
        self.es.push(edge_prop);
        self.m += 1;
    }
    fn vertices_cnt(&self) -> usize { self.n }
    fn edges_cnt(&self) -> usize { self.m }
    fn delta(&self , v : &Vertex) -> Iter<Edge> {
        self.g[v.0].iter()
    }
    fn vprop_mut(&mut self, v : &Vertex) -> &mut Self::VP {
        &mut self.vs[v.0]
    }
    fn vprop(&self, v : &Vertex) -> &Self::VP {
        & self.vs[v.0]
    }
    fn eprop_mut(&mut self, e : &Edge) -> &mut Self::EP {
        &mut self.es[e.index]
    }
    fn eprop(&self, e : &Edge) -> &Self::EP {
        & self.es[e.index]
    }
}

