use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;

use std::cmp::min;

pub fn ff_dfs<'a, N, C>(g: &'a N, v: &N::VType, t: &N::VType, cap: &mut Properties<C>, used: &mut Properties<bool>, f: C) -> C
where N: Residual<'a>, N::AEType: ResidualEdge, C: Capacity {
    if v == t { f }
    else {
        let mut flow = C::zero();
        used[v] = true;
        for ref e in g.delta(v) {
            if !used[e.to()] && cap[e] > C::zero() {
                let c = min(f - flow, cap[e]);
                let d = ff_dfs(g, e.to(), t, cap, used, c);
                cap[e] = cap[e] - d;
                cap[&e.rev()] = cap[&e.rev()] + d;
                flow = flow + d;
            }
        }

        flow
    }
}

pub fn ford_fulkerson<'a, N, C, F>(g: &'a N, s: &N::VType, t: &N::VType, cap: F) -> C 
where N: Residual<'a>, N::AEType: ResidualEdge, C: Capacity, F: Fn(&N::AEType) -> C {
    let mut ff = C::zero();
    let mut rcap = Properties::new(g.e_size(), &C::zero());
    for ref e in g.edges() {
        rcap[e] = cap(e);
    }
    loop {
        let mut used = Properties::new(g.v_size(), &false);
        let f = ff_dfs(g, s, t, &mut rcap, &mut used, C::inf());
        if f == C::zero() { break; }
        ff = ff + f;
    }

    ff
}

#[test]
fn ford_fulkerson_test() {
    use graph::graph::residual_network::*;
    use graph::property::NNegW;
    {
        let mut g = ResidualNetwork::new(4);
        g.add_edge((0usize, 1usize, 2usize));
        g.add_edge((0, 2, 1));
        g.add_edge((1, 2, 1));
        g.add_edge((1, 3, 1));
        g.add_edge((2, 3, 2));
        let mflow = ford_fulkerson(&g, &0, &3, |e| NNegW::Some(e.edge().2));
        assert!(mflow == NNegW::Some(3));
    }
}

#[test]
#[ignore]
fn ford_fulkerson_test2() {
    use graph::graph::ResidualNetwork;
    use graph::property::NNegW;
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let v:Vec<usize> = s.trim().split_whitespace()
        .map(|e|e.parse().unwrap()).collect();
    let (n,m) = (v[0] , v[1]);
    let mut g = ResidualNetwork::new(n);
   
    for _ in 0..m {
        let mut t = String::new();
        std::io::stdin().read_line(&mut t).unwrap();
        let a:Vec<usize> = t.trim().split_whitespace()
            .map(|e|e.parse().unwrap()).collect();
        let (x,y,w) = (a[0],a[1],a[2]);
        g.add_edge((x, y, w));
    }
    if let NNegW::Some(flow) = ford_fulkerson(&g, &0, &(n - 1), |e| NNegW::Some(e.edge().2)) {
        println!("{}", flow);
    }
}
