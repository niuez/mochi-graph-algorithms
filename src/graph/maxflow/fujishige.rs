use graph::*;
use graph::property::*;
use graph::maxflow::*;

use std::cmp::min;
use std::cmp::max;

pub fn fujishige_maxflow(net: &mut MFlowNetWork<usize>) -> usize {
    let ref mut cap = &mut net.cap;
    let ref g = & net.g;
    let s = net.source;
    let t = net.shink;
    let n = net.g.v_size();

    let mut alpha = usize::zero();
    for v in 0..n {
        for e in g.delta(&Vite(v)) {
            alpha = max(alpha, cap[e.0]);
        }
    }
    let mut ans = 0;
    while alpha > usize::zero() {
        let mut i = 0;
        let mut isidxed = vec![0;n];
        let mut b = vec![usize::zero();n];
        let mut x = Vec::<Vite>::new();
        x.push(s);
        isidxed[s.0] += 1;
        let mut ok = true;
        while x[i] != t {
            isidxed[x[i].0] += 1;
            for e in g.delta(&x[i]) {
                let ee = g.edge(e);
                let to = to(x[i], ee);
                if cap[e.0] > usize::zero() && isidxed[to.0] != 2 {
                    b[to.0] = b[to.0] + cap[e.0];
                    if isidxed[to.0] == 0 && b[to.0] >= alpha {
                        x.push(to);
                        isidxed[to.0] += 1;
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
                let e = g.edge(rev).rev;
                let ee = g.edge(&e);
                let from = ee.from();
                let to = ee.to();
                if isidxed[from.0] != 2 { continue; }
                let del = min(cap[e.0], beta[x[i].0]);
                cap[e.0] = cap[e.0] - del;
                cap[rev.0] = cap[rev.0] + del;
                if from == s {
                    ans += del;
                }
                if to == s {
                    ans -= del;
                }
                beta[x[i].0] = beta[x[i].0] - del;
                beta[from.0] = beta[from.0] + del;
            }
            i = i - 1;
        }
    }

    ans
}
