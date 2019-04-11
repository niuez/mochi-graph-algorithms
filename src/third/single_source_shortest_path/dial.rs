use third::*;
use third::property::*;

use std::collections::vec_deque::*;

pub fn dial<'a, V, E, AE, G, F>(g: &'a G, s: &V, cost: F) -> Properties<NNegW<usize>>
where V: Vertex + 'a, E: Edge<VType=V> + 'a, AE: AdjEdge<V, E>, G: Graph<'a, V, E, AE>, F: Fn(&E) -> NNegW<usize> { 
    type W = NNegW<usize>;
    let n = g.v_size();
    let mut dist = Properties::new(n, &W::inf());
    dist[s] = W::zero();
    let mut mw = W::zero();
    for e in g.edges() { 
        if mw < cost(e.edge()) { mw = cost(e.edge()); }
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
            for e in g.delta(&v) { 
                if dist[e.from()] + cost(e.edge()) < dist[e.to()] {
                    dist[e.to()] = dist[e.from()] + cost(e.edge());
                    que[ match dist[e.to()] { NNegW::Some(dt) => dt, _ => unreachable!() } ].push_back((e.to().clone(), dist[e.to()]));
                }
            }
        }
    }
    dist
    
}

#[cfg(test)]
mod dial_test {
    use third::directed_graph::*;
    use third::single_source_shortest_path::dial::*;

    #[test]
    fn dial_test() {
        let mut g = DirectedGraph::new(4);
        g.add_edge((0, 1, 1));
        g.add_edge((0, 2, 4));
        g.add_edge((2, 0, 1));
        g.add_edge((1, 2, 2));
        g.add_edge((3, 1, 1));
        g.add_edge((3, 2, 5));

        let dist = dial(&g, &1, |e| NNegW::Some(e.2 as usize));
        assert!(dist[&0] == NNegW::Some(3));
        assert!(dist[&1] == NNegW::Some(0));
        assert!(dist[&2] == NNegW::Some(2));
        assert!(dist[&3] == NNegW::Inf);
    }
}

