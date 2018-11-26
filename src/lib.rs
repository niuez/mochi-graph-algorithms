pub mod graph;


#[cfg(test)]
mod tests {
    use graph::Vertex;
    use graph::Graph;
    use graph::UndirectedGraph;

    #[test]
    fn undirected_graph_test() {
        let mut g = UndirectedGraph::<i32 , i32>::new(5 , 0);
        g.add_edge(&Vertex(0) , &Vertex(1) , 5);
        for e in g.delta(&Vertex(0)) {
            assert_eq!(e.index, 0);
            assert_eq!(e.to, Vertex(1));
            assert_eq!(*g.eprop(e) , 5);
            for e2 in g.delta(&e.to) {
                assert_eq!(e2.index, 0);
                assert_eq!(e2.to, Vertex(0));
                assert_eq!(*g.eprop(e2) , 5);
            }
        }
    }

    use graph::shortest_path::dijkstra::*;
    use graph::DirectedGraph;

    #[test]
    fn dijkstra_test() {
        let mut g = DirectedGraph::<usize, usize>::new(4,0);
        g.add_edge(&Vertex(0),&Vertex(1),1);
        g.add_edge(&Vertex(0),&Vertex(2),4);
        g.add_edge(&Vertex(1),&Vertex(2),2);
        g.add_edge(&Vertex(2),&Vertex(3),1);
        g.add_edge(&Vertex(1),&Vertex(3),5);
        let res = dijkstra(&g, Vertex(0), 0, |ep| ep);
        for d in res {
            let r = match d {
                Some(i) => i,
                None => 1333
            };
            println!("{}", r);
        }
    }
}
