use graph::kernel::graph::*;
use graph::kernel::Properties;
use graph::data_structures::UnionFindTree;

pub fn boruvka<'a, G, W, F>(g: &'a G, cost: F) -> Option<Properties<bool>>
where G: Undirected<'a>, W: std::cmp::Ord, F: Fn(&G::AEType) -> W {
    let mut ans = Properties::new(g.e_size(), &false);
    let mut cnt = 0;
    let mut uf = UnionFindTree::new(g.v_size());

    while cnt < g.v_size() - 1 {
        let mut selected_edge = Vec::new();
        for _ in 0..g.v_size() {
            selected_edge.push(None);
        }

        for v in g.vertices() {
            for e in g.delta(v) {
                if !uf.same(e.from().id(), e.to().id()) {
                    selected_edge[uf.root(v.id())] = match selected_edge[uf.root(v.id())].take() {
                        None => { Some(e) }
                        Some(ref ne) if cost(&e) < cost(ne) => { Some(e) }
                        any => { any }
                    }
                }
            }
        }

        for es in selected_edge {
            if let Some(ref e) = es {
                if !ans[e] {
                    ans[e] = true;
                    cnt += 1;
                    uf.unite(e.from().id(), e.to().id());
                }
            }
        }
    }

    Some(ans)
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
        let ans = boruvka(&g, |e| e.edge().2).unwrap();
        let mut res = 0;
        for ref e in g.edges() {
            if ans[e] { res += e.edge().2; }
        }
        assert_eq!(res, 5);
    }
}
