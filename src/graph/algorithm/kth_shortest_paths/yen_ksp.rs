use graph::kernel::graph::*;
use graph::kernel::property::*;
use graph::kernel::Properties;
use graph::algorithm::single_source_shortest_path::dijkstra;
use graph::property::PathW;
use graph::graph::SubEdgeGraph;

pub fn yen_ksp<'a, G, W, F>(g: &'a G, s: &G::VType, t: &G::VType, cost: F, k: usize) -> Vec<(Vec<usize>, W)>
where G: Graph<'a>, W: NNegWeight, F: Fn(&G::AEType) -> W {
    let edges = {
        let mut temp = Vec::new();
        for e in g.edges() {
            temp.push(e);
        }
        temp
    };
    let mut kth = Vec::new();
    let mut que = Vec::new();

    let get_path = | dist: Properties<PathW<W, (G::VType, usize)>> | {
        let mut path = Vec::new();
        let mut u = *t;
        while let PathW { weight: w, before: Some((v, eid)) } = dist[&u] {
            path.push((eid, w, dist[&v].weight));
            u = v;
        }
        path
    };

    kth.push({
        let dist = dijkstra(g, s, |e| PathW {
            weight: cost(e),
            before: Some((e.from().clone(), e.id())),
        });
        let mut rpath = get_path(dist);
        rpath.reverse();
        rpath
    });

    for kk in 0..k {
        let length = kth[kk].len();
        for i in 0..length {
            let mut usable = Properties::new(g.e_size(), &true);
            let spur = edges[kth[kk][i].0].from();
            for p in kth.iter() {
                let mut unusable = true;
                for j in 0..i {
                    if p[j] != kth[kk][j] {
                        unusable = false;
                        break;
                    }
                }
                if unusable {
                    usable[&p[i].0] = false;
                }
            }

            let sg = SubEdgeGraph::new(g, |e| usable[e]);
            let dist = dijkstra(&sg, spur, |e| PathW {
                weight: cost(e),
                before: Some((e.from().clone(), e.id())),
            });
            if dist[t].weight == W::inf() { continue; }
            let mut path = get_path(dist);
            for j in 0..path.len() {
                path[j] = (path[j].0, path[j].1 + kth[kk][i].2, path[j].2 + kth[kk][i].2);
            }
            for j in (0..i).rev() {
                path.push(kth[kk][j]);
            }
            path.reverse();
            que.push(path);
        }

        if que.is_empty() { break; }
        que.sort_by(|a, b| b[b.len() - 1].1.cmp(&a[a.len() - 1].1));
        kth.push(que.pop().unwrap());
    }
    let mut ans = Vec::new();
    for path in kth {
        let mut temp = Vec::new();
        for (eid, _, _) in path.iter() { temp.push(*eid); }
        ans.push((temp, path[path.len() - 1].1));
    }
    ans
}

#[test]
fn yen_ksp_test() {
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
    let res = yen_ksp(&g, &0, &5, |e| NNegW::Some(e.edge().2), 2);
    assert_eq!(res.len(), 3);
    assert!(res[0].1 == NNegW::Some(5));
    assert!(res[1].1 == NNegW::Some(7));
    assert!(res[2].1 == NNegW::Some(8));
}
