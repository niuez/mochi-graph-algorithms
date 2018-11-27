use graph::Vertex;
use graph::Graph;
use graph::DirectedGraph;
use graph::property::*;

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum BFResult<W> {
    Some(W),
    NegInf,
    None
}

pub fn bellman_ford<VP,EP,W,F>(g: &DirectedGraph<VP,EP>, s: Vertex, start_prop: W, fp: F) -> Vec<BFResult<W>>
where VP: Property, EP: Property, W: Weighted, F: Fn(&EP) -> &W {
    let n = g.vertices_cnt();
    let mut dist = vec![BFResult::None ; n];
    dist[s.0] = BFResult::Some(start_prop);
    
    for i in 0..n+1 {
        for v in 0..n {
            if let BFResult::Some(dv) = dist[v] {
                for e in g.delta(&Vertex(v)) {
                    dist[e.to.0] = match dist[e.to.0] {
                        BFResult::Some(dt) => {
                            if dv + *fp(g.eprop(e)) < dt {
                                BFResult::Some(dv + *fp(g.eprop(e)))
                            }
                            else {
                                BFResult::Some(dt)
                            }
                        }
                        BFResult::None => {
                            BFResult::Some(dv + *fp(g.eprop(e)))
                        }
                        _ => {
                            BFResult::None
                        }
                    }
                }
            }
        }
    }
    for i in 0..n+1 {
        for v in 0..n {
            if let BFResult::Some(dv) = dist[v] {
                for e in g.delta(&Vertex(v)) {
                    dist[e.to.0] = match dist[e.to.0] {
                        BFResult::Some(dt) => {
                            if dv + *fp(g.eprop(e)) < dt {
                                BFResult::NegInf
                            }
                            else {
                                BFResult::Some(dt)
                            }
                        }
                        BFResult::None => {
                            BFResult::Some(dv + *fp(g.eprop(e)))
                        }
                        BFResult::NegInf => {
                            BFResult::NegInf
                        }
                    }
                }
            }
            else if let BFResult::NegInf = dist[v] {
                for e in g.delta(&Vertex(v)) {
                    dist[e.to.0] = BFResult::NegInf;
                }
            }
        }
    }

    dist
}
