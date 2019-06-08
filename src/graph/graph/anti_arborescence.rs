use graph::kernel::graph::*;

#[derive(Clone)]
pub struct Eite(usize);

pub struct ArboAdjEdge<'a, E: Edge + 'a>(&'a E, usize);

impl<'a, E: Edge + 'a> Clone for ArboAdjEdge<'a, E> {
    fn clone(&self) -> Self { Self(self.0, self.1) }
}

impl<'a, E: Edge + 'a> ID for ArboAdjEdge<'a, E> {
    fn id(&self) -> usize { self.1 }
}

impl<'a, E: Edge + 'a> Edge for ArboAdjEdge<'a, E> {
    type VType = E::VType;
    fn from(&self) -> &E::VType { self.0.from() }
    fn to(&self) -> &E::VType { self.0.to() }
}

impl<'a, E: Edge + 'a> AdjEdge for ArboAdjEdge<'a, E> {
    type EType = E;
    fn edge(&self) -> &E { self.0 }
}

pub struct AdjIter<'a, E: Edge + 'a>  {
    per: Option<(&'a E, usize)>,
}

impl<'a, E: Edge + 'a> std::iter::Iterator for AdjIter<'a, E> {
    type Item = ArboAdjEdge<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.per.take().map(|e| ArboAdjEdge(e.0, e.1))
    }
}

pub struct EIter<'a, E: Edge + 'a> {
    i: usize,
    iter: std::slice::Iter<'a, E>,
}

impl<'a, E: Edge + 'a> std::iter::Iterator for EIter<'a, E> {
    type Item = ArboAdjEdge<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| {
            let i = self.i;
            self.i += 1;
            ArboAdjEdge(e, i)
        })
    }
}

pub struct VIter<'a, V: Vertex + 'a> {
    iter: std::slice:: Iter<'a, Option<V>>,
}

impl<'a, V: Vertex + 'a> std::iter::Iterator for VIter<'a, V> {
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.iter.next() {
            if v.is_none() { continue; }
            else { return v.as_ref() }
        }
        None
    }
}

pub struct AntiArborescence<V: Vertex, E: Edge> {
    per: Vec<Option<Eite>>,
    vs: Vec<Option<V>>,
    es: Vec<E>,
    root_id: usize,
}

impl<'a, V, E> Graph<'a> for AntiArborescence<V, E>
where V: Vertex + 'a, E: Edge<VType=V> + 'a {
    type VType = V;
    type EType = E;
    type AEType = ArboAdjEdge<'a, E>;
    type AdjIter = AdjIter<'a, E>;
    type EIter = EIter<'a, E>;
    type VIter = VIter<'a, V>;
    fn delta(&'a self, v: &V) -> Self::AdjIter {
        AdjIter { per: self.per[v.id()].clone().map(|eite| (&self.es[eite.0], eite.0)) }
    }
    fn edges(&'a self) -> Self::EIter {
        EIter { i: 0, iter: self.es.iter() }
    }
    fn vertices(&'a self) -> Self::VIter { 
        VIter { iter: self.vs.iter() }
    }
    fn v_size(&self) -> usize {
        self.vs.len()
    }
    fn e_size(&self) -> usize {
        self.es.len()
    }
}

pub struct PathIter<'a, E>
where E: Edge + 'a {
    per: &'a Vec<Option<Eite>>,
    es: &'a Vec<E>,
    now: usize,
}

impl<'a, E: Edge + 'a> std::iter::Iterator for PathIter<'a, E> {
    type Item = ArboAdjEdge<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.per[self.now].clone().map(|eite| {
            self.now = self.es[eite.0].to().id();
            ArboAdjEdge(&self.es[eite.0], eite.0)
        })
    }
}

impl<V: Vertex, E: Edge> AntiArborescence<V, E> {
    pub fn new_root(n: usize, root: V) -> Self {
        let mut vs = vec![None; n];
        let id = root.id();
        vs[root.id()] = Some(root);
        AntiArborescence {
            per: vec![None; n],
            vs: vs,
            es: Vec::new(),
            root_id: id,
        }
    }
    pub fn add_vertex(&mut self, v: V, e: E) {
        assert!(self.vs[v.id()].is_none());
        assert!(e.from().id() == v.id());
        assert!(self.vs[e.to().id()].is_some());
        let i = self.es.len();
        self.per[v.id()] = Some(Eite(i));
        self.vs[v.id()] = Some(v);
        self.es.push(e);
    }
    pub fn root(&self) -> &V {
        self.vs[self.root_id].as_ref().unwrap()
    }
    pub fn root_path<'a>(&'a self, v: &'a V) -> PathIter<'a, E> {
        PathIter { per: &self.per, es: &self.es, now: v.id() }
    }
}

#[test]
fn anti_arborescence_test() {
    let mut tree = AntiArborescence::new_root(5, 0);
    tree.add_vertex(1, (1, 0));
    tree.add_vertex(2, (2, 1));
    tree.add_vertex(3, (3, 1));
    tree.add_vertex(4, (4, 3));
    let mut path = tree.root_path(&4);
    assert!(path.next().unwrap().to() == &3);
    assert!(path.next().unwrap().to() == &1);
    assert!(path.next().unwrap().to() == &0);
    assert!(path.next().is_none());
}
