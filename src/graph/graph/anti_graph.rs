use graph::kernel::graph::*;

#[derive(Clone, Copy)]
pub struct AntiAdjEdge<'g, AE: AdjEdge + 'g>(&'g AE, usize);

impl<'a, AE> ID for AntiAdjEdge<'a, AE>
where AE: AdjEdge + 'a {
    fn id(&self) -> usize { self.1 }
}

impl<'a, AE> Edge for AntiAdjEdge<'a, AE>
where AE: AdjEdge + 'a {
    type VType = AE::VType;
    fn from(&self) -> &AE::VType { self.0.to() }
    fn to(&self) -> &AE::VType { self.0.from() }
}

impl<'a, AE> AdjEdge for AntiAdjEdge<'a, AE>
where AE: AdjEdge + 'a {
    type EType = AE;
    fn edge(&self) -> &Self::EType { self.0 }
}

pub struct AdjIter<'a, AE: AdjEdge + 'a> {
    iter: std::slice::Iter<'a, usize>,
    edges: &'a Vec<AE>
}

impl<'a, AE: AdjEdge> std::iter::Iterator for AdjIter<'a, AE> {
    type Item = AntiAdjEdge<'a, AE>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(ei) => {
                Some( AntiAdjEdge(&self.edges[*ei], *ei) )
            }
            None => None
        }    
    }
}

pub struct EIter<'a, AE: AdjEdge + 'a> {
    i: usize,
    iter: std::slice::Iter<'a, AE>,
}

impl<'a, AE: AdjEdge + 'a> std::iter::Iterator for EIter<'a, AE> {
    type Item = AntiAdjEdge<'a, AE>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(e) => {
                let i = self.i;
                self.i += 1;
                Some( AntiAdjEdge(e, i) )
            }
            None => None
        }
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

pub struct AntiGraph<'g, G>
where G: Graph<'g> + 'g {
    g: &'g G,
    adj: Vec<Vec<usize>>,
    es: Vec<G::AEType>,
}

impl<'g, G> AntiGraph<'g, G>
where G: Graph<'g> + 'g {
    pub fn new(g: &'g G) -> Self {
        let mut es = Vec::new();
        let mut adj = vec![Vec::new(); g.v_size()];
        let mut i = 0;
        for e in g.edges() {
            adj[e.to().id()].push(i);
            i += 1;
            es.push(e);
        }
        AntiGraph {
            g: g,
            adj: adj,
            es: es,
        }
    }
}

impl<'a, 'g: 'a, G> Graph<'a> for AntiGraph<'g, G>
where Self: 'a, G: Graph<'g> + 'g {
    type VType = G::VType;
    type EType = G::AEType;
    type AEType = AntiAdjEdge<'a, G::AEType>;
    type AdjIter = AdjIter<'a, G::AEType>;
    type EIter = EIter<'a, G::AEType>;
    type VIter = VIter<'a, 'g, G>;
    fn delta(&'a self, v: &Self::VType) -> Self::AdjIter {
        AdjIter { iter: self.adj[v.id()].iter(), edges: &self.es }
    }
    fn edges(&'a self) -> Self::EIter {
        EIter { i: 0, iter: self.es.iter() }
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
pub mod anti_graph_test_mod {
    use graph::kernel::graph::*;
    use graph::algorithm::single_source_shortest_path::bfs;
    use graph::graph::anti_graph::*;
    use graph::kernel::Properties;
    use graph::property::NNegW;
    use graph::graph::DirectedGraph;

    fn anti_bfs<'a, G>(g: &'a G, s: &G::VType) -> Properties<NNegW<usize>>
    where G: Graph<'a> {
        let sg = AntiGraph::new(g);
        bfs(&sg, s)
    }

    #[test]
    fn subedge_graph_test() {
        let mut g = DirectedGraph::new(3);

        g.add_edge((0, 1));
        g.add_edge((0, 2));

        let dist = anti_bfs(&g, &2);
        assert!(dist[&0] == NNegW::Some(1));
        assert!(dist[&1] == NNegW::Inf);
        assert!(dist[&2] == NNegW::Some(0));
    }
}

