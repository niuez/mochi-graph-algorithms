use third::*;
use third::property::*;

use std::collections::VecDeque;

pub fn spfa<'a, V, E, AE, G, W, F>(g: &'a G, s: &V, cost: F) -> Option<Properties<Option<W>>>
where V: Vertex, E: Edge<VType=V> + 'a, AE: AdjEdge<V, E>, G: Directed<'a, V, E, AE>, W: Weight, F: Fn(&E) -> W {
    let n = g.v_size();
    let mut dist = Properties::new(n, &None);
    let mut que = VecDeque::new();
    let mut inc = Properties::new(n, &false);
    let mut cnt = Properties::new(n, &0);

    dist[s] = Some(W::zero());
    que.push_back(s.clone());
    inc[s] = true;
    cnt[s] += 1;

    while let Some(v) = que.pop_back() {
        inc[&v] = false;
        let d = dist[&v].unwrap();
        for e in g.delta(&v) {
            if match dist[e.to()] {
                Some(dt) => d + cost(e.edge()) < dt,
                None => true,
            } {
                dist[e.to()] = Some(d + cost(e.edge()));
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

#[cfg(test)]
mod spfa_test {
    use third::*;
    use third::directed_graph::*;
    use third::single_source_shortest_path::spfa::*;

    #[test]
    fn spfa_test() {
        {
            let mut g = DirectedGraph::new(4);
            g.add_edge((0, 1, 2));
            g.add_edge((0, 2, 3));
            g.add_edge((1, 2, -5));
            g.add_edge((1, 3, 1));
            g.add_edge((2, 3, 2));

            let dist = spfa(&g, &0, |e| e.2).unwrap();
            assert!(dist[&0] == Some(0));
            assert!(dist[&1] == Some(2));
            assert!(dist[&2] == Some(-3));
            assert!(dist[&3] == Some(-1));
        }
        {
            let mut g = DirectedGraph::new(4);
            g.add_edge((0, 1, 2));
            g.add_edge((0, 2, 3));
            g.add_edge((1, 2, -5));
            g.add_edge((1, 3, 1));
            g.add_edge((2, 3, 2));
            g.add_edge((3, 1, 0));

            assert!(spfa(&g, &0, |e| e.2).is_none());
        }
    }
}

