use graph::*;
use graph::property::*;

pub fn warshall_floyd<'a,V,E,AE,G,W,F>(g: &'a G, cost: F) -> Option<Properties<Properties<W>>>
where V: Vertex + 'a, E: Edge<VType=V> + 'a, AE: AdjEdge<V, E>, G: Graph<'a, V, E, AE>, W: ArbWeight, F: Fn(&E) -> W {
    let n = g.v_size();
    let mut dist = Properties::new(n, &Properties::new(n, &W::inf()));

    for v in g.vertices() {
        dist[v][v] = W::zero();
        for e in g.delta(v) {
            dist[e.from()][e.to()] = cost(e.edge());
        }
    }

    for k in g.vertices() {
        for i in g.vertices() {
            for j in g.vertices() {
                if dist[i][k] + dist[k][j] < dist[i][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    for v in g.vertices() {
        if dist[v][v] < W::zero() { return None }
    }

    Some(dist)
}

#[cfg(test)]
mod warshall_floyd_test {
    use graph::directed_graph::*;
    use graph::all_pairs_shortest_path::warshall_floyd::*;
    #[test]
    fn warshall_floyd_test() {
        {
            let mut g = DirectedGraph::new(4);
            g.add_edge((0, 1, 1));
            g.add_edge((0, 2, 5));
            g.add_edge((1, 2, 2));
            g.add_edge((1, 3, 4));
            g.add_edge((2, 3, 1));
            g.add_edge((3, 2, 7));
            let dist = warshall_floyd(&g, |e| ArbW::Some(e.2)).unwrap();
            let ans = vec![vec![0, 1, 3, 4], vec![-1, 0, 2, 3], vec![-1, -1, 0, 1], vec![-1, -1, 7, 0]];
            for u in g.vertices() {
                for v in g.vertices() {
                    assert!(dist[u][v] == match ans[*u][*v] { -1 => ArbW::inf(), pos => ArbW::Some(pos as usize) });
                }
            }
        }
    }
}
