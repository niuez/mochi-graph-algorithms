use graph::*;
use graph::property::*;
use graph::network::*;
use graph::directed_graph::*;

use std::cmp::min;
use std::collections::vec_deque::*;

fn g_level<C: Capacity>(g: &DirectedGraph<usize,Edge>, s: &Vertex, cap: &mut Vec<C>) -> Vec<i32> {
    let mut level = vec![-1;g.vertices_cnt()];

    let mut que = VecDeque::<Vertex>::new();
    que.push_back(s.clone());
    level[s.0] = 0;
    while let Some(v) = que.pop_front() {
        for e in g.delta(&v) {
            if cap[e.index] > C::zero() && level[e.to.0] == -1{
                level[e.to.0] = level[v.0] + 1;
                que.push_back(e.to.clone());
            }
        }
    }
    level
}

fn dinic_dfs<C: Capacity>(g: &DirectedGraph<usize,Edge>, v: &Vertex, t: &Vertex,cap: &mut Vec<C>, level: &Vec<i32>, f: C) -> C {
    if v == t { f }
    else {
        let mut now = f;
        for e in g.delta(v) {
            if cap[e.index] > C::zero() && level[e.to.0] > level[v.0] {
                let c = min(now, cap[e.index]);
                let d = dinic_dfs(g,&e.to,t,cap,level,c);
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
    let mut ans = C::zero();
    let mut inf = C::zero();
    for e in g.delta(&s) {
        inf = inf + cap[e.index];
    }
    loop {
        let level = g_level(g, &s, cap);
        if level[net.shink.0] >= 0 {
            loop {
                let f = dinic_dfs(&g, &s, &t,cap,&level, inf);
                if f > C::zero() {
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

