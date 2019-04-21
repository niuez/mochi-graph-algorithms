use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;
use graph::algorithm::single_source_shortest_path::feasible_potential;
use graph::algorithm::single_source_shortest_path::dijkstra;

pub fn dijkstra_with_potential<'a, G, W, F>(g: &'a G, cost: F) -> Option<Properties<Properties<W>>>
where G: Directed<'a>, W: ArbWeight, F: Fn(&G::AEType) -> W, <W as ToNNegWeight>::Output: ToArbWeight<Output=W> {
    let n = g.v_size();

    if let Some(pi) = feasible_potential(g, |e| cost(e)) {
        let mut dist = Properties::new(n, &Properties::new(n, &W::inf()));

        for s in g.vertices() {
            let ndist = dijkstra(g, s, |e| (cost(e) + pi[e.from()] - pi[e.to()]).to_nnegw());
            for t in g.vertices() {
                dist[s][t] = ndist[t].to_arbw() + pi[t] - pi[s];
            }
        }

        Some(dist)
    }
    else { None }
}

#[test]
fn dijkstra_with_potential_test() {
    use graph::graph::DirectedGraph;
    use graph::property::ArbW;
    {
        let mut g = DirectedGraph::new(4);
        g.add_edge((0, 1, 1));
        g.add_edge((0, 2, 5));
        g.add_edge((1, 2, 2));
        g.add_edge((1, 3, 4));
        g.add_edge((2, 3, 1));
        g.add_edge((3, 2, 7));
        let dist = dijkstra_with_potential(&g, |e| ArbW::Some(e.edge().2 as isize)).unwrap();
        let ans = vec![vec![0, 1, 3, 4], vec![-1, 0, 2, 3], vec![-1, -1, 0, 1], vec![-1, -1, 7, 0]];
        for u in g.vertices() {
            for v in g.vertices() {
                assert!(dist[u][v] == match ans[*u][*v] { -1 => ArbW::inf(), pos => ArbW::Some(pos) });
            }
        }
    }
}


