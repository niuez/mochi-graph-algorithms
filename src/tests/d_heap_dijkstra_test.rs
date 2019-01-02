extern crate rand;

use graph::directed_graph::*;
use graph::shortest_path::dijkstra::*;
use graph::shortest_path::d_heap_dijkstra::*;

use graph::*;
use self::rand::Rng;

#[test]
fn d_heap_dijkstra_test() {
    for _ in 0..10 {
        let v = 1000;
        let e = 5000;
        let mut g = DirectedGraph::<usize, usize>::new(v, 0);
        for _ in 0..e {
            let a = rand::thread_rng().gen_range(0, v);
            let b = rand::thread_rng().gen_range(0, v);
            let w = rand::thread_rng().gen_range(1, 1001);
            g.add_edge(&Vertex(a), &Vertex(b), w);
        }
        let dh_res = d_heap_dijkstra(&g, Vertex(0), |w| w);
        let di_res = dijkstra(&g, Vertex(0), |w| w);
        for i in 0..v {
            if di_res[i] != dh_res[i] {
                assert!(false, "not collect");
            }
        }
    }
}

use std::time::Instant;

#[test]
fn d_heap_bench() {
    let start = Instant::now();
    for _ in 0..10 {
        let v = 1000;
        let e = 5000;
        let mut g = DirectedGraph::<usize, usize>::new(v, 0);
        for _ in 0..e {
            let a = rand::thread_rng().gen_range(0, v);
            let b = rand::thread_rng().gen_range(0, v);
            let w = rand::thread_rng().gen_range(1, 1001);
            g.add_edge(&Vertex(a), &Vertex(b), w);
        }
        let dh_res = d_heap_dijkstra(&g, Vertex(0), |w| w);
        let di_res = dijkstra(&g, Vertex(0), |w| w);
        for i in 0..v {
            if di_res[i] != dh_res[i] {
                assert!(false, "not collect");
            }
        }
    }
    let end = start.elapsed();
    println!("d_heap {}.{}",end.as_secs(), end.subsec_millis());
}
