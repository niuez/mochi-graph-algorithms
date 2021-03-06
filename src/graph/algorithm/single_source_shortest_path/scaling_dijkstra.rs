use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;
use graph::property::NNegW;

pub fn scaling_dijkstra<'a, G, F>(g: &'a G, s: &G::VType, cost: F) -> Properties<NNegW<usize>>
where G: Graph<'a>, F: Fn(&G::AEType) -> NNegW<usize> { 
    type W = NNegW<usize>;
    let n = g.v_size();
    let mut dist = Properties::new(n, &W::zero());
    let mut mw = W::zero();
    for ref e in g.edges() {
        if mw < cost(e) { mw = cost(e); }
    }

    let logw = {
        let mut cnt = 0usize;
        while mw > W::zero() {
            mw = mw >> 1;
            cnt += 1;
        }
        cnt
    };

    for k in (0..logw+1).rev() {

        for v in g.vertices() {
            dist[v] = dist[v] << 1;
        }

        let mut temp = Properties::new(n, &W::inf());
        temp[s] = W::zero();
        let mut heap = vec![Vec::new(); n];
        heap[0].push(s.clone());

        for d in 0..heap.len() {
            while let Some(v) = heap[d].pop() {
                if temp[&v] < NNegW::Some(d) { continue; }
                for ref e in g.delta(&v) {
                    let c = (cost(e) >> k) + dist[e.from()] - dist[e.to()];
                    if temp[e.from()] + c < temp[e.to()] && temp[e.from()] + c < NNegW::Some(heap.len()) {
                        temp[e.to()] = temp[e.from()] + c;
                        heap[d + match c { NNegW::Some(cc) => cc, _ => unreachable!() }].push(e.to().clone());
                    }
                }
            }
        }

        for v in g.vertices() {
            dist[v] = dist[v] + temp[v];
        }
    }

    dist
}

#[test]
fn scaling_dijkstra_test() {
    use graph::graph::DirectedGraph;
    use graph::property::NNegW;
    let mut g = DirectedGraph::new(4);
    g.add_edge((0, 1, 1));
    g.add_edge((0, 2, 4));
    g.add_edge((2, 0, 1));
    g.add_edge((1, 2, 2));
    g.add_edge((3, 1, 1));
    g.add_edge((3, 2, 5));

    let dist = scaling_dijkstra(&g, &1, |e| NNegW::Some(e.edge().2));
    assert!(dist[&0] == NNegW::Some(3));
    assert!(dist[&1] == NNegW::Some(0));
    assert!(dist[&2] == NNegW::Some(2));
    assert!(dist[&3] == NNegW::Inf);
}

