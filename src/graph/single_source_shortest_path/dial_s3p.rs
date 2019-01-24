use graph::*;
use graph::property::*;
use std::cmp::max;

pub fn dial<'a,V,E,G,F>(g: &'a G, s: Vite,fp: F) -> Vec<Option<usize>>
where V: Vertex, E: Edge, G: Graph<'a,V,E>, F: Fn(&E) -> usize {
    let n = g.v_size();
    let mut mv = usize::zero();
    let mut dist = vec![None;n];
    for i in 0..n {
        for e in g.delta(&Vite(i)) {
            mv = max(mv, fp(g.edge(e)));
        }
    }

    let mut pack = vec![Vec::<Vite>::new();mv * n + 1];
    dist[s.0] = Some(usize::zero());
    pack[dist[s.0].unwrap()].push(s);

    for d in 0..pack.len() {
        while let Some(v) = pack[d].pop() {
            if dist[v.0].unwrap() < d { continue; }
            for ei in g.delta(&v) {
                let e = g.edge(ei);
                let to = to(v, e);
                if dist[to.0] == None || dist[to.0].unwrap() > d + fp(e) {
                    dist[to.0] = Some(d + fp(e));
                    pack[dist[to.0].unwrap()].push(to);
                }
            }
        }
    }

    dist
}

