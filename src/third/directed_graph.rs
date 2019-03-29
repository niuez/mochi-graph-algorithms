use third::*;

pub struct DirectedGraph<V: Vertex, E: Edge<VType=V>> {
    n: usize,
    m: usize,
    g: Vec<Vec<Eite>>,
    es: Vec<IEdge<E>>,
}

impl<'a, V, E> Graph<'a,V,E> for DirectedGraph<V,E> where V: Vertex, E: Edge<VType=V> + 'a {
    type EIter = AdjIter<'a, E>;
    fn add_edge(&mut self, e: E) {
        let ei = Eite(self.m);
        self.m += 1;
        self.g[e.from().id()].push(ei);
        self.es.push(IEdge(e, ei.0));
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


