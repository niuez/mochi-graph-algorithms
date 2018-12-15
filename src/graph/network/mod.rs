pub mod ford_fulkerson;
pub mod dinic;
pub mod fifo_preflow_relabel;

use graph::*;
use graph::directed_graph::*;
use graph::property::*;

pub struct Network<C: Capacity> {
    pub cap: Vec<C>,
    pub g: DirectedGraph<usize,Edge>,
    pub source: Vertex,
    pub shink: Vertex,
    pub max_flow: C
}

impl<C: Capacity> Network<C> {
    pub fn build<VP: Property, EP: Property, F>(gg: &DirectedGraph<VP,EP>,s: Vertex, t: Vertex,cap_sel: F) -> Network<C>
    where F: Fn(&EP) -> C {
        let mut g = DirectedGraph::<usize,Edge>::new(gg.vertices_cnt(),0);
        let mut cap = Vec::<C>::new();

        for i in 0..gg.vertices_cnt() {
            for e in gg.delta(&Vertex(i)) {
                let m = g.edges_cnt();
                g.add_edge(&e.from, &e.to, Edge {from: e.to, to: e.from, index: m + 1});
                cap.push(cap_sel(gg.eprop(e)));
                g.add_edge(&e.to, &e.from, Edge {from: e.from, to: e.to, index: m});
                cap.push(C::zero());
            }
        }

        Network {
            cap: cap,
            g: g,
            source: s,
            shink: t,
            max_flow: C::zero()
        }
    }
}
