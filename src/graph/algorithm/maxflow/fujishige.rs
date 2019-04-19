use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;

use std::cmp::{ min, max };

pub fn fujishige<'a, N, C, F>(g: &'a N, s: &N::VType, t: &N::VType, capacity: F) -> C
where N: Residual<'a>, N::AEType: ResidualEdge, C: Capacity, F: Fn(&N::EType) -> C {
    let n = g.v_size();
    let mut cap = Properties::new(g.e_size(), &C::zero());
    let mut alpha = C::zero();
    let mut ans = C::zero();
    for ref e in g.edges() {
        cap[e] = capacity(e.edge());
        alpha = max(alpha, cap[e]);
    }
    while alpha > C::zero() {
        let mut i = 0;
        let mut index = Properties::new(n, &0);
        let mut b = Properties::new(n, &C::zero());
        let mut x = Vec::new();
        x.push(s.clone());
        index[s] += 1;
        let mut ok = true;
        while x[i] != *t {
            index[&x[i]] += 1;
            for ref e in g.delta(&x[i]) {
                if cap[e] > C::zero() && index[e.to()] != 2 {
                    b[e.to()] = b[e.to()] + cap[e];
                    if index[e.to()] == 0 && b[e.to()] >= alpha {
                        x.push(e.to().clone());
                        index[e.to()] += 1;
                    }
                }
            }
            if x.len() == i + 1 {
                alpha = alpha >> 1;
                ok = false;
                break;
            }
            i = i + 1;
        }
         if !ok { continue; }
         
         let mut beta = Properties::new(n, &C::zero());
         beta[t] = alpha;
         while i > 0 {
             index[&x[i]] = 0;
             for ref rev in g.delta(&x[i]) {
                 let e = &rev.rev();
                 if index[e.from()] != 2 { continue; }
                 let del = min(cap[e], beta[&x[i]]);
                 cap[e] = cap[e] - del;
                 cap[rev] = cap[rev] + del;
                 if e.from() == s {
                     ans = ans + del;
                 }
                 if e.to() == s {
                     ans = ans - del;
                 }
                 beta[e.to()] = beta[e.to()] - del;
                 beta[e.from()] = beta[e.from()] + del;
             }
             i = i - 1;
         }
    }

    ans
}

#[test]
fn fujishige_test() {
    use graph::graph::residual_network::*;
    use graph::property::NNegW;
    {
        let mut g = ResidualNetwork::new(4);
        g.add_edge((0usize, 1usize, 2usize));
        g.add_edge((0, 2, 1));
        g.add_edge((1, 2, 1));
        g.add_edge((1, 3, 1));
        g.add_edge((2, 3, 2));
        let mflow = fujishige(&g, &0, &3, |e| NNegW::Some(e.2));
        assert!(mflow == NNegW::Some(3));
    }
}
