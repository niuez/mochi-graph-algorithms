extern crate rand;

use second::*;
use second::directed_graph::*;
use second::single_source_shortest_path::dijkstra::*;
use second::single_source_shortest_path::spfa::*;

use self::rand::Rng;

#[test]
fn spfa_s3p_test() {
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
        let di_res = dijkstra_s3p(&g, Vite(0), |w| w.2);
        let spfa_res = spfa_s3p(&g, Vite(0), |w| w.2).unwrap();
        assert_eq!(di_res, spfa_res);
    }
}
