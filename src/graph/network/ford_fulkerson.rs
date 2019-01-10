use graph::*;
use graph::property::*;
use graph::network::*;
use graph::directed_graph::*;

use std::cmp::min;

fn ff_dfs<C: Capacity>(g: &DirectedGraph<usize,Edge>, v: &Vertex, t: &Vertex, cap: &mut Vec<C>, used: &mut Vec<bool>, f: C) -> C {
    if v == t { f }
    else {
        let mut now = f;
        used[v.0] = true;
        for e in g.delta(v) {
            if !used[e.to.0] && cap[e.index] > C::zero() {
                let c = min(now, cap[e.index]);
                let d = ff_dfs(g, &e.to, t, cap, used, c);
                cap[e.index] = cap[e.index] -  d;
                cap[g.eprop(&e).index] = cap[g.eprop(&e).index] + d;
                now = now - d;
            }
        }
        now = f - now;
        now
    }
}

pub fn ford_fulkerson<C: Capacity>(net: &mut Network<C>) -> C {
    let ref mut cap = &mut net.cap;
    let ref g = & net.g;
    let s = net.source;
    let t = net.shink;
    let mut ff = C::zero();
    let mut inf = C::zero();
    for e in g.delta(&s) {
        inf = inf + cap[e.index];
    }
    loop {
        let mut used = vec![false;g.vertices_cnt()];
        let f = ff_dfs(&g, &s, &t,cap,&mut used, inf);
        if f == C::zero() { break; }
        ff = ff + f;
    }
    ff
}
