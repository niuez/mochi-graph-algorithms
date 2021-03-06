use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;
use graph::property::NNegW;
use graph::graph::SubEdgeGraph;
use graph::algorithm::single_source_shortest_path::bfs;

use std::cmp::min;

fn dinic_dfs<'a, N, C>(g: &mut Vec<std::iter::Peekable<N::AdjIter>>, v: &N::VType, t: &N::VType, cap: &mut Properties<C>, level: &Properties<NNegW<usize>>, f: C) -> C 
where N: Residual<'a>, N::AEType: ResidualEdge, C: Capacity {
    if v == t { f }
    else {
        let mut flow = C::zero();
        while let Some(ref e) = g[v.id()].peek().cloned() {
            let c = min(f - flow, cap[e]);
            if c > C::zero() && level[e.from()] < level[e.to()] {
                let d = dinic_dfs::<N, C>(g, e.to(), t, cap, level, c);
                cap[e] = cap[e] - d;
                cap[&e.rev()] = cap[&e.rev()] + d;
                flow = flow + d;
                if f == flow { break; }
            }
            g[v.id()].next();
        }

        flow
    }
}

pub fn dinic<'a, N , C, F>(g: &'a N, s: &N::VType, t: &N::VType, cap: F) -> C
where N: Residual<'a>, N::AEType: ResidualEdge, C: Capacity, F: Fn(&N::AEType) -> C {
    let mut ff = C::zero();
    let mut rcap = Properties::new(g.e_size(), &C::zero());
    for ref e in g.edges() {
        rcap[e] = cap(e);
    }
    let mut level;
    while {
        let sg = SubEdgeGraph::new(g, |e| rcap[e] > C::zero());
        level = bfs(&sg, s);
        match level[t] { NNegW::Inf => false, _ => true }
    } {
        let mut g_peek = g.vertices().map(|v| g.delta(v).peekable()).collect();
        ff = ff + dinic_dfs::<N, C>(&mut g_peek, s, t, &mut rcap, &level, C::inf());
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
        let mflow = dinic(&g, &0, &3, |e| NNegW::Some(e.edge().2));
        assert!(mflow == NNegW::Some(3));
    }
}
