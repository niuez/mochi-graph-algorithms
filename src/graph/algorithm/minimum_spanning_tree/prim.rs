use graph::kernel::graph::*;
use graph::kernel::Properties;

use std::collections::BinaryHeap;
use std::cmp::Ordering;


struct PrimNode<W, V>(W, V, usize) where W: std::cmp::Ord, V: Vertex;

impl<W, V> Ord for PrimNode<W, V>
where W: std::cmp::Ord, V: Vertex {
    fn cmp(&self, rhs: &Self) -> Ordering {
        rhs.0.cmp(&self.0)
    }
}

impl<W, V> PartialOrd for PrimNode<W, V>
where W: std::cmp::Ord, V: Vertex {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl<W, V> PartialEq for PrimNode<W, V>
where W: std::cmp::Ord, V: Vertex {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}

impl<W, V> Eq for PrimNode<W, V>
where W: std::cmp::Ord, V: Vertex {}

pub fn prim<'a, G, W, F>(g: &'a G, cost: F) -> Option<Properties<bool>>
where G: Undirected<'a>, W: std::cmp::Ord, F: Fn(&G::AEType) -> W {
    let mut ans = Properties::new(g.e_size(), &false);
    let mut used = Properties::new(g.v_size(), &false);

    match g.vertices().next() {
        Some(s) => {
            let mut heap = BinaryHeap::new();
            let mut cnt = 1;
            used[s] = true;
            for ref e in g.delta(s) {
                heap.push(PrimNode(cost(e), e.to().clone(), e.id()));
            }

            while cnt < g.v_size() {
                match heap.pop() {
                    Some(PrimNode(_, ref v, ref id)) => {
                        ans[id] = true;
                        used[v] = true;
                        cnt += 1;
                        for ref e in g.delta(v) {
                            if !used[e.to()] {
                                heap.push(PrimNode(cost(e), e.to().clone(), e.id()));
                            }
                        }
                    }
                    None => break,
                }
            }
            if cnt == g.v_size() { Some(ans) }
            else { None }
        }
        None => Some(ans)
    }
}

#[test]

fn prim_test() {
    use graph::graph::UndirectedGraph;
    {
        let mut g = UndirectedGraph::new(6);
        g.add_edge((0, 1, 1));
        g.add_edge((0, 2, 3));
        g.add_edge((1, 2, 1));
        g.add_edge((1, 3, 7));
        g.add_edge((2, 4, 1));
        g.add_edge((1, 4, 3));
        g.add_edge((3, 4, 1));
        g.add_edge((3, 5, 1));
        g.add_edge((4, 5, 6));
        let ans = prim(&g, |e| e.edge().2).unwrap();
        let mut res = 0;
        for ref e in g.edges() {
            if ans[e] { res += e.edge().2; }
        }
        assert_eq!(res, 5);
    }
}
