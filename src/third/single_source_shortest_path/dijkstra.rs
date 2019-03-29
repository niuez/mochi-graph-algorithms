use third::*;
use third::property::*;

use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct DijkstraNode<W: NonNegativeWeighted, V: Vertex> {
    dist: Option<W>,
    ver : V,
}

impl<W: NonNegativeWeighted, V: Vertex> Ord for DijkstraNode<W, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}
impl<W: NonNegativeWeighted, V: Vertex> PartialOrd for DijkstraNode<W, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.dist.cmp(&self.dist))
    }
}
impl<W: NonNegativeWeighted, V: Vertex> PartialEq for DijkstraNode<W, V> {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl<W: NonNegativeWeighted, V: Vertex> Eq for DijkstraNode<W, V> { }


pub fn dijkstra<'a, V, E, G, W, F>(g: &'a G, s: &V, cost: F) -> Properties<Option<W>>
where V: Vertex, E: Edge<VType=V> + 'a, G: Graph<'a, V, E>, W: NonNegativeWeighted, F: Fn(&E) -> W { 

    let n = g.v_size();
    let mut dist = Properties::new(n, &None);
    dist[s] = Some(W::zero());

    let mut heap = BinaryHeap::new();
    heap.push(DijkstraNode { dist: dist[s], ver: s.clone() });

    while let Some(DijkstraNode { dist: Some(d), ver: v}) = heap.pop() {
        if let Some(now) = dist[&v] {
            if now < d { continue }
        }
        for e in g.delta(&v) {
            if match dist[e.to()] {
                Some(d2) => { 
                    cost(e.edge()) + d < d2
                }
                None => true
            } {
                dist[e.to()] = Some(cost(e.edge()) + d);
                heap.push(DijkstraNode{ dist: dist[e.to()], ver: e.to().clone() })
            }
        }
    }

    dist
}

#[cfg(test)]
mod dijkstra_test {
    use third::*;
    use third::directed_graph::*;
    use third::single_source_shortest_path::dijkstra::*;

    #[test]
    fn dijkstra_test() {
        let mut g = DirectedGraph::new(4);
        g.add_edge((0, 1, 1));
        g.add_edge((0, 2, 4));
        g.add_edge((2, 0, 1));
        g.add_edge((1, 2, 2));
        g.add_edge((3, 1, 1));
        g.add_edge((3, 2, 5));

        let dist = dijkstra(&g, &1, |e| e.2);
        assert!(dist[&0] == Some(3));
        assert!(dist[&1] == Some(0));
        assert!(dist[&2] == Some(2));
        assert!(dist[&3] == None);
    }
}

