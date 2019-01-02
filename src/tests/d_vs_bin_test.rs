
extern crate rand;

use graph::directed_graph::*;
use graph::shortest_path::dijkstra::*;
use graph::shortest_path::d_heap_dijkstra::*;

use graph::*;
use self::rand::Rng;

use std::time::Instant;

fn get_graph() -> DirectedGraph<usize,usize> {
    let v = 100000;
    let e = 1000000;
    let mut g = DirectedGraph::<usize, usize>::new(v, 0);
    for _ in 0..e {
        let a = rand::thread_rng().gen_range(0, v);
        let b = rand::thread_rng().gen_range(0, v);
        let w = rand::thread_rng().gen_range(1, 101);
        g.add_edge(&Vertex(a), &Vertex(b), w);
    }
    return g;
}

#[test]
fn d_heap_bench() {
    for _ in 0..10 {
        let g = get_graph();
        let dhs = Instant::now();
        let _ = d_heap_dijkstra(&g, Vertex(0), |w| w);
        let dhe = dhs.elapsed();
        let ds = Instant::now();
        let _ = dijkstra(&g, Vertex(0), |w| w);
        let de = ds.elapsed();
        println!("d_heap   {}.{}",dhe.as_secs(), dhe.subsec_nanos());
        println!("dijkstra {}.{}",de.as_secs(), de.subsec_nanos());
    }
}
