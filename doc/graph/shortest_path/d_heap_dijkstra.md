---
layout: default
title: d-any heap Dijkstra's Algorithm
---

# d-any heap Dijkstra's Algorithm
---

## Summary

d-any heap Dijkstra's Algorithm は非負重み付きグラフの単一始点の最短経路を`O(E log<sub>E/V<sub>(V)`で求めるAlgorithm.

`d = ceil(2 + m / n)`としてd分heapを使うdijkstra

denseなグラフに対して有効???(要検証)

## Reference

- Tarjan - Data Structures and Network Algorithms

## Code

[mochi-graph-algorithms - d_heap_dijkstra.rs](https://github.com/kutimoti/mochi-graph-algorithms/blob/master/src/graph/shortest_path/d_heap_dijkstra.rs)

```rust
use graph::*;
use graph::property::*;

use std::cmp::min;
use std::cmp::Ordering;

struct DAnyHeap<T> where T: Ord {
    heap: Vec<T>,
    d: usize
}

impl<T: Ord> DAnyHeap<T> {
    fn new(d: usize) -> DAnyHeap<T> {
        DAnyHeap {
            heap: Vec::<T>::new(),
            d: d
        }
    }
    fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() { None }
        else {
            let l = self.heap.len();
            self.heap.swap(0,l - 1);
            let temp = self.heap.pop().unwrap();
            for i in 1..min(self.d + 1, self.heap.len()) {
                if self.heap[i].cmp(&self.heap[0]) == Ordering::Less {
                    self.heap.swap(0, i);
                }
            }
            Some(temp)
        }
    }

    fn push(&mut self, t: T) {
        self.heap.push(t);
        let mut i = self.heap.len() - 1;
        while i > 0 {
            if self.heap[i].cmp(&self.heap[(i - 1) / 2]) == Ordering::Less {
                self.heap.swap(i, (i - 1) / 2);
            }
            i = (i - 1) / 2;
        }
    }
}

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

pub fn d_heap_dijkstra<'a,VP,EP,G,W,F>(g: &'a G, s: Vertex,fp: F) -> Vec<Option<W>>
where VP: Property , EP: Property,G: Directed<'a,VP,EP> + StaticGraph<'a,VP,EP>, W: NonNegativeWeighted , F: Fn(&EP) -> &W {
    let n = g.vertices_cnt();
    let m = g.edges_cnt();
    let mut dist = vec![None ; n];
    dist[s.0] = Some(W::zero());
    
    let mut heap = DAnyHeap::new(2 + (m + n - 1) / n);

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

```

## Verify

- [AOJ - Single Source Shortest Path](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_A)

## Ability

またやる(まだ種類が少ない)
