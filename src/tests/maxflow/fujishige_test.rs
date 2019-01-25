extern crate rand;

use graph::*;
use graph::directed_graph::*;
use graph::maxflow::*;
use graph::maxflow::ford_fulkerson::*;
use graph::maxflow::fujishige::*;

use self::rand::Rng;

#[test]
fn fujishige_test() {
    for _ in 0..50 {
        let v = 200;
        let e = 400;
        let mut g = DirectedGraph::<usize,(usize,usize,usize)>::new(v);
        for _ in 0..e {
            let a = rand::thread_rng().gen_range(0, v);
            let b = rand::thread_rng().gen_range(0, v);
            let c = rand::thread_rng().gen_range(0, 100);
            g.add_edge((a,b,c));
        }
        let mut net1 = MFlowNetWork::build(&g, Vite(0), Vite(1), |c| c.2.clone());
        let mut net2 = MFlowNetWork::build(&g, Vite(0), Vite(1), |c| c.2.clone());
        let ans1 = ford_fulkerson_maxflow(&mut net1);
        let ans2 = fujishige_maxflow(&mut net2);
        assert_eq!(ans1, ans2);
    }
}