use graph::*;
use graph::property::*;

use std::collections::vec_deque::*;

pub fn hk_dfs<'a,VP,EP,G>(g:&'a G, v: &Vertex, dist: &mut Vec<i32>, mate: &mut Vec<Option<usize>>, used: &mut Vec<bool>, vis: &mut Vec<bool>) -> bool
where VP: Property, EP: Property, G: Bipartite<'a,VP,EP> + StaticGraph<'a,VP,EP>{
    vis[v.0] = true;
    for e in g.delta(v) {
        let ok = match mate[e.to.0] {
            Some(m) => {
                if !vis[m] && dist[m] == dist[v.0] + 1 && hk_dfs(g,&Vertex(m),dist,mate,used,vis) {
                    true
                }
                else {
                    false
                }
            }
            None => {
                true
            }
        };
        if ok {
            mate[e.to.0] = Some(v.0);
            used[v.0] = true;
            return true;
        }
    }

    false
}

pub fn hopcroft_karp<'a,VP,EP,G>(g: &'a G) -> Vec<(Vertex,Vertex)>
where VP: Property, EP: Property, G: Bipartite<'a,VP,EP> + StaticGraph<'a,VP,EP> {
    let mut ans = Vec::<(Vertex,Vertex)>::new();
    let n = g.vertices_cnt();
    
    let mut mate: Vec<Option<usize>> = vec![None;n];
    let mut used = vec![false;n];

    loop {
        let mut vis = vec![false;n];
        let mut dist = vec![-1;n];
        let mut que = VecDeque::new();

        for i in 0..n {
            if !used[i] {
                que.push_back(i);
                dist[i] = 0;
            }
        }

        while let Some(v) = que.pop_front() {
            for e in g.delta(&Vertex(v)) {
                if let Some(m) = mate[e.to.0] {
                    if dist[m] == -1 {
                        dist[m] = dist[v] + 1;
                        que.push_back(m);
                    }
                }
            }
        }

        let mut has_end = true;
        for i in g.left_vertices() {
            if !used[i.0] && hk_dfs(g,i,&mut dist,&mut mate,&mut used,&mut vis) {
                has_end = false;
            }
        }
        if has_end {
            break;
        }
    }

    for i in g.right_vertices() {
        if let Some(m) = mate[i.0] {
            ans.push((i.clone(),Vertex(m)));
        }
    }
    ans
}
