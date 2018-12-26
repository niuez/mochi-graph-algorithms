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
use std::cmp::max;

pub fn dial<'a,VP,EP,G,F>(g: &'a G, s: Vertex,fp: F) -> Vec<Option<usize>>
where VP: Property, EP: Property,G: Directed<'a,VP,EP> + StaticGraph<'a,VP,EP>, F: Fn(&EP) -> &usize {
    let n = g.vertices_cnt();
    let mut mv = usize::zero();
    let mut dist = vec![None;n];
    for i in 0..n {
        for e in g.delta(&Vertex(i)) {
            mv = max(mv, fp(g.eprop(&e)).clone());
        }
    }

    let mut pack = vec![Vec::<Vertex>::new();mv * n + 1];
    dist[s.0] = Some(usize::zero());
    pack[dist[s.0].unwrap()].push(s);

    for d in 0..pack.len() {
        while let Some(v) = pack[d].pop() {
            if dist[v.0].unwrap() < d { continue; }
            for e in g.delta(&v) {
                if dist[e.to.0] == None || dist[e.to.0].unwrap() > d + fp(g.eprop(&e)) {
                    dist[e.to.0] = Some(d + fp(g.eprop(&e)));
                    pack[dist[e.to.0].unwrap()].push(e.to);
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
