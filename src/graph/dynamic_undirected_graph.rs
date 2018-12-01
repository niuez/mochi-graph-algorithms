use graph::*;
use graph::property::Property;

use std::collections::btree_map::*;


///struct for UndirectedGraph.
pub struct DynamicUndirectedGraph<VP: Property,EP: Property> {
    n : usize,
    m : usize,
    g : BTreeMap<usize,BTreeMap<usize,Edge>>,
    es : BTreeMap<usize,EP>,
    vs : BTreeMap<usize,VP>
}

impl<'a,VP : Property ,EP : Property> Graph<'a,VP,EP> for DynamicUndirectedGraph<VP,EP> {
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
        match self.g.get_mut(&to.0) {
            Some(arr) => {
                arr.insert(self.m, Edge{index: self.m, from: to.clone(), to: from.clone()});
            },
            None => {
                assert!(false, "the vertex is unknown.");
            }
        }
        self.es.insert(self.m,edge_prop);
        self.m += 1;
    }
    fn vertices_cnt(&self) -> usize { self.vs.len() }
    fn edges_cnt(&self) -> usize { self.es.len() }
    fn vprop_mut(&mut self, v : &Vertex) -> &mut VP {
        self.vs.get_mut(&v.0).unwrap()
    }
    fn vprop(&self, v : &Vertex) -> &VP {
        self.vs.get(&v.0).unwrap()
    }
    fn eprop_mut(&mut self, e : &Edge) -> &mut EP {
        self.es.get_mut(&e.index).unwrap()
    }
    fn eprop(&self, e : &Edge) -> &EP {
        self.es.get(&e.index).unwrap()
    }
    fn delta(&'a self , v : &Vertex) -> Self::EIter {
        self.g[&v.0].values()
    }
}

impl<'a,VP: Property, EP: Property> DynamicGraph<'a,VP,EP> for DynamicUndirectedGraph<VP,EP> {
    fn new() -> Self {
        DynamicUndirectedGraph {
            n: 0,
            m: 0,
            g: BTreeMap::<usize,BTreeMap<usize,Edge>>::new(),
            es: BTreeMap::<usize,EP>::new(),
            vs: BTreeMap::<usize,VP>::new()
        }
    }
}
impl<'a,VP : Property, EP: Property> Undirected<'a,VP,EP> for DynamicUndirectedGraph<VP,EP> {
}
