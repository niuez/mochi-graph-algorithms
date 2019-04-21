use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;

pub fn warshall_floyd<'a, G, W, F>(g: &'a G, cost: F) -> Option<Properties<Properties<W>>>
where G: Graph<'a>, W: ArbWeight, F: Fn(&G::AEType) -> W {
    let n = g.v_size();
    let mut dist = Properties::new(n, &Properties::new(n, &W::inf()));

    for v in g.vertices() {
        dist[v][v] = W::zero();
        for ref e in g.delta(v) {
            dist[e.from()][e.to()] = cost(e);
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

#[test]
fn warshall_floyd_test() {
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
        let dist = warshall_floyd(&g, |e| ArbW::Some(e.edge().2)).unwrap();
        let ans = vec![vec![0, 1, 3, 4], vec![-1, 0, 2, 3], vec![-1, -1, 0, 1], vec![-1, -1, 7, 0]];
        for u in g.vertices() {
            for v in g.vertices() {
                assert!(dist[u][v] == match ans[*u][*v] { -1 => ArbW::inf(), pos => ArbW::Some(pos as usize) });
            }
        }
    }
}
