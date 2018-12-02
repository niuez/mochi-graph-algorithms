use graph::*;
use graph::property::*;

pub struct BipartiteGraph<VP: Property,EP: Property> {
    left: usize,
    right: usize,
    m: usize,
    g: Vec<Vec<Edge>>,
    es: Vec<EP>,
    vs: Vec<VP>
}

impl<'a,VP: Property,EP: Property> Graph<'a,VP,EP> for BipartiteGraph<VP,EP> {
    type EIter = std::slice::Iter<'a,Edge>;

    fn add_edge(&mut self , l : &Vertex , r : &Vertex , edge_prop : EP) {
        assert!(l.0 < self.left , "it is not left vertex");
        assert!(self.left <= r.0 && r.0 < self.right, "it is not right vertex");
        self.g[l.0].push(Edge{index : self.m, from : l.clone() , to : r.clone()});
        self.g[r.0].push(Edge{index : self.m, from : r.clone() , to : l.clone()});
        self.es.push(edge_prop);
        self.m += 1;
    }

    fn vertices_cnt(&self) -> usize { self.left + self.right }
    fn edges_cnt(&self) -> usize { self.m }
    fn vprop_mut(&mut self, v : &Vertex) -> &mut VP {
        &mut self.vs[v.0]
    }
    fn vprop(&self, v : &Vertex) -> &VP {
        & self.vs[v.0]
    }
    fn eprop_mut(&mut self, e : &Edge) -> &mut EP {
        &mut self.es[e.index]
    }
    fn eprop(&self, e : &Edge) -> &EP {
        & self.es[e.index]
    }

    fn delta(&'a self , v : &Vertex) -> Self::EIter {
        self.g[v.0].iter()
    }
}

impl<'a,VP : Property ,EP : Property> StaticGraph<'a,VP,EP> for BipartiteGraph<VP,EP> {
    fn new(n : usize , vp_init: VP) -> Self {
        BipartiteGraph {
            left: n,
            right: n,
            m: 0,
            g: vec![Vec::<Edge>::new(); n + n],
            es: Vec::<EP>::new(),
            vs: vec![vp_init; n + n]
        }
    }
}

impl<'a,VP: Property, EP: Property> Undirected<'a,VP,EP> for BipartiteGraph<VP,EP> {
}
impl<'a,VP: Property, EP: Property> Bipartite<'a,VP,EP> for BipartiteGraph<VP,EP> {
    fn new(left: usize, right: usize, vp_init: VP) -> Self {
        assert!(left <= right, "it is not graph");
        BipartiteGraph {
            left: left,
            right: right,
            m: 0,
            g: vec![Vec::<Edge>::new(); right],
            es: Vec::<EP>::new(),
            vs: vec![vp_init; right]
        }
    }
}

