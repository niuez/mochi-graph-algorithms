---
layout: default
title: Dinic's Algorithm
---

# Dinic's Algorithm

## Summary

最大フローを`O(V^2E)`で解くAlgorithm. しかし, 計算量よりかなり体感的に早く動く. 最悪ケースのときはしっかり遅い.

実装は蟻本のものを写すのが手っ取り早いが, 一回ずつフローを流す実装になっているのでこれをしないようにすると良い.

## Reference

- 蟻本

- [れんしゅうちょう。最大流問題について.](http://topcoder.g.hatena.ne.jp/Mi_Sawa/20140311/1394730336)

## Code

[mochi-graph-algorithms - dinic.rs](https://github.com/kutimoti/mochi-graph-algorithms/blob/master/src/graph/network/dinic.rs)

```rust
use graph::*;
use graph::property::*;
use graph::network::*;
use graph::directed_graph::*;

use std::cmp::min;
use std::collections::vec_deque::*;

fn g_level<C: Capacity>(g: &DirectedGraph<usize,Edge>, s: &Vertex, zero: C, cap: &mut Vec<C>) -> Vec<i32> {
    let mut level = vec![-1;g.vertices_cnt()];

    let mut que = VecDeque::<Vertex>::new();
    que.push_back(s.clone());
    level[s.0] = 0;
    while let Some(v) = que.pop_front() {
        for e in g.delta(&v) {
            if cap[e.index] > zero && level[e.to.0] == -1{
                level[e.to.0] = level[v.0] + 1;
                que.push_back(e.to.clone());
            }
        }
    }
    level
}

fn dinic_dfs<C: Capacity>(g: &DirectedGraph<usize,Edge>, v: &Vertex, t: &Vertex, zero: C, cap: &mut Vec<C>, level: &Vec<i32>, f: C) -> C {
    if v == t { f }
    else {
        let mut now = f;
        for e in g.delta(v) {
            if cap[e.index] > zero && level[e.to.0] > level[v.0] {
                let c = min(now, cap[e.index]);
                let d = dinic_dfs(g,&e.to,t,zero,cap,level,c);
                cap[e.index] = cap[e.index] - d;
                cap[g.eprop(&e).index] = cap[g.eprop(&e).index] + d;
                now = now - d;
            }
        }
        now = f - now;
        now
    }
}

pub fn dinic<C: Capacity>(net: &mut Network<C>) -> C {
    let ref mut cap = &mut net.cap;
    let ref g = & net.g;
    let s = net.source;
    let t = net.shink;
    let mut ans = net.zero;
    loop {
        let level = g_level(g, &s, net.zero, cap);
        if level[net.shink.0] >= 0 {
            loop {
                let f = dinic_dfs(&g, &s, &t, net.zero,cap,&level, net.inf);
                if f > net.zero {
                    ans = ans + f;
                }
                else {
                    break;
                }
            }
        }
        else {
            break;
        }
    }
    ans
}
```

## Verify

- [AOJ - Maximum Flow](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A)

## Ability

まだやってない. Maximum Flowはまとめてやる.