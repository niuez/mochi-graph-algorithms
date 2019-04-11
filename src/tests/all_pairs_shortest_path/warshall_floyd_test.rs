extern crate rand;

use second::*;
use second::directed_graph::*;
use second::all_pairs_shortest_path::warshall_floyd::*;
use second::single_source_shortest_path::dijkstra::*;
use self::rand::Rng;

#[test]
fn warshall_floyd_test() {
    for _ in 0..3 {
        let v = 200;
        let e = 400;
        let mut g = DirectedGraph::<usize, (usize,usize,usize)>::new(v);
        for _ in 0..e {
            let a = rand::thread_rng().gen_range(0, v);
            let b = rand::thread_rng().gen_range(0, v);
            let w = rand::thread_rng().gen_range(1, 1001);
            g.add_edge((a,b,w));
        }
        let di_res = dijkstra_s3p(&g, Vite(0), |w| w.2);
        let wf_res = warshall_floyd_apsp(&g, |w| w.2);
        assert_eq!(di_res, wf_res[0]);
    }
}
