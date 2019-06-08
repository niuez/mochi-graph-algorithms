use graph::kernel::graph::*;

#[derive(Clone,Copy,Eq,PartialEq,Debug)]
pub struct REite(usize, bool);

pub struct RAdjEdge<'a, E: Edge + 'a>(&'a E, usize, bool);

impl<'a, E: Edge + 'a> Clone for RAdjEdge<'a, E> {
    fn clone(&self) -> Self { Self(self.0, self.1, self.2) }
}

impl<'a, E: Edge + 'a> Copy for RAdjEdge<'a, E> {}

impl<'a, E: Edge + 'a> ID for RAdjEdge<'a, E> {
    fn id(&self) -> usize { self.1 * 2 + match self.2 { true => 0, false => 1 } } 
}

impl<'a, E> Edge for RAdjEdge<'a, E> where E: Edge + 'a {
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

impl<'a, E> AdjEdge for RAdjEdge<'a, E> where E: Edge + 'a {
    type EType = E;
    fn edge(&self) -> &E { self.0 }
}

impl<'a, E> ResidualEdge for RAdjEdge<'a, E> where E: Edge + 'a {
    fn rev(&self) -> Self {
        RAdjEdge(self.0, self.1, self.2 ^ true)
    }
}

pub struct AdjIter<'a, E: Edge + 'a> {
    iter: std::slice::Iter<'a, REite>,
    edges: &'a Vec<E>,
}

impl<'a, E: Edge + 'a> std::iter::Iterator for AdjIter<'a, E> {
    type Item = RAdjEdge<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(REite(ei, dir)) => Some(RAdjEdge(&self.edges[*ei], *ei, *dir)),
            None => None,
        }
    }
}

pub struct EIter<'a, E: Edge + 'a> {
    i: usize,
    iter: std::slice::Iter<'a, E>,
}

impl<'a, E: Edge + 'a> std::iter::Iterator for EIter<'a, E> {
    type Item = RAdjEdge<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(e) => {
                let i = self.i;
                self.i += 1;
                Some(RAdjEdge(&e, i, true))
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
pub struct ResidualNetwork<V: Vertex, E: Edge<VType=V>> {
    n: usize,
    m: usize,
    g: Vec<Vec<REite>>,
    es: Vec<E>,
    vs: Vec<Option<V>>, 
}

impl<'a, V, E> Graph<'a> for ResidualNetwork<V,E> where V: Vertex + 'a, E: Edge<VType=V> + 'a {
    type VType = V;
    type EType = E;
    type AEType = RAdjEdge<'a, E>;
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
        self.m * 2
    }
}

impl<V: Vertex, E: Edge<VType=V>> ResidualNetwork<V,E> {
    pub fn new(n: usize) -> Self {
        ResidualNetwork {
            n: n,
            m: 0,
            g: vec![Vec::<REite>::new(); n],
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
        self.g[e.from().id()].push(REite(ei, true));
        self.g[e.to().id()].push(REite(ei, false));
        self.vertex_regist(e.from().clone());
        self.vertex_regist(e.to().clone());
        self.es.push(e);
    }
}

impl<'a, V, E> Directed<'a> for ResidualNetwork<V, E> where V: Vertex + 'a, E: Edge<VType=V> + 'a {}
impl<'a, V, E> Residual<'a> for ResidualNetwork<V, E> where V: Vertex + 'a, E: Edge<VType=V> + 'a {}

#[test] 

fn residual_network_test() {
    let mut net = ResidualNetwork::new(3);
    net.add_edge((0, 1, 0));
    net.add_edge((0, 2, 1));
    net.add_edge((1, 2, 2));
    {
        let mut d = net.delta(&0);
        if let Some(e) = d.next() {
            assert!(*e.from() == 0 && *e.to() == 1);
        }
        if let Some(e) = d.next() {
            assert!(*e.from() == 0 && *e.to() == 2);
        }
    }
    {
        let mut d = net.delta(&1);
        if let Some(e) = d.next() {
            assert!(*e.from() == 1 && *e.to() == 0);
        }
        if let Some(e) = d.next() {
            assert!(*e.from() == 1 && *e.to() == 2);
        }
    }
    {
        let mut d = net.delta(&2);
        if let Some(e) = d.next() {
            assert!(*e.from() == 2 && *e.to() == 0);
        }
        if let Some(e) = d.next() {
            assert!(*e.from() == 2 && *e.to() == 1);
        }
    }
}
