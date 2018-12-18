extern crate rand;

use graph::*;
use graph::directed_graph::*;
use graph::shortest_path::dijkstra::*;
use graph::shortest_path::scaling_dijkstra::*;

use self::rand::Rng;

#[test]
fn scaling_dijkstra_test() {
    for _ in 0..50{
        let v = 1000;
        let e = 3000;
        let mut g = DirectedGraph::<usize,usize>::new(v, 0);
        for _ in 0..e {
            let a = rand::thread_rng().gen_range(0, v);
            let b = rand::thread_rng().gen_range(0, v);
            let w = rand::thread_rng().gen_range(1, 1001);
            g.add_edge(&Vertex(a), &Vertex(b), w);
        }
        let di_res = dijkstra(&g, Vertex(0), |w| w);
        let sc_res = scaling_dijkstra(&g, Vertex(0), |w| w);
        for i in 0..v {
            if di_res[i] != sc_res[i] {
                assert!(false, "not collect");
            }
        }
    }
}
