use second::*;

pub struct BipartiteUndirectedGraph<V: Vertex, E: Edge> {
    le: usize,
    ri: usize,
    m: usize,
    g: Vec<Vec<Eite>>,
    es: Vec<E>,
    vs: Vec<V>,
    ls: Vec<Vite>,
    rs: Vec<Vite>
}

impl<'a, V: Vertex, E: Edge> Graph<'a,V,E> for BipartiteUndirectedGraph<V,E> {
    type EsIter = std::slice::Iter<'a,Eite>;
    fn add_edge(&mut self, e: E) { 
        assert!(e.from().0 < self.le);
        assert!(self.le <= e.to().0 && e.to().0 < self.le + self.ri);
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
        self.le + self.ri
    }
    fn e_size(&self) -> usize {
        self.m
    }
}

impl<'a,V: Vertex, E: Edge> Undirected<'a,V,E> for BipartiteUndirectedGraph<V,E> {  }
impl<'a,V: Vertex, E: Edge> Bipartite<'a,V,E> for BipartiteUndirectedGraph<V,E> { 
    fn left_size(&self) -> usize { self.le }
    fn right_size(&self) -> usize { self.ri }
    fn left_vs(&self) -> std::slice::Iter<Vite> { self.ls.iter() }
    fn right_vs(&self) -> std::slice::Iter<Vite> { self.rs.iter() }
}

impl<V: Vertex, E: Edge> BipartiteUndirectedGraph<V,E> {
     pub fn new(le: usize, ri: usize) -> BipartiteUndirectedGraph<V,E> { 
         BipartiteUndirectedGraph { 
             le: le,
             ri: ri,
             m: 0,
             g: vec![Vec::<Eite>::new(); le + ri],
             es: Vec::<E>::new(),
             vs: Vec::<V>::new(),
             ls: (0..le).map(|i| Vite(i)).collect(),
             rs: (le..le+ri).map(|i| Vite(i)).collect()
         }
     }
}
