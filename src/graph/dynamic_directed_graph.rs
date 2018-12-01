use graph::*;
use graph::property::Property;

use std::collections::btree_map::*;

/// struct for Directed Graph.
pub struct DynamicDirectedGraph<VP: Property,EP: Property> {
    m : usize,
    g : BTreeMap<usize,BTreeMap<usize,Edge>>,
    es : BTreeMap<usize,EP>,
    vs : BTreeMap<usize,VP>,
    rev_e: BTreeMap<usize,Vec<Edge>>
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
        match self.rev_e.get_mut(&to.0) {
            Some(arr) => {
                arr.push(Edge{index: self.m, from: from.clone(), to: to.clone()});
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

impl<'a,VP : Property, EP: Property> DynamicGraph<'a,VP,EP> for DynamicDirectedGraph<VP,EP> {
    fn new() -> Self {
        DynamicDirectedGraph {
            m: 0,
            g: BTreeMap::<usize,BTreeMap<usize,Edge>>::new(),
            es: BTreeMap::<usize,EP>::new(),
            vs: BTreeMap::<usize,VP>::new(),
            rev_e: BTreeMap::<usize,Vec<Edge>>::new()
        }
    }
    fn add_vertex(&'a mut self,v: &Vertex,vp: VP) -> bool {
        if self.vs.contains_key(&v.0) {
            false
        }
        else {
            self.vs.insert(v.0,vp);
            self.g.insert(v.0,BTreeMap::<usize,Edge>::new());
            self.rev_e.insert(v.0,Vec::<Edge>::new());
            true
        }
    }
    fn erase_vertex(&'a mut self, v: &Vertex) -> bool {
        if self.vs.contains_key(&v.0) {
            self.vs.remove(&v.0);
            // TODO: erase connected edges
            for e in self.rev_e.get(&v.0).unwrap() {
                self.g.get_mut(&e.from.0).unwrap().remove(&e.index);
                self.es.remove(&e.index);
            }
            self.rev_e.remove(&v.0);
            self.g.remove(&v.0);
            true
        }
        else {
            false
        }
    }
    fn erase_edge(&'a mut self, e: &Edge) -> bool { 
        if self.vs.contains_key(&e.index) {
            self.g.get_mut(&e.from.0).unwrap().remove(&e.index);
            self.es.remove(&e.index);
            true
        }
        else {
            false
        }
    }
}

impl<'a,VP : Property, EP: Property> Directed<'a,VP,EP> for DynamicDirectedGraph<VP,EP> {
}
