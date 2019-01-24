use graph::*;
use graph::property::*;
use graph::maxflow::*;

use std::collections::vec_deque::*;
use std::cmp::min;

pub fn fifo_preflow_relabel_maxflow<C: Capacity>(net: &mut MFlowNetWork<C>) -> C {
    let mut que = VecDeque::<Vite>::new();
    let ref mut cap = &mut net.cap;
    let ref g = & net.g;
    let s = net.source;
    let t = net.shink;
    let n = net.g.v_size();
    let mut ex = vec![C::zero();n];
    let mut d = vec![1;n];
    d[s.0] = n;
    d[t.0] = 0;

    for e in g.delta(&s) {
        let ee = g.edge(e);
        let to = to(s, ee);
        let rev = ee.rev;
        if to != t && to != s && ex[to.0] == C::zero() && cap[e.0] > C::zero() {
            que.push_back(to);
        }
        ex[to.0] = ex[to.0] + cap[e.0];
        cap[rev.0] = cap[e.0];
        cap[e.0] = C::zero();
    }

    while let Some(v) = que.pop_front() {
        while ex[v.0] > C::zero() {
            let mut admissible = false;
            for e in g.delta(&v) {
                let ee = g.edge(e);
                let to = to(v,ee);
                let rev = ee.rev;
                if d[v.0] == d[to.0] + 1 && cap[e.0] > C::zero() {
                    admissible = true;
                    let del = min(ex[v.0],cap[e.0]);
                    if to != t && to != s && ex[to.0] == C::zero() && del > C::zero() {
                        que.push_back(to);
                    }
                    ex[v.0] = ex[v.0] - del;
                    ex[to.0] = ex[to.0] + del;
                    cap[e.0] = cap[e.0] - del;
                    cap[rev.0] = cap[rev.0] + del;
                }
            }
            if !admissible {
                d[v.0] = n + 1;
                for e in g.delta(&v) {
                    if cap[e.0] > C::zero() {
                        d[v.0] = min(d[v.0] , d[to(v,g.edge(e)).0] + 1);
                    }
                }
                if v != s && v != t && ex[v.0] > C::zero() && d[v.0] < n + 1 {
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
