use third::*;

#[derive(Clone,Copy,Eq,PartialEq,Debug)]
pub struct UEite(usize, bool);

pub struct IUdEdge<'a, E: Edge + 'a>(&'a E, usize, bool);

impl<'a, E: Edge + 'a> ID for IUdEdge<'a, E> {
    fn id(&self) -> usize { self.1 } 
}

impl<'a, V, E> IEdge<V, E> for IUdEdge<'a, E> 
where V: Vertex, E: Edge<VType=V> + 'a {
    fn from(&self) -> &E::VType { 
        match self.2 {
            true => self.0.from(),
            false => self.0.to(),
        }
    }
    fn to(&self) -> &E::VType {
        match self.2 {
            true => self.0.to(),
            false => self.0.from(),
        }
    }
    fn edge(&self) -> &E { self.0 }
}

pub struct AdjIter<'a, E: Edge + 'a> {
    iter: std::slice::Iter<'a, UEite>,
    edges: &'a Vec<E>,
}

impl<'a, E: Edge + 'a> std::iter::Iterator for AdjIter<'a, E> {
    type Item = IUdEdge<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(UEite(ei, dir)) => Some(IUdEdge(&self.edges[*ei], *ei, *dir)),
            None => None,
        }
    }
}

pub struct UndirectedGraph<V: Vertex, E: Edge<VType=V>> {
    n: usize,
    m: usize,
    g: Vec<Vec<UEite>>,
    es: Vec<E>,
}

impl<'a, V, E> Graph<'a,V,E,IUdEdge<'a, E>> for UndirectedGraph<V,E> where V: Vertex, E: Edge<VType=V> + 'a {
    type EIter = AdjIter<'a, E>;
    fn add_edge(&mut self, e: E) {
        let ei = self.m;
        self.m += 1;
        self.g[e.from().id()].push(UEite(ei, true));
        self.g[e.to().id()].push(UEite(ei, false));
        self.es.push(e);
    }
    fn delta(&'a self, v: &V) -> Self::EIter {
        AdjIter { iter: self.g[v.id()].iter(), edges: &self.es }
    }
    fn v_size(&self) -> usize {
        self.n
    }
    fn e_size(&self) -> usize {
        self.m
    }
}

impl<V: Vertex, E: Edge<VType=V>> UndirectedGraph<V,E> {
    pub fn new(n: usize) -> Self {
        UndirectedGraph {
            n: n,
            m: 0,
            g: vec![Vec::<UEite>::new(); n],
            es: Vec::new(),
        }
    }
}

#[test]
fn undigraph_test() {
    let mut g = UndirectedGraph::new(4);
    g.add_edge((0, 1));
    g.add_edge((2, 3));
    for e in g.delta(&0) {
        assert!(e.to() == &1);
    }
    for e in g.delta(&1) {
        assert!(e.to() == &0);
    }
    for e in g.delta(&2) {
        assert!(e.to() == &3);
    }
}
