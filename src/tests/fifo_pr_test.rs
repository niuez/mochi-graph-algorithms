extern crate rand;

use graph::*;
use graph::directed_graph::*;
use graph::network::*;
use graph::network::fifo_preflow_relabel::*;
use graph::network::dinic::*;

use self::rand::Rng;

#[test]
fn dinic_test() {
    for _ in 0..50 {
        let v = 200;
        let e = 400;
        let mut g = DirectedGraph::<usize,usize>::new(v, 0);
        for _ in 0..e {
            let a = rand::thread_rng().gen_range(0, v);
            let b = rand::thread_rng().gen_range(0, v);
            let c = rand::thread_rng().gen_range(0, 100);
            g.add_edge(&Vertex(a), &Vertex(b), c);
        }
        let mut net1 = Network::build(&g, Vertex(0), Vertex(1), |c| c.clone());
        let mut net2 = Network::build(&g, Vertex(0), Vertex(1), |c| c.clone());
        assert!(fifo_preflow_relabel(&mut net1) == dinic(&mut net2, 100 * 400  + 10), "not collect");
    }
}
