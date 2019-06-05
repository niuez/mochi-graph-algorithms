#[test]
fn ziman() {
    use graph::kernel::graph::*;
    use graph::graph::{ AntiGraph, SubEdgeGraph, DirectedGraph };
    use graph::property::NNegW;
    use graph::algorithm::single_source_shortest_path::dijkstra;
    let mut g = DirectedGraph::new(10);
    g.add_edge((0, 1, NNegW::Some(5usize)));
    g.add_edge((0, 2, NNegW::Some(3)));
    g.add_edge((1, 2, NNegW::Some(4)));
    g.add_edge((1, 3, NNegW::Some(9)));
    g.add_edge((2, 3, NNegW::Some(2)));
    g.add_edge((2, 4, NNegW::Some(1)));
    g.add_edge((3, 5, NNegW::Some(4)));
    g.add_edge((4, 5, NNegW::Some(4)));
    g.add_edge((2, 5, NNegW::Some(6)));
    let rev_dist = dijkstra(
        &AntiGraph::new(
            &SubEdgeGraph::new(
                &g, |ae| ae.edge().2 >= NNegW::Some(4)
                )
            )
        , &5, |ae| ae.edge().edge().2);
    assert!(rev_dist[&0] == NNegW::Some(15));
}
