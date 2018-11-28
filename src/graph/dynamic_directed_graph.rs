use graph::*;
use graph::property::Property;

use std::collections::btree_map::*;

/// struct for Directed Graph.
pub struct DynamicDirectedGraph<VP: Property,EP: Property> {
    n : usize,
    m : usize,
    g : BTreeMap<usize,BTreeMap<usize,Edge>>,
    es : Vec<EP>,
    vs : Vec<VP>
}

impl<VP : Property ,EP : Property> Graph for DynamicDirectedGraph<VP,EP> {
    type VP = VP;
    type EP = EP;
    fn add_edge(&mut self , from : &Vertex , to : &Vertex , edge_prop : Self::EP) {
        match self.g.get_mut(&from.0) {
            Some(arr) => {
                arr.insert(self.m, Edge{index: self.m, from: from.clone(), to: to.clone()});
            },
            None => {
                assert!(false, "the vertex is unknown.");
            }
        }
        self.es.push(edge_prop);
        self.m += 1;
    }
    fn vertices_cnt(&self) -> usize { self.n }
    fn edges_cnt(&self) -> usize { self.m }
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

impl<VP : Property, EP: Property> DynamicGraph for DynamicDirectedGraph<VP,EP> {
    fn delta(&self , v : &Vertex) -> Values<usize,Edge> {
        self.g[&v.0].values()
    }
}
