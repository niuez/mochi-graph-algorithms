use graph::*;

use std::collections::vec_deque::*;

pub fn hk_dfs<'a,V,E,G>(g:&'a G, v: &Vite, dist: &mut Vec<i32>, mate: &mut Vec<Option<usize>>, used: &mut Vec<bool>, vis: &mut Vec<bool>) -> bool
where V: Vertex, E: Edge, G: Bipartite<'a,V,E> + Undirected<'a,V,E> {
    vis[v.0] = true;
    for e in g.delta(v) {
        let to = to(*v, g.edge(e));
        let ok = match mate[to.0] {
            Some(m) => {
                if !vis[m] && dist[m] == dist[v.0] + 1 && hk_dfs(g,&Vite(m),dist,mate,used,vis) {
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
            mate[to.0] = Some(v.0);
            used[v.0] = true;
            return true;
        }
    }

    false
}

pub fn hopcroft_karp<'a,V,E,G>(g: &'a G) -> Vec<(Vite,Vite)>
where V: Vertex, E: Edge, G: Bipartite<'a,V,E> + Undirected<'a,V,E> {
    let mut ans = Vec::<(Vite,Vite)>::new();
    let n = g.v_size();
    
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
            for e in g.delta(&Vite(v)) {
                let to = to(Vite(v), g.edge(e));
                if let Some(m) = mate[to.0] {
                    if dist[m] == -1 {
                        dist[m] = dist[v] + 1;
                        que.push_back(m);
                    }
                }
            }
        }

        let mut has_end = true;
        for i in g.left_vs() {
            if !used[i.0] && hk_dfs(g,i,&mut dist,&mut mate,&mut used,&mut vis) {
                has_end = false;
            }
        }
        if has_end {
            break;
        }
    }

    for i in g.right_vs() {
        if let Some(m) = mate[i.0] {
            ans.push((i.clone(),Vite(m)));
        }
    }
    ans
}
