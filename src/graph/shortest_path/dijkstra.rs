use graph::Vertex;
use graph::Graph;
use graph::DirectedGraph;
use graph::property::*;

use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq)]
struct DijkstraNode<W: NonNegativeWeighted> {
    dist: Option<W>,
    ver : Vertex,
}

impl<W: NonNegativeWeighted> Ord for DijkstraNode<W> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}
impl<W: NonNegativeWeighted> PartialOrd for DijkstraNode<W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.dist.cmp(&self.dist))
    }
}
impl<W: NonNegativeWeighted> PartialEq for DijkstraNode<W> {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

pub fn dijkstra<VP,EP,W,F>(g: &DirectedGraph<VP,EP>, s: Vertex, start_prop: W, fp: F) -> Vec<Option<W>>
where VP: Property , EP: Property, W: NonNegativeWeighted , F: Fn(&EP) -> &W {
    let n = g.vertices_cnt();
    let mut dist = vec![None ; n];
    dist[s.0] = Some(start_prop);
    
    let mut heap = BinaryHeap::new();

    heap.push(DijkstraNode::<W>{ dist : dist[s.0] , ver : s});

    loop{
        if let Some(DijkstraNode::<W>{dist : Some(d) , ver : v}) = heap.pop() {
            if let Some(now) = dist[v.0] {
                if now < d { continue }
            }
            for e in g.delta(&v) {
                let ok = match dist[e.to.0] {
                    Some(d2) => {
                        if *fp(g.eprop(&e)) + d < d2 {
                            true
                        }
                        else {
                            false
                        }
                    },
                    None => true
                };
                if ok {
                    dist[e.to.0] = Some(*fp(g.eprop(&e)) + d);
                    heap.push(DijkstraNode::<W>{ dist: dist[e.to.0] , ver : e.to.clone() });
                }
            }
        }
        else { break }
    }

    dist
}
