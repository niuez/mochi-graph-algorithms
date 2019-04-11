use second::*;
use second::property::*;

use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq)]
struct DijkstraNode<W: NonNegativeWeighted> {
    dist: Option<W>,
    ver : Vite,
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

pub fn dijkstra_s3p<'a,V,E,G,W,F>(g: &'a G, s: Vite,fp: F) -> Vec<Option<W>>
where V: Vertex , E: Edge,G: Graph<'a,V,E>, W: NonNegativeWeighted , F: Fn(&E) -> W {
    let n = g.v_size();
    let mut dist = vec![None ; n];
    dist[s.0] = Some(W::zero());
    
    let mut heap = BinaryHeap::new();

    heap.push(DijkstraNode::<W>{ dist : dist[s.0] , ver : s});

    loop{
        if let Some(DijkstraNode::<W>{dist : Some(d) , ver : v}) = heap.pop() {
            if let Some(now) = dist[v.0] {
                if now < d { continue }
            }
            for ei in g.delta(&v) {
                let e = g.edge(ei);
                let to = to(v,e);
                if match dist[to.0] {
                    Some(d2) => {
                        if fp(e) + d < d2 { true }
                        else { false }
                    }
                    None => true
                } {
                    dist[to.0] = Some(fp(e) + d);
                    heap.push(DijkstraNode::<W>{ dist: dist[to.0] , ver : to.clone() });
                }
            }
        }
        else { break }
    }

    dist
}
