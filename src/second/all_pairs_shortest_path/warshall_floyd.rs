use second::*;
use second::property::*;

use std::cmp::min;

pub fn warshall_floyd_apsp<'a,V,E,G,W,F>(g: &'a G,fp: F) -> Vec<Vec<Option<W>>>
where V: Vertex , E: Edge,G: Graph<'a,V,E>, W: Weighted , F: Fn(&E) -> W {
    let n = g.v_size();
    let mut dist = vec![vec![None; n];n];

    for v in 0..n {
        dist[v][v] = Some(W::zero());
        for e in g.delta(&Vite(v)) {
            let u = to(Vite(v), g.edge(e)).0;
            dist[v][u] = match dist[v][u] {
                Some(d) => Some(min(d, fp(g.edge(e)))),
                None => Some(fp(g.edge(e)))
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = match (dist[i][j], dist[i][k], dist[k][j]){
                    (None, Some(d1), Some(d2)) => Some(d1 + d2),
                    (Some(d), Some(d1), Some(d2)) => Some(min(d1 + d2, d)),
                    (d, _, _) => d
                }
            }
        }
    }

    dist
}
