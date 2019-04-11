use second::*;

pub struct UndirectedGraph<V: Vertex, E: Edge> {
    n: usize,
    m: usize,
    g: Vec<Vec<Eite>>,
    es: Vec<E>,
    vs: Vec<V>
}

impl<'a,V: Vertex, E: Edge> Graph<'a,V,E> for UndirectedGraph<V,E> {
    type EsIter = std::slice::Iter<'a,Eite>;
    fn add_edge(&mut self, e: E) {
        let ei = Eite(self.m);
        self.m += 1;
        self.g[e.from().0].push(ei);
        self.g[e.to().0].push(ei);
        self.es.push(e);
    }
    fn delta(&'a self, v: &Vite) -> Self::EsIter {
        self.g[v.0].iter()
    }
    fn edge(&self, e: &Eite) -> &E {
        &self.es[e.0]
    }
    fn vertex(&self, v: &Vite) -> &V {
       &self.vs[v.0]
    }
    fn v_size(&self) -> usize {
        self.n
    }
    fn e_size(&self) -> usize {
        self.m
    }
}

impl<'a,V: Vertex, E: Edge> Undirected<'a,V,E> for UndirectedGraph<V,E> {  }

impl<V: Vertex, E: Edge> UndirectedGraph<V,E> {
    pub fn new(n: usize) -> Self {
        let mut g = UndirectedGraph {
            n: n,
            m: 0,
            g: vec![Vec::<Eite>::new(); n],
            es: Vec::<E>::new(),
            vs: Vec::<V>::new()
        };
        for i in 0..n {
            g.vs.push(V::new(i));
        }
        g
    }
}

