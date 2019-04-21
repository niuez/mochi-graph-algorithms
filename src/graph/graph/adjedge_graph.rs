use graph::kernel::graph::*;

pub struct AEdge<'a, G>(G::AEType) 
where G: Graph<'a> + 'a;

impl<'a, G> ID for AEdge<'a, G>
where G: Graph<'a> + 'a {
    fn id(&self) -> usize { self.0.id() }
}

impl<'a, G> Edge for AEdge<'a, G>
where G: Graph<'a> + 'a {
    type VType = <G::AEType as Edge>::VType;
    fn from(&self) -> &Self::VType { self.0.from() }
    fn to(&self) -> &Self::VType { self.0.to() }
}

impl<'a, G> AdjEdge for AEdge<'a, G>
where G: Graph<'a> + 'a {
    type EType = G::AEType;
    fn edge(&self) -> &Self::EType { &self.0 }
}

pub struct AdjIter<'a, G>
where G: Graph<'a> + 'a {
    iter: G::AdjIter,
}

impl<'a, G> std::iter::Iterator for AdjIter<'a, G>
where G: Graph<'a> + 'a {
    type Item = AEdge<'a, G>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(item) => Some(AEdge(item)),
            None => None,
        }
    }
}

pub struct EIter<'a, G>
where G: Graph<'a> + 'a {
    iter: G::EIter,
}

impl<'a, G> std::iter::Iterator for EIter<'a, G>
where G: Graph<'a> + 'a {
    type Item = AEdge<'a, G>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(item) => Some(AEdge(item)),
            None => None,
        }
    }
}

pub struct AdjEdgeGraph<'a, G>
where G: Graph<'a> + 'a {
    g: &'a G,
}

impl<'a, G> AdjEdgeGraph<'a, G>
where G: Graph<'a> + 'a {
    pub fn new(g: &'a G) -> Self {
        AdjEdgeGraph {
            g: g,
        }
    }
}

impl<'a, G> Graph<'a> for AdjEdgeGraph<'a, G>
where G: Graph<'a> + 'a {
    type VType = G::VType;
    type EType = G::AEType;
    type AEType = AEdge<'a, G>;
    type AdjIter = AdjIter<'a, G>;
    type EIter = EIter<'a, G>;
    type VIter = G::VIter;
    fn delta(&'a self, v: &Self::VType) -> Self::AdjIter {
        AdjIter { iter: self.g.delta(v) }
    }
    fn edges(&'a self) -> Self::EIter {
        EIter { iter: self.g.edges() }
    }
    fn vertices(&'a self) -> Self::VIter {
        self.g.vertices()
    }
    fn e_size(&self) -> usize {
        self.g.e_size()
    }
    fn v_size(&self) -> usize {
        self.g.v_size()
    }
}

impl<'a, G> Directed<'a> for AdjEdgeGraph<'a, G>
where G: Directed<'a> + 'a {}
impl<'a, G> Undirected<'a> for AdjEdgeGraph<'a, G>
where G: Undirected<'a> + 'a {}
impl<'a, G> Bipartite<'a> for AdjEdgeGraph<'a, G>
where G: Bipartite<'a> + 'a {
    type BVIter = G::BVIter;
    fn left_vertices(&'a self) -> Self::BVIter {
        self.g.left_vertices()
    }
    fn right_vertices(&'a self) -> Self::BVIter {
        self.g.left_vertices()
    }
}

#[test]
fn adjedge_graph_test() {
    use graph::kernel::Properties;
    use graph::graph::DirectedGraph;
    use graph::graph::SubEdgeGraph;
    use graph::property::NNegW;
    use graph::algorithm::single_source_shortest_path::bfs;
    let mut g = DirectedGraph::new(6);
    g.add_edge((0, 1, true));
    g.add_edge((1, 3, true));
    g.add_edge((3, 5, true));
    g.add_edge((0, 2, true));
    g.add_edge((2, 3, true));
    g.add_edge((4, 5, true));

    g.add_edge((1, 5, false));
    g.add_edge((0, 4, false));
    g.add_edge((2, 4, false));

    let mut ok = Properties::new(g.e_size(), &false);
    for ref e in g.edges() {
        ok[e] = e.edge().2;
    }

    let ag = AdjEdgeGraph::new(&g);
    let sg = SubEdgeGraph::new(&ag, |e| ok[e]);
    let dist = bfs(&sg, &0);
    assert!(dist[&0] == NNegW::Some(0));
    assert!(dist[&1] == NNegW::Some(1));
    assert!(dist[&2] == NNegW::Some(1));
    assert!(dist[&3] == NNegW::Some(2));
    assert!(dist[&4] == NNegW::Inf);
    assert!(dist[&5] == NNegW::Some(3));
}

