extern crate rand;

use graph::*;
use graph::directed_graph::*;
use graph::single_source_shortest_path::bellman_ford::*;
use graph::single_source_shortest_path::dial::*;

use self::rand::Rng;

#[test]
fn dijkstra_test() {
    for _ in 0..100 {
        let v = 200;
        let e = 400;
        let mut g = DirectedGraph::<usize, (usize,usize,usize)>::new(v);
        for _ in 0..e {
            let a = rand::thread_rng().gen_range(0, v);
            let b = rand::thread_rng().gen_range(0, v);
            let w = rand::thread_rng().gen_range(1, 100);
            g.add_edge((a,b,w));
        }
        let bf_res = bellman_ford_s3p(&g, Vite(0), |w| w.2);
        let di_res = dial_s3p(&g, Vite(0), |w| w.2);
        let ans: Vec<Option<usize>> = bf_res
            .iter()
            .map(|r| match r {
                BFResult::Some(w) => Some(*w),
                _ => None,
            }).collect();
        for i in 0..v {
            if di_res[i] != ans[i] {
                assert!(false, "not collect");
            }
        }
    }
}
