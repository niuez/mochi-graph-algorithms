use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;

use graph::property::{ PathW };
use graph::algorithm::single_source_shortest_path::{ dijkstra, bellman_ford };
use graph::graph::AntiArborescence;

use std::cmp::min;

#[derive(Clone, Copy)]
struct AntiEdge<AE: AdjEdge>(AE);

impl<AE: AdjEdge> ID for AntiEdge<AE> {
    fn id(&self) -> usize { self.0.id() }
}

impl<AE: AdjEdge> Edge for AntiEdge<AE> {
    type VType = AE::VType;
    fn from(&self) -> &Self::VType { self.0.to() }
    fn to(&self) -> &Self::VType { self.0.from() }
}

impl<AE: AdjEdge> AdjEdge for AntiEdge<AE> {
    type EType = AE;
    fn edge(&self) -> &Self::EType { &self.0 }
}

fn shortest_path_tree<'a, G, W, F>(g: &'a G, s: &G::VType, cost: F) -> (Properties<W>, AntiArborescence<G::VType, AntiEdge<G::AEType>>)
where G: Graph<'a>, W: NNegWeight, F: Fn(&G::AEType) -> W { 
    let mut path = AntiArborescence::new_root(g.v_size(), s.clone());
    let pathw_dist = dijkstra(g, s, |e| PathW {
        weight: cost(e),
        before: Some(e.clone()),
    });
    let dist = pathw_dist.iter().map(|w| w.weight).collect();
    pathw_dist.iter().for_each(|w| {
        if let Some(e) = w.before {
            path.add_vertex(e.to().clone(), AntiEdge(e.clone()))
        }
    });
    (dist, path)
}

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

        let (dist, tree) = shortest_path_tree(g, s, |e| {

            if cap[e] > Cap::zero() { (co[e] + potential[e.from()] - potential[e.to()]).to_nnegw() }
            else { Co::inf().to_nnegw() }
        });
        match tree.root_path(t) {
            None => return None,

            Some(path) => {

                let mut neck = path.clone().fold(remain, |ff, e| min(ff, cap[e.edge()]));
                path.for_each(|ae| {

                    let e = ae.edge();
                    ans = ans + co[e.edge()] * neck;
                    cap[e.edge()] = cap[e.edge()] - neck;
                    cap[&e.edge().rev()] = cap[&e.edge().rev()] + neck;

                });
                remain = remain - neck;
            }
        }

        for v in g.vertices() {
            potential[v] = potential[v] + dist[v].to_arbw();
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

