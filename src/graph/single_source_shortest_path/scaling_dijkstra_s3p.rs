use graph::*;
use graph::property::*;

use std::collections::vec_deque::*;
use std::cmp::max;

pub fn scaling_dijkstra<'a,V,E,G,F>(g: &'a G, s: Vite,fp: F) -> Vec<Option<usize>>
where V: Vertex ,E: Edge,G: Graph<'a,V,E>, F: Fn(&E) -> usize {
    let n = g.v_size();
    let m = g.e_size();
    let mut mv = usize::zero();
    for i in 0..n {
        for e in g.delta(&Vite(i)) {
            mv = max(mv, fp(g.edge(e)));
        }
    }
    let mut cnt = 0usize;
    while mv > usize::zero() {
        mv = mv >> 1;
        cnt += 1;
    }

    let mut dist = vec![Some(usize::zero()); n];

    for k in (0..cnt+1).rev() {
        for i in 0..n {
            dist[i] = match dist[i] {
                Some(d) => Some(d * 2),
                None => None
            }
        }
        let mut temp = vec![None; n];
        temp[s.0] = Some(usize::zero());

        let mut vec : Vec<VecDeque<Vite>> = vec![VecDeque::<Vite>::new();m + 10];
        vec[0].push_back(s);

        for d in 0..vec.len() {
            while let Some(v) = vec[d].pop_front() {
                if temp[v.0].unwrap() < d { continue; }
                for ei in g.delta(&v) {
                    let e = g.edge(ei);
                    let to = to(v, e);
                    if dist[to.0] == None { continue; }
                    let c = (fp(e) >> k) + dist[v.0].unwrap() - dist[to.0].unwrap();
                    if d + c >= vec.len() { continue; }
                    if temp[to.0] == None {
                        temp[to.0] = Some(d + c);
                        vec[d + c].push_back(to);
                    }
                    else if temp[to.0].unwrap() > d + c {
                        temp[to.0] = Some(d + c);
                        vec[d + c].push_back(to);
                    }
                }
            }
        }

        for i in 0..n {
            dist[i] = if let (Some(d) , Some(t)) = (dist[i] , temp[i]) {
                Some(d + t)
            }
            else {
                None
            }
        }
    }
    dist
}
