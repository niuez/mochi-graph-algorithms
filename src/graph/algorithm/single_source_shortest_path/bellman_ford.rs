use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::property::Properties;


pub fn bellman_ford<'a, G, W, F>(g: &'a G, s: &G::VType, cost: F) -> Properties<W>
where G: Directed<'a>, W: ArbWeight, F: Fn(&G::EType) -> W {
    let n = g.v_size();
    let mut dist = Properties::new(n, &W::inf());
    dist[s] = W::zero();

    for _ in 0..n {
        for e in g.edges() {
            if dist[e.from()] + cost(e.edge()) < dist[e.to()] {
                dist[e.to()] = dist[e.from()] + cost(e.edge());
            }
        }
    }

    for _ in 0..n {
        for e in g.edges() {
            dist[e.to()] = 
                if dist[e.from()] == W::neg_inf() { W::neg_inf() }
                else if dist[e.from()] + cost(e.edge()) < dist[e.to()] { W::neg_inf() }
                else { dist[e.to()] }
        }
    }
    
    dist
}

#[test]
fn bellman_ford_test() {
    use graph::graph::DirectedGraph;
    use graph::property::ArbW;
    {
        let mut g = DirectedGraph::new(4);
        g.add_edge((0, 1, ArbW::Some(2)));
        g.add_edge((0, 2, ArbW::Some(3)));
        g.add_edge((1, 2, ArbW::Some(-5)));
        g.add_edge((1, 3, ArbW::Some(1)));
        g.add_edge((2, 3, ArbW::Some(2)));

        let dist = bellman_ford(&g, &0, |e| e.2);
        assert!(dist[&0] == ArbW::Some(0));
        assert!(dist[&1] == ArbW::Some(2));
        assert!(dist[&2] == ArbW::Some(-3));
        assert!(dist[&3] == ArbW::Some(-1));
    }
    {
        let mut g = DirectedGraph::new(4);
        g.add_edge((0, 1, 2));
        g.add_edge((0, 2, 3));
        g.add_edge((1, 2, -5));
        g.add_edge((1, 3, 1));
        g.add_edge((2, 3, 2));
        g.add_edge((3, 1, 0));

        let dist = bellman_ford(&g, &0, |e| ArbW::Some(e.2));

        assert!(dist[&0] == ArbW::Some(0));
        assert!(dist[&1] == ArbW::neg_inf());
        assert!(dist[&2] == ArbW::neg_inf());
        assert!(dist[&3] == ArbW::neg_inf());
    }
}

