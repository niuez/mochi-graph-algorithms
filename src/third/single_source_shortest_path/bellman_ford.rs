use third::*;
use third::property::*;


pub fn bellman_ford<'a, V, E, AE, G, W, F>(g: &'a G, s: &V, cost: F) -> Properties<W>
where V: Vertex, E: Edge<VType=V> + 'a, AE: AdjEdge<V, E>, G: Directed<'a, V, E, AE>, W: ArbWeight, F: Fn(&E) -> W {
    let n = g.v_size();
    let mut dist = Properties::new(n, &W::inf());
    dist[s] = W::zero();

    for _ in 0..n {
        for e in g.edges() {
            if dist[e.from()] + cost(e.edge()) < dist[e.to()] {
                dist[e.to()] = dist[e.from()] + cost(e.edge());
            }
        }
    }

    for _ in 0..n {
        for e in g.edges() {
            if dist[e.from()] + cost(e.edge()) < dist[e.to()] {
                dist[e.to()] = W::neg_inf();
            }
        }
    }
    
    dist
}

#[cfg(test)]
mod bellman_ford_test {
    use third::*;
    use third::directed_graph::*;
    use third::single_source_shortest_path::bellman_ford::*;

    #[test]
    fn bellman_ford_test() {
        {
            let mut g = DirectedGraph::new(4);
            g.add_edge((0, 1, ArbW::Some(2)));
            g.add_edge((0, 2, ArbW::Some(3)));
            g.add_edge((1, 2, ArbW::Some(-5)));
            g.add_edge((1, 3, ArbW::Some(1)));
            g.add_edge((2, 3, ArbW::Some(2)));

            let dist = bellman_ford(&g, &0, |e| e.2);
            assert!(dist[&0] == ArbW::Some(0));
            assert!(dist[&1] == ArbW::Some(2));
            assert!(dist[&2] == ArbW::Some(-3));
            assert!(dist[&3] == ArbW::Some(-1));
        }
    }
}

