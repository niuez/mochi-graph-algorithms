use graph::*;
use graph::property::*;

use std::collections::vec_deque::*;
use std::cmp::max;

pub fn scaling_dijkstra<'a,VP,EP,G,F>(g: &'a G, s: Vertex,fp: F) -> Vec<Option<usize>>
where VP: Property , EP: Property,G: Directed<'a,VP,EP> + StaticGraph<'a,VP,EP>, F: Fn(&EP) -> &usize {
    let n = g.vertices_cnt();
    let m = g.edges_cnt();
    let mut mv = usize::zero();
    for i in 0..n {
        for e in g.delta(&Vertex(i)) {
            mv = max(mv, fp(g.eprop(&e)).clone());
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

        let mut vec : Vec<VecDeque<Vertex>> = vec![VecDeque::<Vertex>::new();m + 10];
        vec[0].push_back(s);

        for d in 0..vec.len() {
            while let Some(v) = vec[d].pop_front() {
                if temp[v.0].unwrap() < d { continue; }
                for e in g.delta(&v) {
                    if dist[e.to.0] == None { continue; }
                    let c = (fp(g.eprop(&e)) >> k) + dist[v.0].unwrap() - dist[e.to.0].unwrap();
                    if d + c >= vec.len() { continue; }
                    if temp[e.to.0] == None {
                        temp[e.to.0] = Some(d + c);
                        vec[d + c].push_back(e.to);
                    }
                    else if temp[e.to.0].unwrap() > d + c {
                        temp[e.to.0] = Some(d + c);
                        vec[d + c].push_back(e.to);
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