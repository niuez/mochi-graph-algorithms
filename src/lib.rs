pub mod graph;

extern crate rand;

#[cfg(test)]
mod tests {
    use graph::*;
    use graph::directed_graph::*;
    use graph::shortest_path::bellman_ford::*;
    use graph::shortest_path::dijkstra::*;
    use rand::Rng;

    #[test]
    fn dijkstra_test() {
        let v = 100;
        let e = 300;
        let mut g = DirectedGraph::<usize,usize>::new(v,0);
        for _ in 0..e {
            let a = rand::thread_rng().gen_range(0,v);
            let b = rand::thread_rng().gen_range(0,v);
            let w = rand::thread_rng().gen_range(1,1001);
            g.add_edge(&Vertex(a), &Vertex(b), w);
        }
        let bf_res = bellman_ford(&g, Vertex(0), 0, |w| w);
        let di_res = dijkstra(&g, Vertex(0), 0, |w| w);
        let ans: Vec<Option<usize>> = bf_res.iter().map(|r| {
            match r {
                BFResult::Some(w) => Some(*w),
                _ => None
            }
        }).collect();
        for i in 0..v {
            if di_res[i] != ans[i] {
                assert!(false, "not collect");
            }
        }
    }
}
