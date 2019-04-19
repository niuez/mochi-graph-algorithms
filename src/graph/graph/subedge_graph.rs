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

