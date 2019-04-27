use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;

use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct DijkstraNode<W: NNegWeight, V: Vertex> {
    dist: W,
    ver : V,
}

impl<W: NNegWeight, V: Vertex> Ord for DijkstraNode<W, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}
impl<W: NNegWeight, V: Vertex> PartialOrd for DijkstraNode<W, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.dist.cmp(&self.dist))
    }
}
impl<W: NNegWeight, V: Vertex> PartialEq for DijkstraNode<W, V> {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl<W: NNegWeight, V: Vertex> Eq for DijkstraNode<W, V> { }


pub fn dijkstra<'a, G, W, F>(g: &'a G, s: &G::VType, cost: F) -> Properties<W>
where G: Graph<'a>, W: NNegWeight, F: Fn(&G::AEType) -> W { 

    let n = g.v_size();
    let mut dist = Properties::new(n, &W::inf());
    dist[s] = W::zero();

    let mut heap = BinaryHeap::new();
    heap.push(DijkstraNode { dist: dist[s], ver: s.clone() });

    while let Some(DijkstraNode { dist: d, ver: ref v}) = heap.pop() {
        if dist[v] < d { continue }
        for ref e in g.delta(v) {
            if dist[e.from()] + cost(e) < dist[e.to()] {
                dist[e.to()] = dist[e.from()] + cost(e);
                heap.push(DijkstraNode{ dist: dist[e.to()], ver: e.to().clone() })
            }
        }
    }

    dist
}

#[test]
fn dijkstra_test() {
    use graph::graph::DirectedGraph;
    use graph::property::NNegW;
    let mut g = DirectedGraph::new(4);
    g.add_edge((0usize, 1usize, 1usize));
    g.add_edge((0, 2, 4));
    g.add_edge((2, 0, 1));
    g.add_edge((1, 2, 2));
    g.add_edge((3, 1, 1));
    g.add_edge((3, 2, 5));

    let dist = dijkstra(&g, &1, |e| NNegW::Some(e.edge().2));
    assert!(dist[&0] == NNegW::Some(3));
    assert!(dist[&1] == NNegW::Some(0));
    assert!(dist[&2] == NNegW::Some(2));
    assert!(dist[&3] == NNegW::Inf);
}

