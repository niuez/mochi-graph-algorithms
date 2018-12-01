use graph::*;
use graph::property::*;

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum BFResult<W> {
    Some(W),
    NegInf,
    None
}

/// return distances from s to all vertices in graph by Bellman-Ford Algorithm.
/// running time O(EV)
/// if a vertex cannot reach from s, result of the vertex is `None`,
/// else if the distance of the vertex can be -INF , result is `NegInf` and
/// else result is `Some(distance)`
pub fn bellman_ford<'a,VP,EP,G,W,F>(g: &'a G, s: Vertex, start_prop: W, fp: F) -> Vec<BFResult<W>>
where VP: Property, EP: Property,G: Directed<'a,VP,EP>, W: Weighted, F: Fn(&EP) -> &W {
    let n = g.vertices_cnt();
    let mut dist = vec![BFResult::None ; n];
    dist[s.0] = BFResult::Some(start_prop);
    
    for _ in 0..n+1 {
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
    for _ in 0..n+1 {
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
