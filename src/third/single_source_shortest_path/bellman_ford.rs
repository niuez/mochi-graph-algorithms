use third::*;
use third::property::*;

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum BFResult<W> {
    Some(W),
    NegInf,
    None
}

pub fn bellman_ford<'a, V, E, AE, G, W, F>(g: &'a G, s: &V, cost: F) -> Properties<BFResult<W>>
where V: Vertex, E: Edge<VType=V> + 'a, AE: AdjEdge<V, E>, G: Directed<'a, V, E, AE>, W: Weight, F: Fn(&E) -> W {
    let n = g.v_size();
    let mut dist = Properties::new(n, &BFResult::None);
    dist[s] = BFResult::Some(W::zero());

    for _ in 0..n {
        for e in g.edges() {
            if let BFResult::Some(d) = dist[e.from()] {
                dist[e.to()] = match dist[e.to()] {
                    BFResult::Some(dt) => {
                        if d + cost(e.edge()) < dt { BFResult::Some(d + cost(e.edge())) }
                        else { BFResult::Some(dt) }
                    }
                    _ => BFResult::Some(d + cost(e.edge())),
                }
            }
        }
    }

    for _ in 0..n {
        for e in g.edges() {
            dist[e.to()] = match dist[e.from()] {
                BFResult::Some(d) => match dist[e.to()] {
                    BFResult::Some(dt) => {
                        if d + cost(e.edge()) < dt { BFResult::NegInf }
                        else { BFResult::Some(dt) }
                    }
                    BFResult::None => BFResult::Some(d + cost(e.edge())),
                    BFResult::NegInf => BFResult::NegInf,
                }
                BFResult::None => dist[e.to()],
                BFResult::NegInf => BFResult::NegInf,
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
            g.add_edge((0, 1, 2));
            g.add_edge((0, 2, 3));
            g.add_edge((1, 2, -5));
            g.add_edge((1, 3, 1));
            g.add_edge((2, 3, 2));

            let dist = bellman_ford(&g, &0, |e| e.2);
            assert!(dist[&0] == BFResult::Some(0));
            assert!(dist[&1] == BFResult::Some(2));
            assert!(dist[&2] == BFResult::Some(-3));
            assert!(dist[&3] == BFResult::Some(-1));
        }
    }
}

