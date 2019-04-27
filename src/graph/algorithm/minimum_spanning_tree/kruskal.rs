use graph::kernel::graph::*;
use graph::kernel::Properties;
use graph::data_structures::UnionFindTree;

pub fn kruskal<'a, G, W, F>(g: &'a G, cost: F) -> Option<Properties<bool>>
where G: Undirected<'a>, W: std::cmp::Ord, F: Fn(&G::AEType) -> W {
    let mut ans = Properties::new(g.e_size(), &false);
    let mut edges = Vec::new();
    let mut cnt = 0;
    let mut uf = UnionFindTree::new(g.v_size());

    for e in g.edges() {
        edges.push(e);
    }
    edges.sort_by(|a, b| cost(a).cmp(&cost(b)));

    for ref e in edges {
        if uf.unite(e.from().id(), e.to().id()).is_some() {
            ans[e] = true;
            cnt += 1;
        }
    }
    if cnt + 1 == g.v_size() { Some(ans) }
    else { None }
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
        let ans = kruskal(&g, |e| e.edge().2).unwrap();
        let mut res = 0;
        for ref e in g.edges() {
            if ans[e] { res += e.edge().2; }
        }
        assert_eq!(res, 5);
    }
}
