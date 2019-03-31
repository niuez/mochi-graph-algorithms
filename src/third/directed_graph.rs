use third::*;

#[derive(Clone,Copy,Eq,PartialEq,Debug)]
pub struct Eite(pub usize);

pub struct IDiEdge<'a, E: Edge + 'a>(&'a E, usize);

impl<'a, E: Edge + 'a> ID for IDiEdge<'a, E> {
    fn id(&self) -> usize { self.1 }
}

impl<'a, V, E> IEdge<V, E> for IDiEdge<'a, E> where V: Vertex, E: Edge<VType=V> + 'a {
    fn from(&self) -> &E::VType { self.0.from() }
    fn to(&self) -> &E::VType { self.0.to() }
    fn edge(&self) -> &E { self.0 }
}

pub struct AdjIter<'a, E: Edge + 'a> {
    iter: std::slice::Iter<'a, Eite>,
    edges: &'a Vec<E>,
}

impl<'a, E: Edge + 'a> std::iter::Iterator for AdjIter<'a, E> {
    type Item = IDiEdge<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(ei) => {
                Some( IDiEdge(&self.edges[ei.0], ei.0) )
            }
            None => {
                None
            }
        }
    }
}

pub struct EIter<'a, E: Edge + 'a> {
    i: usize,
    iter: std::slice::Iter<'a, E>,
}

impl<'a, E: Edge + 'a> std::iter::Iterator for EIter<'a, E> {
    type Item = IDiEdge<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(e) => {
                let i = self.i;
                self.i += 1;
                Some(IDiEdge(&e, i))
            }
            None => None
        }
    }
}

pub struct DirectedGraph<V: Vertex, E: Edge<VType=V>> {
    n: usize,
    m: usize,
    g: Vec<Vec<Eite>>,
    es: Vec<E>,
}

impl<'a, V, E> Graph<'a,V,E,IDiEdge<'a, E>> for DirectedGraph<V,E> where V: Vertex, E: Edge<VType=V> + 'a {
    type AdjIter = AdjIter<'a, E>;
    type EIter = EIter<'a, E>;
    fn add_edge(&mut self, e: E) {
        let ei = Eite(self.m);
        self.m += 1;
        self.g[e.from().id()].push(ei);
        self.es.push(e);
    }
    fn delta(&'a self, v: &V) -> Self::AdjIter {
        AdjIter { iter: self.g[v.id()].iter(), edges: &self.es }
    }
    fn edges(&'a self) -> Self::EIter {
        EIter { i: 0, iter: self.es.iter() }
    }
    fn v_size(&self) -> usize {
        self.n
    }
    fn e_size(&self) -> usize {
        self.m
    }
}

impl<V: Vertex, E: Edge<VType=V>> DirectedGraph<V,E> {
    pub fn new(n: usize) -> Self {
        DirectedGraph {
            n: n,
            m: 0,
            g: vec![Vec::<Eite>::new(); n],
            es: Vec::new(),
        }
    }
}

impl<'a, V, E> Directed<'a, V, E, IDiEdge<'a, E>> for DirectedGraph<V, E> where V: Vertex, E: Edge<VType=V> + 'a {}

#[test]
fn digraph_test() {
    let mut g = DirectedGraph::new(4);
    g.add_edge((0, 1));
    g.add_edge((1, 2));
    g.add_edge((2, 3));
    for e in g.delta(&0) {
        assert!(e.to() == &1);
    }
    for e in g.delta(&1) {
        assert!(e.to() == &2);
    }
    for e in g.delta(&2) {
        assert!(e.to() == &3);
    }
    for e in g.delta(&0) {
        for ee in g.delta(e.to()) {
            assert!(ee.to() == &2)
        }
    }
}


