---
layout: default
title: Dijkstra's Algorithm
---

# Dijkstra's Algorithm

## Summary

Dijkstra's Algorithm は非負重み付きグラフの単一始点の最短経路を`O((E + V)logV) (binary heap)`で求めるAlgorithmである.

`Pairing Heap`などを使えばもっと早い.

## Reference

- 蟻本

- [dep notes - ダイクストラ法(最短経路問題)](http://www.deqnotes.net/acmicpc/dijkstra/)

## Code

[mochi-graph-algorithms - dijkstra.rs](https://github.com/kutimoti/mochi-graph-algorithms/blob/master/src/graph/shortest_path/dijkstra.rs)

```rust
use graph::*;
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

pub fn dijkstra<'a,VP,EP,G,W,F>(g: &'a G, s: Vertex, start_prop: W, fp: F) -> Vec<Option<W>>
where VP: Property , EP: Property,G: Directed<'a,VP,EP> + StaticGraph<'a,VP,EP>, W: NonNegativeWeighted , F: Fn(&EP) -> &W {
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

```

## Verify

- [AOJ - Single Source Shortest Path](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_A)

## Ability

またやる(まだ種類が少ない)