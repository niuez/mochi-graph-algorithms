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

impl<'a,VP : Property ,EP : Property> Graph<'a,VP,EP> for DynamicDirectedGraph<VP,EP> {
    type EIter = std::collections::btree_map::Values<'a,usize,Edge>;
    fn add_edge(&mut self , from : &Vertex , to : &Vertex , edge_prop : EP) {
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
        self.g[&v.0].values()
    }
}

impl<'a,VP : Property, EP: Property> DynamicGraph<'a,VP,EP> for DynamicDirectedGraph<VP,EP> {
}

impl<'a,VP : Property, EP: Property> Directed<'a,VP,EP> for DynamicDirectedGraph<VP,EP> {
}
