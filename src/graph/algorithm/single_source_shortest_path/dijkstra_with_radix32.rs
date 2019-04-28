use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;
use graph::property::NNegW;
use graph::data_structures::RadixHeap32;

pub fn dijkstra_with_radix32<'a, G, F>(g: &'a G, s: &G::VType, cost: F) -> Properties<NNegW<u32>>
where G: Graph<'a>, F: Fn(&G::AEType) -> NNegW<u32> { 
    type W = NNegW<u32>;
    let n = g.v_size();
    let mut dist = Properties::new(n, &W::inf());
    dist[s] = W::zero();

    let mut heap = RadixHeap32::new();
    heap.push((match dist[s] { NNegW::Some(raw) => raw, _ => unreachable!()}, s.clone()));

    while let Some((raw, ref v)) = heap.pop() {
        if dist[v] < NNegW::Some(raw) { continue }
        for ref e in g.delta(v) {
            if dist[e.from()] + cost(e) < dist[e.to()] {
                dist[e.to()] = dist[e.from()] + cost(e);
                heap.push((match dist[e.to()] { NNegW::Some(raw) => raw, _ => unreachable!()}, e.to().clone()));
            }
        }
    }

    dist
}

#[test]
fn dijkstra_test() {
    use graph::graph::DirectedGraph;
    use graph::property::NNegW;
    let mut g = DirectedGraph::new(4);
    g.add_edge((0usize, 1usize, 1u32));
    g.add_edge((0, 2, 4));
    g.add_edge((2, 0, 1));
    g.add_edge((1, 2, 2));
    g.add_edge((3, 1, 1));
    g.add_edge((3, 2, 5));

    let dist = dijkstra_with_radix32(&g, &1, |e| NNegW::Some(e.edge().2));
    assert!(dist[&0] == NNegW::Some(3));
    assert!(dist[&1] == NNegW::Some(0));
    assert!(dist[&2] == NNegW::Some(2));
    assert!(dist[&3] == NNegW::Inf);
}

