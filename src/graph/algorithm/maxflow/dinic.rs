use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;
use graph::property::NNegW;

use std::cmp::min;
use std::collections::VecDeque;

fn dinic_bfs<'a, N, C>(g: &'a N, s: &N::VType, cap : &Properties<C>) -> Properties<NNegW<usize>> 
where N: Residual<'a>, N::AEType: ResidualEdge, C: Capacity {
    let mut dist = Properties::new(g.v_size(), &NNegW::inf());
    dist[s] = NNegW::zero();
    let mut que = VecDeque::new();
    que.push_back(s.clone());
    while let Some(ref v) = que.pop_front() {
        for ref e in g.delta(v) {
            if cap[e] > C::zero() && dist[e.from()] + NNegW::Some(1) < dist[e.to()] {
                dist[e.to()] = dist[e.from()] + NNegW::Some(1);
                que.push_back(e.to().clone());
            }
        }
    }

    dist 
}

fn dinic_dfs<'a, N, C>(g: &'a N, v: &N::VType, t: &N::VType, cap: &mut Properties<C>, level: &Properties<NNegW<usize>>, f: C) -> C 
where N: Residual<'a>, N::AEType: ResidualEdge, C: Capacity {
    if v == t { f }
    else {
        let mut flow = C::zero();
        for ref e in g.delta(v) {
            if cap[e] > C::zero() && level[e.from()] < level[e.to()] {
                let c = min(f - flow, cap[e]);
                let d = dinic_dfs(g, e.to(), t, cap, level, c);
                cap[e] = cap[e] - d;
                cap[&e.rev()] = cap[&e.rev()] + d;
                flow = flow + d;
            }
        }

        flow
    }
}

pub fn dinic<'a, N , C, F>(g: &'a N, s: &N::VType, t: &N::VType, cap: F) -> C
where N: Residual<'a>, N::AEType: ResidualEdge, C: Capacity, F: Fn(&N::EType) -> C {
    let mut ff = C::zero();
    let mut rcap = Properties::new(g.e_size(), &C::zero());
    for ref e in g.edges() {
        rcap[e] = cap(e.edge());
    }
    let mut level;
    while {
        level = dinic_bfs(g, s, &rcap);
        match level[t] { NNegW::Inf => false, _ => true }
    } {
        ff = ff + dinic_dfs(g, s, t, &mut rcap, &level, C::inf());
    }

    ff
}

#[test]
fn dinic_test() {
    use graph::graph::residual_network::*;
    use graph::property::NNegW;
    {
        let mut g = ResidualNetwork::new(4);
        g.add_edge((0usize, 1usize, 2usize));
        g.add_edge((0, 2, 1));
        g.add_edge((1, 2, 1));
        g.add_edge((1, 3, 1));
        g.add_edge((2, 3, 2));
        let mflow = dinic(&g, &0, &3, |e| NNegW::Some(e.2));
        assert!(mflow == NNegW::Some(3));
    }
}
