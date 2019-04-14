use graph::*;
use graph::property::*;


pub fn feasible_potential<'a, V, E, AE, G, W, F>(g: &'a G, cost: F) -> Option<Properties<W>>
where V: Vertex + 'a, E: Edge<VType=V> + 'a, AE: AdjEdge<V, E>, G: Directed<'a, V, E, AE>, W: ArbWeight, F: Fn(&E) -> W {
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
