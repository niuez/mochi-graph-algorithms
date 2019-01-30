use graph::*;
use graph::property::*;
use graph::directed_graph::*;
use graph::maxflow::*;

use std::cmp::min;
use std::collections::vec_deque::*;

fn g_level<C: Capacity>(g: &DirectedGraph<MFlowV,MFlowE>, s: &Vite, cap: &mut Vec<C>) -> Vec<i32> {
    let mut level = vec![-1;g.v_size()];

    let mut que = VecDeque::<Vite>::new();
    que.push_back(s.clone());
    level[s.0] = 0;
    while let Some(v) = que.pop_front() {
        for e in g.delta(&v) {
            let to = to(v,g.edge(e));
            if cap[e.0] > C::zero() && level[to.0] == -1{
                level[to.0] = level[v.0] + 1;
                que.push_back(to.clone());
            }
        }
    }
    level
}

fn dinic_dfs<C: Capacity>(g: &DirectedGraph<MFlowV,MFlowE>,iters: &mut Vec<std::slice::Iter<Eite>>, v: &Vite, t: &Vite,cap: &mut Vec<C>, level: &Vec<i32>, f: C) -> C {
    if v == t { f }
    else {
        let mut now = f;
        while let Some(e) = iters[v.0].next() {
            let ee = g.edge(e);
            let to = to(*v,ee);
            let rev = ee.rev;
            if cap[e.0] > C::zero() && level[to.0] > level[v.0] {
                let c = min(now, cap[e.0]);
                let d = dinic_dfs(g,iters,&to,t,cap,level,c);
                cap[e.0] = cap[e.0] - d;
                cap[rev.0] = cap[rev.0] + d;
                now = now - d;
            }
        }
        now = f - now;
        now
    }
}

pub fn dinic_maxflow<C: Capacity>(net: &mut MFlowNetWork<C>) -> C {
    let ref mut cap = &mut net.cap;
    let ref g = & net.g;
    let s = net.source;
    let t = net.shink;
    let mut ans = C::zero();
    let mut inf = C::zero();
    for e in g.delta(&s) {
        inf = inf + cap[e.0];
    }
    loop {
        let level = g_level(g, &s, cap);
        if level[net.shink.0] >= 0 {
            let mut iters = Vec::new();
            for i in 0..g.v_size() {
                iters.push(g.delta(&Vite(i)).clone());
            }
            loop {
                let f = dinic_dfs(&g,&mut iters, &s, &t,cap,&level, inf);
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



