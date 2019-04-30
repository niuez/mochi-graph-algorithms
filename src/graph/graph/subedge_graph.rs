use graph::kernel::graph::*;

pub struct AdjIter<'g, 'f, G, F>
where G: Graph<'g> + 'g, F: Fn(&G::AEType) -> bool + 'f {
    iter: G::AdjIter,
    cond: &'f F,
}

impl<'g, 'f, G, F> std::iter::Iterator for AdjIter<'g, 'f, G, F> 
where G: Graph<'g> + 'g, F: Fn(&G::AEType) -> bool + 'f {
    type Item = G::AEType;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if (self.cond)(&item) { return Some(item) }
        }
        None
    }
}

pub struct EIter<'g, 'f, G, F>
where G: Graph<'g> + 'g, F: Fn(&G::AEType) -> bool + 'f {
    iter: G::EIter,
    cond: &'f F,
}

impl<'g, 'f, G, F> std::iter::Iterator for EIter<'g, 'f, G, F>
where G: Graph<'g> + 'g, F: Fn(&G::AEType) -> bool + 'f {
    type Item = G::AEType;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if (self.cond)(&item) { return Some(item) }
        }
        None
    }
}

pub struct VIter<'a, 'g, G>
where G: Graph<'g> + 'g {
    iter: G::VIter,
    _marker: std::marker::PhantomData<&'a usize>,
}

impl<'a, 'g: 'a, G> std::iter::Iterator for VIter<'a, 'g, G> 
where Self: 'a, G: Graph<'g> + 'g {
    type Item = &'a G::VType;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct SubEdgeGraph<'g, G, F>
where G: Graph<'g> + 'g, F: Fn(&G::AEType) -> bool {
    g: &'g G,
    cond: F,
}

impl<'g, G, F> SubEdgeGraph<'g, G, F>
where G: Graph<'g> + 'g, F: Fn(&G::AEType) -> bool{ 
    pub fn new(g: &'g G, cond: F) -> Self {
        SubEdgeGraph {
            g: g,
            cond: cond,
        }
    }
}

impl<'a, 'g: 'a, G, F> Graph<'a> for SubEdgeGraph<'g, G, F>
where Self: 'a, G: Graph<'g> + 'g, F: Fn(&G::AEType) -> bool + 'a {
    type VType = G::VType;
    type EType = G::EType;
    type AEType = G::AEType;
    type AdjIter = AdjIter<'g, 'a, G, F>;
    type EIter = EIter<'g, 'a, G, F>;
    type VIter = VIter<'a, 'g, G>;
    fn delta(&'a self, v: &Self::VType) -> Self::AdjIter {
        AdjIter { iter: self.g.delta(v), cond: &self.cond  }
    }
    fn edges(&'a self) -> Self::EIter {
        EIter { iter: self.g.edges(), cond: &self.cond }
    }
    fn vertices(&'a self) -> Self::VIter {
        VIter { iter: self.g.vertices(), _marker: std::marker::PhantomData }
    }
    fn v_size(&self) -> usize {
        self.g.v_size()
    }
    fn e_size(&self) -> usize {
        self.g.e_size()
    }
}

#[cfg(test)]
pub mod subedge_test_mod {
    use graph::kernel::graph::*;
    use graph::algorithm::single_source_shortest_path::bfs;
    use graph::graph::subedge_graph::*;
    use graph::kernel::Properties;
    use graph::property::NNegW;
    fn test_test<'a, G, F>(g: &'a G, s: &G::VType, cond: F) -> Properties<NNegW<usize>>
    where G: Graph<'a>, F: Fn(&G::AEType) -> bool {
        let sg = SubEdgeGraph::new(g, cond);
        bfs(&sg, s)
    }
    #[test]
    fn subedge_graph_test() {
        use graph::graph::DirectedGraph;
        use graph::property::NNegW;
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
        {
            let dist = test_test(&g, &0, |e| e.edge().2);
            assert!(dist[&0] == NNegW::Some(0));
            assert!(dist[&1] == NNegW::Some(1));
            assert!(dist[&2] == NNegW::Some(1));
            assert!(dist[&3] == NNegW::Some(2));
            assert!(dist[&4] == NNegW::Inf);
            assert!(dist[&5] == NNegW::Some(3));
        }
    }
}

