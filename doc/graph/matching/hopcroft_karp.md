---
layout: default
title: Hopcroft-Karp Algorithm
---

# Hopcroft-Karp Algorithm

## Summary

Bipartite Cardinality Matching Problem(二部グラフでの最大マッチング)を `O(√V E)`で解くAlgorithm.二部マッチングでは最速(だと思う).

ただし,最大フローの`Dinic Algorithm`を使えば同じ計算量で求められることが証明されているので, わざわざ作る必要は無いかも知れない.

証明がかなりすごいのでぜひ論文を見てほしい.

## Reference

- [An n^2.5 Algorithm for Maximum Matchings in Bipartite Graphs](https://epubs.siam.org/doi/abs/10.1137/0202019)

- [ei1333.github.io - 二部グラフの最大マッチング(Hopcroft-Karp)](https://ei1333.github.io/luzhiled/snippets/graph/hopcroft-karp.html)

- [‪Qiita - 実世界で超頻出！二部マッチング (輸送問題、ネットワークフロー問題）の解法を総整理！‬](https://qiita.com/drken/items/e805e3f514acceb87602)

## Code

[mochi-graph-algorithms - hopcroft_karp.rs](https://github.com/kutimoti/mochi-graph-algorithms/blob/master/src/graph/matching/hopcroft_karp.rs)

```rust
use graph::*;
use graph::property::*;

use std::collections::vec_deque::*;

pub fn hk_dfs<'a,VP,EP,G>(g:&'a G, v: &Vertex, dist: &mut Vec<i32>, mate: &mut Vec<Option<usize>>, used: &mut Vec<bool>, vis: &mut Vec<bool>) -> bool
where VP: Property, EP: Property, G: Bipartite<'a,VP,EP> + StaticGraph<'a,VP,EP>{
    vis[v.0] = true;
    for e in g.delta(v) {
        let ok = match mate[e.to.0] {
            Some(m) => {
                if !vis[m] && dist[m] == dist[v.0] + 1 && hk_dfs(g,&Vertex(m),dist,mate,used,vis) {
                    true
                }
                else {
                    false
                }
            }
            None => {
                true
            }
        };
        if ok {
            mate[e.to.0] = Some(v.0);
            used[v.0] = true;
            return true;
        }
    }

    false
}

pub fn hopcroft_karp<'a,VP,EP,G>(g: &'a G) -> Vec<(Vertex,Vertex)>
where VP: Property, EP: Property, G: Bipartite<'a,VP,EP> + StaticGraph<'a,VP,EP> {
    let mut ans = Vec::<(Vertex,Vertex)>::new();
    let n = g.vertices_cnt();
    
    let mut mate: Vec<Option<usize>> = vec![None;n];
    let mut used = vec![false;n];

    loop {
        let mut vis = vec![false;n];
        let mut dist = vec![-1;n];
        let mut que = VecDeque::new();

        for i in 0..n {
            if !used[i] {
                que.push_back(i);
                dist[i] = 0;
            }
        }

        while let Some(v) = que.pop_front() {
            for e in g.delta(&Vertex(v)) {
                if let Some(m) = mate[e.to.0] {
                    if dist[m] == -1 {
                        dist[m] = dist[v] + 1;
                        que.push_back(m);
                    }
                }
            }
        }

        let mut has_end = true;
        for i in g.left_vertices() {
            if !used[i.0] && hk_dfs(g,i,&mut dist,&mut mate,&mut used,&mut vis) {
                has_end = false;
            }
        }
        if has_end {
            break;
        }
    }

    for i in g.right_vertices() {
        if let Some(m) = mate[i.0] {
            ans.push((i.clone(),Vertex(m)));
        }
    }
    ans
}
```

## Verify

- [AOJ - Bipartite Matching](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_7_A)

- [AtCoder - 2D Plane 2N Points](https://beta.atcoder.jp/contests/abc091/tasks/arc092_a)