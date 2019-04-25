use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;

use graph::property::PathW;
use graph::algorithm::single_source_shortest_path::{ dijkstra, bellman_ford };

use std::cmp::min;

pub fn successive_shortest_paths<'a, N, Cap, Co, FCap, FCo>(g: &'a N, s: &N::VType, t: &N::VType, flow: Cap, capacity: FCap, cost: FCo) -> Option<Co>
where N: Residual<'a>, N::AEType: ResidualEdge, Cap: Capacity + NNegWeight, Co: Cost<Cap>, <Co as ToNNegWeight>::Output: ToArbWeight<Output=Co>, FCap: Fn(&N::AEType) -> Cap, FCo: Fn(&N::AEType) -> Co {
    let mut ans = Co::zero();
    let mut cap = Properties::new(g.e_size(), &Cap::zero());
    let mut co = Properties::new(g.e_size(), &Co::zero());
    let mut rev = Properties::new(g.e_size(), &0usize);

    for ref e in g.edges() {
        cap[e] = capacity(e);
        co[e] = cost(e);
        co[&e.rev()] = Co::zero() - cost(e);
        rev[e] = e.rev().id();
        rev[&e.rev()] = e.id();
    }

    let mut potential = bellman_ford(g, s, |e| {
        if cap[e] > Cap::zero() { co[e] }
        else { Co::inf() }
    });

    let mut remain = flow;

    while remain > Cap::zero() {
        let dist = dijkstra(g, s, |e| {
            if cap[e] > Cap::zero() { PathW { 
                weight: (co[e] + potential[e.from()] - potential[e.to()]).to_nnegw(), 
                before: Some((e.from().clone(), e.id())),
            } }
            else { PathW::inf() }
        });

        if dist[t] == PathW::inf() { return None }

        let mut ff = remain;
        let mut u = *t;
        while let Some((before, ref id)) = dist[&u].before {
            ff = min(ff, cap[id]);
            u = before;
        }
        let mut u = *t;
        while let Some((before, ref id)) = dist[&u].before {
            ans = ans + co[id] * ff;
            cap[id] = cap[id] - ff;
            cap[&rev[id]] = cap[&rev[id]] + ff;
            u = before;
        }

        remain = remain - ff;

        for v in g.vertices() {
            potential[v] = potential[v] + dist[v].weight.to_arbw();
        }
    }

    Some(ans)
}

#[test]
fn successive_shortest_paths_test() {
    use graph::graph::ResidualNetwork;
    use graph::property::NNegW;
    use graph::property::ArbW;
    {
        let mut g = ResidualNetwork::new(4);
        g.add_edge((0usize, 1usize, (2u32, 1i64)));
        g.add_edge((0, 2, (1, 2)));
        g.add_edge((1, 2, (1, 1)));
        g.add_edge((1, 3, (1, 3)));
        g.add_edge((2, 3, (2, 1)));
        let result = successive_shortest_paths(&g, &0, &3, NNegW::Some(2u32), |e| NNegW::Some((e.edge().2).0), |e| ArbW::Some((e.edge().2).1));

        assert!(result == Some(ArbW::Some(6)));
    }
}

