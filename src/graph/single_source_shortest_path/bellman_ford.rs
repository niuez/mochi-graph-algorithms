use graph::*;
use graph::property::*;

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum BFResult<W> {
    Some(W),
    NegInf,
    None
}

pub fn bellman_ford_s3p<'a,V,E,G,W,F>(g: &'a G, s: Vite, fp: F) -> Vec<BFResult<W>>
where V: Vertex, E: Edge,G: Directed<'a,V,E>, W: Weighted, F: Fn(&E) -> W {
    let n = g.v_size();
    let mut dist = vec![BFResult::None ; n];
    dist[s.0] = BFResult::Some(W::zero());
    
    for _ in 0..n+1 {
        for v in 0..n {
            if let BFResult::Some(dv) = dist[v] {
                for ei in g.delta(&Vite(v)) {
                    let e = g.edge(ei);
                    let to = to(Vite(v), e);
                    dist[to.0] = match dist[to.0] {
                        BFResult::Some(dt) => {
                            if dv + fp(e) < dt {
                                BFResult::Some(dv + fp(e))
                            }
                            else {
                                BFResult::Some(dt)
                            }
                        }
                        BFResult::None => {
                            BFResult::Some(dv + fp(e))
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
                for ei in g.delta(&Vite(v)) {
                    let e = g.edge(ei);
                    let to = to(Vite(v) , e);
                    dist[to.0] = match dist[to.0] {
                        BFResult::Some(dt) => {
                            if dv + fp(e) < dt {
                                BFResult::NegInf
                            }
                            else {
                                BFResult::Some(dt)
                            }
                        }
                        BFResult::None => {
                            BFResult::Some(dv + fp(e))
                        }
                        BFResult::NegInf => {
                            BFResult::NegInf
                        }
                    }
                }
            }
            else if let BFResult::NegInf = dist[v] {
                for ei in g.delta(&Vite(v)) {
                    let to = to(Vite(v), g.edge(ei));
                    dist[to.0] = BFResult::NegInf;
                }
            }
        }
    }
    dist
}
