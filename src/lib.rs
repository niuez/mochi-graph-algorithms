pub mod graph;


#[cfg(test)]
mod tests {
    use graph::*;
    use graph::undirected_graph::*;
    #[test]
    fn undirected_new() {
        let mut g = UndirectedGraph::<usize,usize>::new(5,0);
        g.add_edge(&Vertex(0),&Vertex(1),133);
    }
}
