---
layout: default
title: Bellman-Ford Algorithm
---

# Bellman-Ford Algorithm

## Summary

Bellman-Fordは重み付きグラフの単一始点の最短経路を`O(VE)`で求めるAlgorithmである.

Bellman-Fordは負の辺を含んでいても動き, また最短路が無限に小さくなることも判定できる.

## Reference

- 蟻本

## Code

[mochi-graph-algorithms - bellman_ford.rs](https://github.com/kutimoti/mochi-graph-algorithms/blob/master/src/graph/shortest_path/bellman_ford.rs)

```rust
use graph::*;
use graph::property::*;

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum BFResult<W> {
    Some(W),
    NegInf,
    None
}

pub fn bellman_ford<'a,VP,EP,G,W,F>(g: &'a G, s: Vertex, start_prop: W, fp: F) -> Vec<BFResult<W>>
where VP: Property, EP: Property,G: Directed<'a,VP,EP> + StaticGraph<'a,VP,EP>, W: Weighted, F: Fn(&EP) -> &W {
    let n = g.vertices_cnt();
    let mut dist = vec![BFResult::None ; n];
    dist[s.0] = BFResult::Some(start_prop);
    
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

- [AOJ - Single Source Shortest Path(Negative Edges)](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_B)

- [AtCoder - Score Attack](https://beta.atcoder.jp/contests/abc061/tasks/abc061_d)

## Ability

既知なのでいいかな...