use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;

pub fn feasible_potential<'a, G, W, F>(g: &'a G, cost: F) -> Option<Properties<W>>
where G: Directed<'a>, W: ArbWeight, F: Fn(&G::EType) -> W {
    let n = g.v_size();
    let mut dist = Properties::new(n, &W::zero());

    for i in 0..n + 1 {
        for e in g.edges() {
            if dist[e.from()] + cost(e.edge()) < dist[e.to()] {
                if i == n { return None; }
                dist[e.to()] = dist[e.from()] + cost(e.edge());
            }
        }
    }

    Some(dist)
}
