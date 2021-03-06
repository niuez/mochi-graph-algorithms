use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;

use std::collections::VecDeque;

pub fn spfa<'a, G, W, F>(g: &'a G, s: &G::VType, cost: F) -> Option<Properties<W>>
where G: Directed<'a>, W: ArbWeight, F: Fn(&G::AEType) -> W {
    let n = g.v_size();
    let mut dist = Properties::new(n, &W::inf());
    let mut que = VecDeque::new();
    let mut inc = Properties::new(n, &false);
    let mut cnt = Properties::new(n, &0);

    dist[s] = W::zero();
    que.push_back(s.clone());
    inc[s] = true;
    cnt[s] += 1;

    while let Some(v) = que.pop_back() {
        inc[&v] = false;
        for ref e in g.delta(&v) {
            if dist[e.from()] + cost(e) < dist[e.to()] {
                dist[e.to()] = dist[e.from()] + cost(e);
                if !inc[e.to()] {
                    que.push_back(e.to().clone());
                    inc[e.to()] = true;
                    cnt[e.to()] += 1;
                    if cnt[e.to()] >= n { return None; }
                }
            }
        }
    }

    Some(dist)
} 

#[test]
fn spfa_test() {
    use graph::graph::DirectedGraph;
    use graph::property::ArbW;
    {
        let mut g = DirectedGraph::new(4);
        g.add_edge((0, 1, 2));
        g.add_edge((0, 2, 3));
        g.add_edge((1, 2, -5));
        g.add_edge((1, 3, 1));
        g.add_edge((2, 3, 2));

        let dist = spfa(&g, &0, |e| ArbW::Some(e.edge().2)).unwrap();
        assert!(dist[&0] == ArbW::Some(0));
        assert!(dist[&1] == ArbW::Some(2));
        assert!(dist[&2] == ArbW::Some(-3));
        assert!(dist[&3] == ArbW::Some(-1));
    }
    {
        let mut g = DirectedGraph::new(4);
        g.add_edge((0, 1, 2));
        g.add_edge((0, 2, 3));
        g.add_edge((1, 2, -5));
        g.add_edge((1, 3, 1));
        g.add_edge((2, 3, 2));
        g.add_edge((3, 1, 0));

        assert!(spfa(&g, &0, |e| ArbW::Some(e.edge().2)).is_none());
    }
}

