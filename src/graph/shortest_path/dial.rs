use graph::*;
use graph::property::*;
use std::cmp::max;

pub fn dial<'a,VP,EP,G,F>(g: &'a G, s: Vertex,fp: F) -> Vec<Option<usize>>
where VP: Property, EP: Property,G: Directed<'a,VP,EP> + StaticGraph<'a,VP,EP>, F: Fn(&EP) -> &usize {
    let n = g.vertices_cnt();
    let mut mv = usize::zero();
    let mut dist = vec![None;n];
    for i in 0..n {
        for e in g.delta(&Vertex(i)) {
            mv = max(mv, fp(g.eprop(&e)).clone());
        }
    }

    let mut pack = vec![Vec::<Vertex>::new();mv * n + 1];
    dist[s.0] = Some(usize::zero());
    pack[dist[s.0].unwrap()].push(s);

    for d in 0..pack.len() {
        while let Some(v) = pack[d].pop() {
            if dist[v.0].unwrap() < d { continue; }
            for e in g.delta(&v) {
                if dist[e.to.0] == None || dist[e.to.0].unwrap() > d + fp(g.eprop(&e)) {
                    dist[e.to.0] = Some(d + fp(g.eprop(&e)));
                    pack[dist[e.to.0].unwrap()].push(e.to);
                }
            }
        }
    }

    dist
}

