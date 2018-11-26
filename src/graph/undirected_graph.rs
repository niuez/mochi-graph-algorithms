use graph::*;
use graph::property::*;
use std::slice::Iter;

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
        self.g[from.0].push(Edge{index : self.m , to : to.clone()});
        self.g[to.0].push(Edge{index : self.m, to : from.clone()});
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


