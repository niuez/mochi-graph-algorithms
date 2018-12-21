---
layout: default
title: FIFO Preflow Relabel Algorithm
---

# FIFO Preflow Relabel Algorithm

## Summary

最大フローを`O(V^3)`で解くAlgorithm. 単純に早いし, Dinicの最悪ケースが抜けられるので競技プログラミングでは重宝しそう.

## Reference

- [A New Approach to the Maximum-Flow Problem](https://t.co/uJfdmCpKH6)

## Code

[mochi-graph-algorithms - fifo_preflow_relabel.rs](https://github.com/kutimoti/mochi-graph-algorithms/blob/master/src/graph/network/fifo_preflow_relabel.rs)

```rust
use graph::*;
use graph::property::*;
use graph::network::*;

use std::collections::vec_deque::*;
use std::cmp::min;

pub fn fifo_preflow_relabel<C: Capacity>(net: &mut Network<C>) -> C {
    let mut que = VecDeque::<Vertex>::new();
    let ref mut cap = &mut net.cap;
    let ref g = & net.g;
    let s = net.source;
    let t = net.shink;
    let n = net.g.vertices_cnt();
    let mut ex = vec![net.zero;n];
    let mut d = vec![1;n];
    d[s.0] = n;
    d[t.0] = 0;

    for e in g.delta(&s) {
        if e.to != t && e.to != s && ex[e.to.0] == net.zero && cap[e.index] > net.zero {
            que.push_back(e.to);
        }
        ex[e.to.0] = ex[e.to.0] + cap[e.index];
        cap[g.eprop(&e).index] = cap[e.index];
        cap[e.index] = net.zero;
    }

    while let Some(v) = que.pop_front() {
        while ex[v.0] > net.zero {
            let mut admissible = false;
            for e in g.delta(&v) {
                if d[v.0] == d[e.to.0] + 1 && cap[e.index] > net.zero {
                    admissible = true;
                    let del = min(ex[v.0],cap[e.index]);
                    if e.to != t && e.to != s && ex[e.to.0] == net.zero && del > net.zero {
                        que.push_back(e.to);
                    }
                    ex[v.0] = ex[v.0] - del;
                    ex[e.to.0] = ex[e.to.0] + del;
                    cap[e.index] = cap[e.index] - del;
                    cap[g.eprop(&e).index] = cap[g.eprop(&e).index] + del;
                }
            }
            if !admissible {
                d[v.0] = n + 1;
                for e in g.delta(&v) {
                    if cap[e.index] > net.zero {
                        d[v.0] = min(d[v.0] , d[e.to.0] + 1);
                    }
                }
                if v != s && v != t && ex[v.0] > net.zero && d[v.0] < n + 1 {
                    que.push_back(v);
                }
                else {
                    break;
                }
            }
        }
    }

    ex[t.0]
}
```

## Verify

- [AOJ - Maximum Flow](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A)

## Ability

まだやってない. Maximum Flowはまとめてやる.
