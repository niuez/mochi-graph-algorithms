pub mod dinic;
pub mod fifo_push_relabel;
pub mod ford_fulkerson;
pub mod fujishige;

use graph::*;
use graph::directed_graph::*;
use graph::property::*;

pub struct MFlowV {
    pub i: usize
}

impl Vertex for MFlowV {
    fn new(id: usize) -> Self {
        MFlowV { i: id }
    }
    fn id(&self) -> usize {
        self.i
    }
}

pub struct MFlowE {
    pub from: Vite,
    pub to: Vite,
    pub rev: Eite
}

impl Edge for MFlowE {
    fn from(&self) -> Vite { self.from }
    fn to(&self) -> Vite { self.to }
}

pub struct MFlowNetWork<C: Capacity> {
    pub cap: Vec<C>,
    pub g: DirectedGraph<MFlowV,MFlowE>,
    pub source: Vite,
    pub shink: Vite,
    pub max_flow: C
}

impl<C: Capacity> MFlowNetWork<C> {
    pub fn build<V,E,CF>(gg: &DirectedGraph<V,E>, s: Vite, t: Vite, capf: CF) -> MFlowNetWork<C>
        where V: Vertex, E: Edge, CF: Fn(&E) -> C {
            let mut g = DirectedGraph::<MFlowV,MFlowE>::new(gg.v_size());
            let mut cap = Vec::<C>::new();
            for i in 0..gg.v_size() {
                for ei in gg.delta(&Vite(i)) {
                    let e = gg.edge(ei);
                    let from = Vite(i);
                    let to = to(Vite(i), e);
                    let m = g.e_size();
                    g.add_edge(MFlowE{ from: from, to: to, rev: Eite(m + 1) });
                    cap.push(capf(e));
                    g.add_edge(MFlowE{ from: to, to: from, rev: Eite(m) });
                    cap.push(C::zero());
                }
            }
            MFlowNetWork {
                cap: cap,
                g: g,
                source: s,
                shink: t,
                max_flow: C::zero()
            }
        }
}
