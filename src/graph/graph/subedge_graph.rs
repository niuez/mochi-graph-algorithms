use graph::kernel::graph::*;

pub struct AdjIter<'a, G, F>
where G: Graph<'a> + 'a, F: Fn(&G::EType) -> bool + 'a {
    iter: G::AdjIter,
    cond: &'a F,
}

impl<'a, G, F> std::iter::Iterator for AdjIter<'a, G, F> 
where G: Graph<'a> + 'a, F: Fn(&G::EType) -> bool {
    type Item = G::AEType;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if (self.cond)(item.edge()) { return Some(item) }
        }
        None
    }
}

pub struct SubEdgeGraph<'a, G, F>
where G: Graph<'a> + 'a, F: Fn(&G::EType) -> bool + 'a {
    g: &'a G,
    cond: F,
}

pub struct EIter<'a, G, F>
where G: Graph<'a> + 'a, F: Fn(&G::EType) -> bool + 'a {
    iter: G::EIter,
    cond: &'a F,
}

impl<'a, G, F> std::iter::Iterator for EIter<'a, G, F>
where G: Graph<'a> + 'a, F: Fn(&G::EType) -> bool + 'a {
    type Item = G::AEType;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if (self.cond)(item.edge()) { return Some(item) }
        }
        None
    }
}

impl<'a, G, F> SubEdgeGraph<'a, G, F>
where G: Graph<'a> + 'a, F: Fn(&G::EType) -> bool + 'a {
    pub fn new(g: &'a G, cond: F) -> Self {
        SubEdgeGraph {
            g: g,
            cond: cond,
        }
    }
}

impl<'a, G, F> Graph<'a> for SubEdgeGraph<'a, G, F>
where G: Graph<'a> + 'a, F: Fn(&G::EType) -> bool + 'a {
    type VType = G::VType;
    type EType = G::EType;
    type AEType = G::AEType;
    type AdjIter = AdjIter<'a, G, F>;
    type EIter = EIter<'a, G, F>;
    type VIter = G::VIter;
    fn delta(&'a self, v: &Self::VType) -> Self::AdjIter {
        AdjIter { iter: self.g.delta(v), cond: &self.cond  }
    }
    fn edges(&'a self) -> Self::EIter {
        EIter { iter: self.g.edges(), cond: &self.cond }
    }
    fn vertices(&'a self) -> Self::VIter {
        self.g.vertices()
    }
    fn v_size(&self) -> usize {
        self.g.v_size()
    }
    fn e_size(&self) -> usize {
        self.g.e_size()
    }
}

impl<'a, G, F> Directed<'a> for SubEdgeGraph<'a, G, F>
where G: Directed<'a> + 'a, F: Fn(&G::EType) -> bool + 'a {}

impl<'a, G, F> Undirected<'a> for SubEdgeGraph<'a, G, F>
where G: Undirected<'a> + 'a, F: Fn(&G::EType) -> bool + 'a {}

impl<'a, G, F> Bipartite<'a> for SubEdgeGraph<'a, G, F>
where G: Bipartite<'a> + 'a, F: Fn(&G::EType) -> bool + 'a {
    type BVIter = G::BVIter;
    fn left_vertices(&'a self) -> Self::BVIter {
        self.g.left_vertices()
    }
    fn right_vertices(&'a self) -> Self::BVIter {
        self.g.left_vertices()
    }
}

impl<'a, G, F> Residual<'a> for SubEdgeGraph<'a, G, F>
where G: Residual<'a> + 'a, G::AEType: ResidualEdge, F: Fn(&G::EType) -> bool + 'a {}

#[test]
fn subedge_graph_test() {
    use graph::graph::DirectedGraph;
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

    let sg = SubEdgeGraph::new(&g, |e| e.2);
    let dist = bfs(&sg, &0);
    assert!(dist[&0] == NNegW::Some(0));
    assert!(dist[&1] == NNegW::Some(1));
    assert!(dist[&2] == NNegW::Some(1));
    assert!(dist[&3] == NNegW::Some(2));
    assert!(dist[&4] == NNegW::Inf);
    assert!(dist[&5] == NNegW::Some(3));
}
