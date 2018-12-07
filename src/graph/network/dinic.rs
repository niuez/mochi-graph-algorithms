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
        for e in g.delta(v) {
            if cap[e.index] > zero && level[e.to.0] > level[v.0] {
                let mi = min(f, cap[e.index]);
                let fl = dinic_dfs(g,&e.to,t,zero,cap,level,mi);
                if fl > zero {
                    cap[e.index] = cap[e.index] - fl;
                    cap[g.eprop(&e).index] = cap[g.eprop(&e).index] + fl;
                    return fl;
                }
            }
        }
        zero
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
