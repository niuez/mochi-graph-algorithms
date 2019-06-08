use graph::kernel::graph::*;

#[derive(Clone,Copy,Eq,PartialEq,Debug)]
pub struct UEite(usize, bool);

pub struct UnAdjEdge<'a, E: Edge + 'a>(&'a E, usize, bool);

impl<'a, E: Edge + 'a> Clone for UnAdjEdge<'a, E> {
    fn clone(&self) -> Self { Self(self.0, self.1, self.2) }
}

impl<'a, E: Edge + 'a> Copy for UnAdjEdge<'a, E> {}

impl<'a, E: Edge + 'a> ID for UnAdjEdge<'a, E> {
    fn id(&self) -> usize { self.1 } 
}

impl<'a, E> Edge for UnAdjEdge<'a, E> where E: Edge + 'a {
    type VType = E::VType;
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
}

impl<'a, E> AdjEdge for UnAdjEdge<'a, E> where E: Edge + 'a {
    type EType = E;
    fn edge(&self) -> &E { self.0 }
}

pub struct AdjIter<'a, E: Edge + 'a> {
    iter: std::slice::Iter<'a, UEite>,
    edges: &'a Vec<E>,
}

impl<'a, E: Edge + 'a> std::iter::Iterator for AdjIter<'a, E> {
    type Item = UnAdjEdge<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(UEite(ei, dir)) => Some(UnAdjEdge(&self.edges[*ei], *ei, *dir)),
            None => None,
        }
    }
}

pub struct EIter<'a, E: Edge + 'a> {
    i: usize,
    iter: std::slice::Iter<'a, E>,
}

impl<'a, E: Edge + 'a> std::iter::Iterator for EIter<'a, E> {
    type Item = UnAdjEdge<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(e) => {
                let i = self.i;
                self.i += 1;
                Some(UnAdjEdge(&e, i, true))
            }
            None => None
        }
    }
}

pub struct VIter<'a, V: Vertex + 'a> {
    iter: std::slice::Iter<'a, Option<V>>,
}

impl<'a, V: Vertex + 'a> std::iter::Iterator for VIter<'a, V> {
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.iter.next() {
            if v.is_none() { continue; }
            else { return v.as_ref(); }
        }
        None
    }
}
pub struct UndirectedGraph<V: Vertex, E: Edge<VType=V>> {
    n: usize,
    m: usize,
    g: Vec<Vec<UEite>>,
    es: Vec<E>,
    vs: Vec<Option<V>>, 
}

impl<'a, V, E> Graph<'a> for UndirectedGraph<V,E> where V: Vertex + 'a, E: Edge<VType=V> + 'a {
    type VType = V;
    type EType = E;
    type AEType = UnAdjEdge<'a, E>;
    type AdjIter = AdjIter<'a, E>;
    type EIter = EIter<'a, E>;
    type VIter = VIter<'a, V>;
    fn delta(&'a self, v: &V) -> Self::AdjIter {
        AdjIter { iter: self.g[v.id()].iter(), edges: &self.es }
    }
    fn edges(&'a self) -> Self::EIter {
        EIter { i: 0, iter: self.es.iter() }
    }
    fn vertices(&'a self) -> Self::VIter {
        VIter { iter: self.vs.iter() }
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
            vs: vec![None; n],
        }
    }

    fn vertex_regist(&mut self, v: V) {
        let i = v.id();
        self.vs[i] = match self.vs[v.id()].take() {
            Some(vv) => {
                assert!(vv.id() == v.id());
                Some(vv)
            }
            None => {
                Some(v)
            }
        }
    }

    pub fn add_edge(&mut self, e: E) {
        let ei = self.m;
        self.m += 1;
        self.g[e.from().id()].push(UEite(ei, true));
        self.g[e.to().id()].push(UEite(ei, false));
        self.vertex_regist(e.from().clone());
        self.vertex_regist(e.to().clone());
        self.es.push(e);
    }
}

impl<'a, V, E> Undirected<'a> for UndirectedGraph<V, E> where V: Vertex + 'a, E: Edge<VType=V> + 'a {}

#[test]
fn undigraph_test() {
    let mut g = UndirectedGraph::new(4);
    g.add_edge((0, 1));
    g.add_edge((2, 3));
    for ref e in g.delta(&0) {
        assert!(e.to() == &1);
    }
    for ref e in g.delta(&1) {
        assert!(e.to() == &0);
    }
    for ref e in g.delta(&2) {
        assert!(e.to() == &3);
    }
}
