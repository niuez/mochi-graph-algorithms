---
layout: default
title: Fujishige's Algorithm
---

# Fujishige's Algorithm

## Summary

整数重みの最大フローを`VElog(MAX CAP)`で解くAlgorithm. Scaling Algorithmの一種である.

## Reference

- [A New Approach to the Maximum-Flow Problem](https://t.co/uJfdmCpKH6)

## Code

[mochi-graph-algorithms - fujishige.rs](https://github.com/kutimoti/mochi-graph-algorithms/blob/master/src/graph/network/fujishige.rs)

```rust
use graph::*;
use graph::property::*;
use graph::network::*;

use std::cmp::min;
use std::cmp::max;

pub fn fujishige(net: &mut Network<usize>) -> usize {
    let ref mut cap = &mut net.cap;
    let ref g = & net.g;
    let s = net.source;
    let t = net.shink;
    let n = net.g.vertices_cnt();

    let mut alpha = usize::zero();
    for v in 0..n {
        for e in g.delta(&Vertex(v)) {
            alpha = max(alpha, cap[e.index]);
        }
    }
    let mut ans = 0;
    while alpha > usize::zero() {
        let mut i = 0;
        let mut isidxed = vec![0;n];
        let mut b = vec![usize::zero();n];
        let mut x = Vec::<Vertex>::new();
        x.push(s);
        isidxed[s.0] += 1;
        let mut ok = true;
        while x[i] != t {
            isidxed[x[i].0] += 1;
            for e in g.delta(&x[i]) {
                if cap[e.index] > usize::zero() && isidxed[e.to.0] != 2 {
                    b[e.to.0] = b[e.to.0] + cap[e.index];
                    if isidxed[e.to.0] == 0 && b[e.to.0] >= alpha {
                        x.push(e.to);
                        isidxed[e.to.0] += 1;
                    }
                }
            }
            if x.len() == i + 1 {
                alpha >>= 1;
                ok = false;
                break;
            }
            i = i + 1;
        }
        if !ok { continue; }

        let mut beta = vec![usize::zero();n];
        beta[t.0] = alpha;
        while i > 0 {
            isidxed[x[i].0] = 0;
            for rev in g.delta(&x[i]) {
                let e = g.eprop(rev);
                if isidxed[e.from.0] != 2 { continue; }
                let del = min(cap[e.index], beta[x[i].0]);
                cap[e.index] = cap[e.index] - del;
                cap[g.eprop(&e).index] = cap[g.eprop(&e).index] + del;
                if e.from == s {
                    ans += del;
                }
                if e.to == s {
                    ans -= del;
                }
                beta[x[i].0] = beta[x[i].0] - del;
                beta[e.from.0] = beta[e.from.0] + del;
            }
            i = i - 1;
        }
    }

    ans
}
```

## Verify

- [AOJ - Maximum Flow](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A)

## Ability

まだやってない. Maximum Flowはまとめてやる.
