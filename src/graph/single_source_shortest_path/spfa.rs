use graph::*;
use graph::property::*;

use std::collections::VecDeque;

pub fn spfa_s3p<'a,V,E,G,W,F>(g: &'a G, s: Vite, fp: F) -> Option<Vec<Option<W>>>
where V: Vertex, E: Edge, G: Graph<'a,V,E>, W: Weighted, F: Fn(&E) -> W {
    let n = g.v_size();
    let mut dist = vec![None; n];
    let mut que = VecDeque::new();
    let mut inc = vec![false; n];
    let mut cnt = vec![0; n];
    dist[s.0] = Some(W::zero());
    que.push_back(s);
    inc[s.0] = true;
    cnt[s.0] += 1;
    while let Some(v) = que.pop_back() {
        inc[v.0] = false;
        let d = dist[v.0].unwrap();
        for ei in g.delta(&v) {
            let e = g.edge(ei);
            let to = to(v, e);
            let cost = fp(e);
            if match dist[to.0] {
                Some(d2) => { cost + d < d2 }
                None => true
            } {
                dist[to.0] = Some(cost + d);
                if !inc[to.0] {
                    que.push_back(to);
                    inc[to.0] = true;
                    cnt[to.0] += 1;
                    if cnt[to.0] >= n { return None; }
                }
            }
        }
    }
    Some(dist)
}
