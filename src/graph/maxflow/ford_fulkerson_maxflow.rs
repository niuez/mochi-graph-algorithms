use graph::*;
use graph::property::*;
use graph::maxflow::*;
use graph::directed_graph::*;

use std::cmp::min;

fn ff_dfs<C: Capacity>(g: &DirectedGraph<MFlowV,MFlowE>, v: &Vite, t: &Vite, cap: &mut Vec<C>, used: &mut Vec<bool>, f: C) -> C {
    if v == t { f }
    else {
        let mut now = f;
        used[v.0] = true;
        for e in g.delta(v) {
            let ee = g.edge(e);
            let to = to(*v,ee);
            let rev = ee.rev;
            if !used[to.0] && cap[e.0] > C::zero() {
                let c = min(now, cap[e.0]);
                let d = ff_dfs(g, &to, t, cap, used, c);
                cap[e.0] = cap[e.0] -  d;
                cap[rev.0] = cap[rev.0] + d;
                now = now - d;
            }
        }
        now = f - now;
        now
    }
}

pub fn ford_fulkerson<C: Capacity>(net: &mut MFlowNetWork<C>) -> C {
    let ref mut cap = &mut net.cap;
    let ref g = & net.g;
    let s = net.source;
    let t = net.shink;
    let mut ff = C::zero();
    let mut inf = C::zero();
    for e in g.delta(&s) {
        inf = inf + cap[e.0];
    }
    loop {
        let mut used = vec![false;g.v_size()];
        let f = ff_dfs(&g, &s, &t,cap,&mut used, inf);
        if f == C::zero() { break; }
        ff = ff + f;
    }
    ff
}
