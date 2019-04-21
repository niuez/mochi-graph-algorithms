use graph::kernel::graph::*;
use graph::kernel::Properties;

use std::collections::vec_deque::*;

pub fn hk_dfs<'a, G>(g: &'a G, v: &G::VType, dist: &mut Properties<Option<usize>>, mate: &mut Properties<Option<G::VType>>, used: &mut Properties<bool>, vis: &mut Properties<bool>) -> bool 
where G: Bipartite<'a> {
    vis[v] = true;
    for ref e in g.delta(v) {
        if match mate[e.to()].clone() {
            Some(ref m) => {
                !vis[m] && 
                match dist[m] { Some(dd) if dd == dist[v].unwrap() + 1 => true, _ => false } &&
                hk_dfs(g, m, dist, mate, used, vis) 
            }
            None => true
        } {
            mate[e.to()] = Some(v.clone());
            used[v] = true;
            return true;
        }
    }

    false
}

pub fn hopcroft_karp<'a, G>(g: &'a G) -> Vec<(G::VType, G::VType)>
where G: Bipartite<'a> {
    let mut ans = Vec::new();
    let n = g.v_size();

    let mut mate = Properties::<Option<G::VType>>::new(n, &None);
    let mut used = Properties::new(n, &false);

    loop {
        let mut vis = Properties::new(n, &false);
        let mut dist = Properties::new(n, &None);
        let mut que = VecDeque::new();
        for s in g.left_vertices() {
            if !used[s] {
                que.push_back(s.clone());
                dist[s] = Some(0);
            }
        }

        while let Some(v) = que.pop_front() {
            for ref e in g.delta(&v) {
                if let Some(m) = mate[e.to()].as_ref() {
                    if dist[m] == None {
                        dist[m] = Some(dist[&v].unwrap() + 1);
                        que.push_back(m.clone());
                    }
                }
            }
        }
         
        let mut has_end = true;
        for v in g.left_vertices() {
            if !used[v] && hk_dfs(g, v, &mut dist, &mut mate, &mut used, &mut vis) {
                has_end = false;
            }
        }
        if has_end { break; }
    }

    for v in g.right_vertices() {
        if let Some(m) = mate[v].take() {
            ans.push((m, v.clone()));
        }
    }

    ans
}

#[test]
fn hopcroft_karp_test() {
    use graph::graph::BipartiteGraph;

    {
        let mut g = BipartiteGraph::new(7);
        g.add_edge((0, 0 + 3));
        g.add_edge((0, 2 + 3));
        g.add_edge((0, 3 + 3));
        g.add_edge((1, 1 + 3));
        g.add_edge((2, 1 + 3));
        g.add_edge((2, 3 + 3));
        assert!(hopcroft_karp(&g).len() == 3);
    }

}
