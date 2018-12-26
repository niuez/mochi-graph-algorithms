---
layout: default
title: Dial's Algorithm
---

# Dial's Algorithm

## Summary

`V * W`が小さい時にできるDijkstra高速化, 最短距離が`d`の頂点`v`を`pack[d]`に突っ込んでやる.

空間量`O(V * Wmax)`,計算量`O(E + V * Wmax)`

## Reference

- [ACM - shortest-path forest with topological ordering](https://dl.acm.org/citation.cfm?id=363610)

- [GeeksforGeeks](https://www.geeksforgeeks.org/dials-algorithm-optimized-dijkstra-for-small-range-weights/)

## Code

[mochi-graph-algorithms - dial.rs](https://github.com/kutimoti/mochi-graph-algorithms/blob/master/src/graph/shortest_path/dial.rs)

```rust
use graph::*;
use graph::property::*;

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum BFResult<W> {
    Some(W),
    NegInf,
    None
}

/// return distances from s to all vertices in graph by Bellman-Ford Algorithm.
/// running time O(EV)
/// if a vertex cannot reach from s, result of the vertex is `None`,
/// else if the distance of the vertex can be -INF , result is `NegInf` and
/// else result is `Some(distance)`
pub fn bellman_ford<'a,VP,EP,G,W,F>(g: &'a G, s: Vertex, fp: F) -> Vec<BFResult<W>>
where VP: Property, EP: Property,G: Directed<'a,VP,EP> + StaticGraph<'a,VP,EP>, W: Weighted, F: Fn(&EP) -> &W {
    let n = g.vertices_cnt();
    let mut dist = vec![BFResult::None ; n];
    dist[s.0] = BFResult::Some(W::zero());
    
    for _ in 0..n+1 {
        for v in 0..n {
            if let BFResult::Some(dv) = dist[v] {
                for e in g.delta(&Vertex(v)) {
                    dist[e.to.0] = match dist[e.to.0] {
                        BFResult::Some(dt) => {
                            if dv + *fp(g.eprop(e)) < dt {
                                BFResult::Some(dv + *fp(g.eprop(e)))
                            }
                            else {
                                BFResult::Some(dt)
                            }
                        }
                        BFResult::None => {
                            BFResult::Some(dv + *fp(g.eprop(e)))
                        }
                        _ => {
                            BFResult::None
                        }
                    }
                }
            }
        }
    }
    for _ in 0..n+1 {
        for v in 0..n {
            if let BFResult::Some(dv) = dist[v] {
                for e in g.delta(&Vertex(v)) {
                    dist[e.to.0] = match dist[e.to.0] {
                        BFResult::Some(dt) => {
                            if dv + *fp(g.eprop(e)) < dt {
                                BFResult::NegInf
                            }
                            else {
                                BFResult::Some(dt)
                            }
                        }
                        BFResult::None => {
                            BFResult::Some(dv + *fp(g.eprop(e)))
                        }
                        BFResult::NegInf => {
                            BFResult::NegInf
                        }
                    }
                }
            }
            else if let BFResult::NegInf = dist[v] {
                for e in g.delta(&Vertex(v)) {
                    dist[e.to.0] = BFResult::NegInf;
                }
            }
        }
    }

    dist
}
```

## Verify

testでbellman_fordでverifiedしたつもり

## Ability

またやる(まだ種類が少ない)
