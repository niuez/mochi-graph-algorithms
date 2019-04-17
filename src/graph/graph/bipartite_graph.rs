use graph::kernel::graph::*;

#[derive(Clone,Copy,Eq,PartialEq,Debug)]
pub struct UEite(usize, bool);

pub struct UnAdjEdge<'a, E: Edge + 'a>(&'a E, usize, bool);

impl<'a, E: Edge + 'a> ID for UnAdjEdge<'a, E> {
    fn id(&self) -> usize { self.1 } 
}

impl<'a, E> AdjEdge for UnAdjEdge<'a, E> where E: Edge + 'a {
    type VType = E::VType;
    type EType = E;
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
                Some(UnAdjEdge(&e, i, false))
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

pub struct AVIter<'a, V: Vertex + 'a> {
    liter: VIter<'a, V>,
    riter: VIter<'a, V>,
}

impl<'a, V: Vertex + 'a> std::iter::Iterator for AVIter<'a, V> {
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        match self.liter.next() {
            None => self.riter.next(),
            some => some,
        }
    }
}

pub struct BipartiteGraph<V: Vertex, E: Edge<VType=V>> {
    n: usize,
    m: usize,
    g: Vec<Vec<UEite>>,
    es: Vec<E>,
    lvs: Vec<Option<V>>, 
    rvs: Vec<Option<V>>, 
}

impl<'a, V, E> Graph<'a> for BipartiteGraph<V,E> where V: Vertex + 'a, E: Edge<VType=V> + 'a {
    type VType = V;
    type EType = E;
    type AEType = UnAdjEdge<'a, E>;
    type AdjIter = AdjIter<'a, E>;
    type EIter = EIter<'a, E>;
    type VIter = AVIter<'a, V>;
    fn delta(&'a self, v: &V) -> Self::AdjIter {
        AdjIter { iter: self.g[v.id()].iter(), edges: &self.es }
    }
    fn edges(&'a self) -> Self::EIter {
        EIter { i: 0, iter: self.es.iter() }
    }
    fn vertices(&'a self) -> Self::VIter {
        AVIter { liter: self.left_vertices(), riter: self.right_vertices() }
    }
    fn v_size(&self) -> usize {
        self.n
    }
    fn e_size(&self) -> usize {
        self.m
    }
}

impl<V: Vertex, E: Edge<VType=V>> BipartiteGraph<V,E> {
    pub fn new(n: usize) -> Self {
        BipartiteGraph {
            n: n,
            m: 0,
            g: vec![Vec::<UEite>::new(); n],
            es: Vec::new(),
            lvs: vec![None; n],
            rvs: vec![None; n],
        }
    }

    fn left_vertex_regist(&mut self, v: V) {
        let i = v.id();
        assert!(self.rvs[i].is_none());
        self.lvs[i] = match self.lvs[v.id()].take() {
            Some(vv) => {
                assert!(vv.id() == v.id());
                Some(vv)
            }
            None => {
                Some(v)
            }
        }
    }

    fn right_vertex_regist(&mut self, v: V) {
        let i = v.id();
        assert!(self.lvs[i].is_none());
        self.rvs[i] = match self.rvs[v.id()].take() {
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
        self.left_vertex_regist(e.from().clone());
        self.right_vertex_regist(e.to().clone());
        self.es.push(e);
    }
}

impl<'a, V, E> Undirected<'a> for BipartiteGraph<V, E> where V: Vertex + 'a, E: Edge<VType=V> + 'a {}

impl<'a, V, E> Bipartite<'a> for BipartiteGraph<V, E> where V: Vertex + 'a, E: Edge<VType=V> + 'a {
    type BVIter = VIter<'a, V>;
    fn left_vertices(&'a self) -> Self::BVIter {
        VIter { iter: self.lvs.iter() }
    }
    fn right_vertices(&'a self) -> Self::BVIter {
        VIter { iter: self.rvs.iter() }
    }
}
