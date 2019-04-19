use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;
use graph::property::NNegW;

use std::collections::VecDeque;

pub fn bfs<'a, G>(g: &'a G, s: &G::VType) -> Properties<NNegW<usize>> where G: Graph<'a> {
    let mut dist = Properties::new(g.v_size(), &NNegW::inf());
    dist[s] = NNegW::zero();
    let mut que = VecDeque::new();
    que.push_back(s.clone());
    while let Some(ref v) = que.pop_front() {
        for ref e in g.delta(v) {
            if dist[e.from()] + NNegW::Some(1) < dist[e.to()] {
                dist[e.to()] = dist[e.from()] + NNegW::Some(1);
                que.push_back(e.to().clone());
            }
        }
    }

    dist 
}
