use graph::*;
use graph::property::*;

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
where G: Graph<'a>, W: NNegWeight, F: Fn(&G::EType) -> W { 

    let n = g.v_size();
    let mut dist = Properties::new(n, &W::inf());
    dist[s] = W::zero();

    let mut heap = BinaryHeap::new();
    heap.push(DijkstraNode { dist: dist[s], ver: s.clone() });

    while let Some(DijkstraNode { dist: d, ver: v}) = heap.pop() {
        if dist[&v] < d { continue }
        for e in g.delta(&v) {
            if dist[e.from()] + cost(e.edge()) < dist[e.to()] {
                dist[e.to()] = dist[e.from()] + cost(e.edge());
                heap.push(DijkstraNode{ dist: dist[e.to()], ver: e.to().clone() })
            }
        }
    }

    dist
}

#[cfg(test)]
mod dijkstra_test {
    use graph::directed_graph::*;
    use graph::single_source_shortest_path::dijkstra::*;

    #[test]
    fn dijkstra_test() {
        let mut g = DirectedGraph::new(4);
        g.add_edge((0, 1, 1));
        g.add_edge((0, 2, 4));
        g.add_edge((2, 0, 1));
        g.add_edge((1, 2, 2));
        g.add_edge((3, 1, 1));
        g.add_edge((3, 2, 5));

        let dist = dijkstra(&g, &1, |e| NNegW::Some(e.2 as usize));
        assert!(dist[&0] == NNegW::Some(3));
        assert!(dist[&1] == NNegW::Some(0));
        assert!(dist[&2] == NNegW::Some(2));
        assert!(dist[&3] == NNegW::Inf);
    }
}

