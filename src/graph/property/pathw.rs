use graph::kernel::graph::*;
use graph::kernel::property::*;

#[derive(Clone, Copy)]
pub struct PathW<W, V> where W: ArbWeight, V: Vertex {
    pub weight: W,
    pub before: Option<V>,
    pub eid: Option<usize>,
}

impl<W, V> ToNNegWeight for PathW<W, V> where W: ArbWeight, V: Vertex {
    type Output = PathW<<W as ToNNegWeight>::Output, V>;
    fn to_nnegw(&self) -> Self::Output {
        PathW { weight: self.weight.to_nnegw(), before: self.before, eid: self.eid }
    }
}

impl<W, V> ToArbWeight for PathW<W, V> where W: ArbWeight, V: Vertex {
    type Output = PathW<<W as ToArbWeight>::Output, V>;
    fn to_arbw(&self) -> Self::Output {
        PathW { weight: self.weight.to_arbw(), before: self.before, eid: self.eid }
    }
}

impl<W, V> std::ops::Add for PathW<W, V> where W: ArbWeight, V: Vertex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        PathW { weight: self.weight + rhs.weight, before: rhs.before, eid: rhs.eid }
    }
}

impl<W, V> std::cmp::PartialEq for PathW<W, V> where W: ArbWeight, V: Vertex {
    fn eq(&self, rhs: &Self) -> bool {
        self.weight == rhs.weight
    }
}
impl<W, V> std::cmp::Eq for PathW<W, V> where W: ArbWeight, V: Vertex {}


impl<W, V> std::cmp::PartialOrd for PathW<W, V> where W: ArbWeight, V: Vertex {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl<W, V> std::cmp::Ord for PathW<W, V> where W: ArbWeight, V: Vertex {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&rhs.weight)
    }
}

impl<W, V> ArbWeight for PathW<W, V> where W: ArbWeight, V: Vertex {
    fn inf() -> Self { PathW{ weight: W::inf(), before: None, eid: None } }
    fn zero() -> Self { PathW{ weight: W::zero(), before: None, eid: None } }
    fn neg_inf() -> Self { PathW{ weight: W::neg_inf(), before: None, eid: None } }
}

impl<W, V> NNegWeight for PathW<W, V> where W: NNegWeight, V: Vertex {}

#[test]
fn pathw_test() {
    use graph::graph::DirectedGraph;
    use graph::property::NNegW;
    use graph::algorithm::single_source_shortest_path::dijkstra;
    {
        let mut g = DirectedGraph::new(6);
        g.add_edge((0usize, 1usize, 1usize));
        g.add_edge((0, 2, 2));
        g.add_edge((1, 3, 2));
        g.add_edge((2, 3, 3));
        g.add_edge((2, 4, 2));
        g.add_edge((1, 5, 4));
        g.add_edge((4, 5, 2));

        let path = dijkstra(&g, &0, |e| PathW{ weight: NNegW::Some(e.edge().2), before: Some(e.from().clone()), eid: Some(e.id()) });

        assert!(path[&0].weight == NNegW::Some(0) && path[&0].before == None);
        assert!(path[&1].weight == NNegW::Some(1) && path[&1].before == Some(0));
        assert!(path[&2].weight == NNegW::Some(2) && path[&2].before == Some(0));
        assert!(path[&3].weight == NNegW::Some(3) && path[&3].before == Some(1));
        assert!(path[&4].weight == NNegW::Some(4) && path[&4].before == Some(2));
        assert!(path[&5].weight == NNegW::Some(5) && path[&5].before == Some(1));
    }
}
