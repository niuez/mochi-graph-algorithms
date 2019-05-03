use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;
use graph::algorithm::single_source_shortest_path::dijkstra;
use graph::property::PathW;
use graph::graph::AntiGraph;

use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct EppisteinNode<W: NNegWeight, V: Vertex> {
    dist: W,
    ver : V,
    sp: bool,
}

impl<W: NNegWeight, V: Vertex> Ord for EppisteinNode<W, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}
impl<W: NNegWeight, V: Vertex> PartialOrd for EppisteinNode<W, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.dist.cmp(&self.dist))
    }
}
impl<W: NNegWeight, V: Vertex> PartialEq for EppisteinNode<W, V> {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl<W: NNegWeight, V: Vertex> Eq for EppisteinNode<W, V> { }


pub fn eppistein<'a, G, W, F>(g: &'a G, s: &G::VType, t: &G::VType, cost: F, k: usize) -> Vec<W>
where G: Graph<'a>, W: NNegWeight + SubtractableWeight, F: Fn(&G::AEType) -> W {
    let mut ans = Vec::new();
    
    let (sp_edge, pi) = {
        let ag = AntiGraph::new(g);
        let dist = dijkstra(&ag, &t, |e| PathW {
            weight: cost(e.edge()),
            before: Some(e.id()),
        });
        let mut mut_sp = Properties::new(g.e_size(), &false);
        let mut mut_pi = Properties::new(g.v_size(), &W::zero());
        for d in dist.iter() {
            if let PathW { weight: _, before: Some(eid) } = d {
                mut_sp[eid] = true;
            }
        }
        for v in g.vertices() { mut_pi[v] = dist[v].weight; }
        (mut_sp, mut_pi)
    };

    let mut heap = BinaryHeap::new();
    heap.push(EppisteinNode { dist: W::zero(), ver: s.clone(), sp: false });

    while let Some(EppisteinNode { dist: d, ver: ref v, sp: is_sp }) = heap.pop() {
        if !is_sp {
            ans.push(d + pi[s]);
            if ans.len() == k + 1 { break; }
        }
        for ref e in g.delta(v) {
            heap.push(EppisteinNode{ dist: d + cost(e) + pi[e.to()] - pi[e.from()], ver: e.to().clone(), sp: sp_edge[e] })
        }
    }

    ans
}

#[test]
fn eppistein_test() {
    use graph::graph::DirectedGraph;
    use graph::property::NNegW;
    let mut g = DirectedGraph::new(6);
    g.add_edge((0, 1, 3usize));
    g.add_edge((1, 2, 4));
    g.add_edge((0, 3, 2));
    g.add_edge((3, 1, 1));
    g.add_edge((3, 2, 2));
    g.add_edge((3, 4, 3));
    g.add_edge((2, 4, 2));
    g.add_edge((2, 5, 1));
    g.add_edge((4, 5, 2));
    let res = eppistein(&g, &0, &5, |e| NNegW::Some(e.edge().2), 2);
    assert_eq!(res.len(), 3);
    assert!(res[0] == NNegW::Some(5));
    assert!(res[1] == NNegW::Some(7));
    assert!(res[2] == NNegW::Some(8));
}
