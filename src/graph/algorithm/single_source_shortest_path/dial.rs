use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;
use graph::property::NNegW;

use std::collections::vec_deque::*;

pub fn dial<'a, G, F>(g: &'a G, s: &G::VType, cost: F) -> Properties<NNegW<usize>>
where G: Graph<'a>, F: Fn(&G::AEType) -> NNegW<usize> { 
    type W = NNegW<usize>;
    let n = g.v_size();
    let mut dist = Properties::new(n, &W::inf());
    dist[s] = W::zero();
    let mut mw = W::zero();
    for ref e in g.edges() { 
        if mw < cost(e) { mw = cost(e); }
    }

    let mut que = match mw {
         NNegW::Some(d) => vec![VecDeque::new(); d + 1], 
         _ => unimplemented!()
    };
    que[0].push_back((s.clone(), NNegW::Some(0usize)));

    let len = que.len();
    for dd in 0..len * n {
        while let Some((v, d)) = que[dd % len].pop_back() {
            if dist[&v] < d { continue }
            for ref e in g.delta(&v) { 
                if dist[e.from()] + cost(e) < dist[e.to()] {
                    dist[e.to()] = dist[e.from()] + cost(e);
                    que[ match dist[e.to()] { NNegW::Some(dt) => dt, _ => unreachable!() } ].push_back((e.to().clone(), dist[e.to()]));
                }
            }
        }
    }
    dist
    
}

#[test]
fn dial_test() {
    use graph::graph::DirectedGraph;
    use graph::property::NNegW;
    let mut g = DirectedGraph::new(4);
    g.add_edge((0, 1, 1));
    g.add_edge((0, 2, 4));
    g.add_edge((2, 0, 1));
    g.add_edge((1, 2, 2));
    g.add_edge((3, 1, 1));
    g.add_edge((3, 2, 5));

    let dist = dial(&g, &1, |e| NNegW::Some(e.edge().2 as usize));
    assert!(dist[&0] == NNegW::Some(3));
    assert!(dist[&1] == NNegW::Some(0));
    assert!(dist[&2] == NNegW::Some(2));
    assert!(dist[&3] == NNegW::Inf);
}

