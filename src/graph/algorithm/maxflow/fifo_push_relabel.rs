use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;

use std::cmp::min;
use std::collections::VecDeque;

pub fn fifo_push_relabel<'a, N, C, F>(g: &'a N, s: &N::VType, t: &N::VType, capacity: F) -> C
where N: Residual<'a>, N::AEType: ResidualEdge, C: Capacity, F: Fn(&N::EType) -> C {
    let n = g.v_size();
    let mut cap = Properties::new(g.e_size(), &C::zero());
    let mut ex = Properties::new(g.v_size(), &C::zero());
    let mut d = Properties::new(g.v_size(), &1);
    let mut que = VecDeque::new();
    d[s] = g.v_size();
    d[t] = 0;

    for ref e in g.edges() { cap[e] = capacity(e.edge()); }

    for ref e in g.delta(s) {
        if e.to() != t && e.to() != s && ex[e.to()] == C::zero() && cap[e] > C::zero() {
            que.push_back(e.to().clone());
        }
        ex[e.to()] = ex[e.to()] + cap[e];
        cap[&e.rev()] = cap[e];
        cap[e] = C::zero();
    }

    while let Some(ref v) = que.pop_front() {
        while ex[v] > C::zero() {
            let mut admissible = false;
            for ref e in g.delta(v) {
                if d[e.from()] == d[e.to()] + 1 && cap[e] > C::zero() {
                    admissible = true;
                    let del = min(ex[v], cap[e]);
                    if e.to() != t && e.to() != s && ex[e.to()] == C::zero() && del > C::zero() {
                        que.push_back(e.to().clone());
                    }
                    ex[e.from()] = ex[e.from()] - del;
                    ex[e.to()] = ex[e.to()] + del;
                    cap[e] = cap[e] - del;
                    cap[&e.rev()] = cap[&e.rev()] + del;
                }
            }
            if !admissible {
                d[v] = n + 1;
                for ref e in g.delta(v) {
                    if cap[e] > C::zero() {
                        d[v] = min(d[v], d[e.to()] + 1);
                    }
                }
                if v != s && v != t && ex[v] > C::zero() && d[v] < n + 1 {
                    que.push_back(v.clone());
                }
                else { break; }
            }
        }
    }

    ex[t]
}

#[test]
fn fifo_push_relabel_test() {
    use graph::graph::residual_network::*;
    use graph::property::NNegW;
    {
        let mut g = ResidualNetwork::new(4);
        g.add_edge((0usize, 1usize, 2usize));
        g.add_edge((0, 2, 1));
        g.add_edge((1, 2, 1));
        g.add_edge((1, 3, 1));
        g.add_edge((2, 3, 2));
        let mflow = fifo_push_relabel(&g, &0, &3, |e| NNegW::Some(e.2));
        assert!(mflow == NNegW::Some(3));
    }
}
