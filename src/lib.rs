pub mod graph;


#[cfg(test)]
mod tests {
    use graph::*;
    use graph::dynamic_directed_graph::*;

    #[test]
    fn test2() {
        println!("test2 start");
        let mut g = DynamicDirectedGraph::<usize,usize>::new();
        g.add_vertex(&Vertex(1), 0);
        g.add_vertex(&Vertex(2), 0);
        g.add_vertex(&Vertex(3), 0);
        g.add_vertex(&Vertex(4), 0);

        g.add_edge(&Vertex(1),&Vertex(2),0);
        g.add_edge(&Vertex(2),&Vertex(3),0);
        g.add_edge(&Vertex(3),&Vertex(4),0);
        g.add_edge(&Vertex(4),&Vertex(1),0);
        for v in 1..=4 {
            println!("from {}" , v);
            for e in g.delta(&Vertex(v)) {
                println!("{}" , e.to.0);
            }
        }
        println!("remove (2,3)");
        g.erase_edge(&Edge{from: Vertex(2) , to: Vertex(3) , index: 1});
        for v in 1..=4 {
            println!("from {}" , v);
            for e in g.delta(&Vertex(v)) {
                println!("{}" , e.to.0);
            }
        }
        g.add_edge(&Vertex(2),&Vertex(3),0);

        println!("remove vertex 2");

        g.erase_vertex(&Vertex(2));
        
        for v in 1..=4 {
            if v == 2 { continue; }
            println!("from {}" , v);
            for e in g.delta(&Vertex(v)) {
                println!("{}" , e.to.0);
            }
        }

        println!("add vertex 2");
        g.add_vertex(&Vertex(2),0);
        for v in 1..=4 {
            println!("from {}" , v);
            for e in g.delta(&Vertex(v)) {
                println!("{}" , e.to.0);
            }
        }
    }
    use graph::dynamic_undirected_graph::*;

    #[test]
    fn test3() {
        println!("test3 start");
        let mut g = DynamicUndirectedGraph::<usize,usize>::new();
        g.add_vertex(&Vertex(1), 0);
        g.add_vertex(&Vertex(2), 0);
        g.add_vertex(&Vertex(3), 0);
        g.add_vertex(&Vertex(4), 0);

        g.add_edge(&Vertex(1),&Vertex(2),0);
        g.add_edge(&Vertex(2),&Vertex(3),0);
        g.add_edge(&Vertex(3),&Vertex(4),0);
        g.add_edge(&Vertex(4),&Vertex(1),0);
        for v in 1..=4 {
            println!("from {}" , v);
            for e in g.delta(&Vertex(v)) {
                println!("{}" , e.to.0);
            }
        }
        println!("remove (2,3)");
        g.erase_edge(&Edge{from: Vertex(2) , to: Vertex(3) , index: 1});
        for v in 1..=4 {
            println!("from {}" , v);
            for e in g.delta(&Vertex(v)) {
                println!("{}" , e.to.0);
            }
        }
        g.add_edge(&Vertex(2),&Vertex(3),0);

        println!("remove vertex 2");

        g.erase_vertex(&Vertex(2));
        
        for v in 1..=4 {
            if v == 2 { continue; }
            println!("from {}" , v);
            for e in g.delta(&Vertex(v)) {
                println!("{}" , e.to.0);
            }
        }

        println!("add vertex 2");
        g.add_vertex(&Vertex(2),0);
        for v in 1..=4 {
            println!("from {}" , v);
            for e in g.delta(&Vertex(v)) {
                println!("{}" , e.to.0);
            }
        }
    }
}
