pub mod ford_fulkerson;

use graph::*;
use graph::directed_graph::*;
use graph::property::*;

pub struct Network<C: Capacity> {
    pub cap: Vec<C>,
    pub flow: Vec<C>,
    pub g: DirectedGraph<usize,Edge>
}

impl<C: Capacity> Network<C> {
    pub fn build<VP: Property, EP: Property, F>(gg: &DirectedGraph<VP,EP>,zero: C,cap_sel: F) -> Network<C>
    where F: Fn(&EP) -> C {
        let mut g = DirectedGraph::<usize,Edge>::new(gg.vertices_cnt(),0);
        let mut cap = Vec::<C>::new();
        let flow = vec![zero;gg.edges_cnt()];

        for i in 0..gg.vertices_cnt() {
            for e in gg.delta(&Vertex(i)) {
                let m = g.edges_cnt();
                g.add_edge(&e.from, &e.to, Edge {from: e.to, to: e.from, index: m + 1});
                g.add_edge(&e.from, &e.to, Edge {from: e.from, to: e.to, index: m});
                cap.push(cap_sel(gg.eprop(e)));
            }
        }

        Network {
            cap: cap,
            flow: flow,
            g: g
        }
    }
}
