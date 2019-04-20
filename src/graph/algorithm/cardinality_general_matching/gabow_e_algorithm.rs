use graph::kernel::graph::*;
use graph::graph::UndirectedGraph;

use std::collections::VecDeque;

#[derive(Clone,Copy,PartialEq,Eq)]
enum GLabel {
    Vertex(usize),
    Edge((usize, usize)),
    NonOuter(i32),
}

fn eval_first(label: &Vec<GLabel>, first: &mut Vec<usize>, x: usize) -> usize {
    if let GLabel::NonOuter(_) = label[first[x]] { first[x].clone() }
    else {
        let v = first[x];
        first[x] = eval_first(label, first, v);
        first[x].clone()
    }
}

fn rematch(mate: &mut Vec<usize>, label: &Vec<GLabel>, v: usize, w: usize) {
    let t = mate[v].clone();
    mate[v] = w;
    if mate[t] != v {  }
    else if let GLabel::Vertex(s) = label[v] {
        mate[t] = s;
        rematch(mate,label,s,t);
    }
    else if let GLabel::Edge(e) = label[v] {
        rematch(mate,label,e.0,e.1);
        rematch(mate,label,e.1,e.0);
    }
}

fn assign_label(mate: & Vec<usize>, label: &mut Vec<GLabel>, first: &mut Vec<usize>, x: usize, y: usize, num: i32, que: &mut VecDeque<usize>) {
    let mut r = eval_first(label,first,x);
    let mut s = eval_first(label,first,y);
    label[r] = GLabel::NonOuter(num);
    label[s] = GLabel::NonOuter(num);
    if r != s {
        let join;
        loop { 
            if s != 0 { std::mem::swap(&mut r, &mut s); }
            if let GLabel::Vertex(temp) = label[mate[r]] {
                r = eval_first(label,first,temp);
                if GLabel::NonOuter(num) == label[r] {
                    join = r;
                    break;
                }
                label[r] = GLabel::NonOuter(num);
            }
        }
        let mut v = first[x];
        while v != join {
            que.push_back(v);
            label[v] = GLabel::Edge((x,y));
            first[v] = join;
            if let GLabel::Vertex(lm) = label[mate[v]] {
                v = first[lm];
            }
        }
        let mut v = first[y];
        while v != join {
            que.push_back(v);
            label[v] = GLabel::Edge((x,y));
            first[v] = join;
            if let GLabel::Vertex(lm) = label[mate[v]] {
                v = first[lm];
            }
        }
    }
}

fn augment_check(g: &UndirectedGraph<usize,(usize,usize)>, mate: &mut Vec<usize>, label: &mut Vec<GLabel>, first: &mut Vec<usize>,u : usize) -> bool {
    let n = g.v_size();
    if mate[u] != 0 {
        return false;
    }
    first[u] = 0;
    label[u] = GLabel::Vertex(0);

    let mut que = VecDeque::new();

    que.push_back(u);

    while let Some(x) = que.pop_front() {
        for e in g.delta(&x) {
            let y = *e.to();
            if mate[y] == 0 && y != u {
                mate[y] = x;
                rematch(mate,label, x, y);
                for j in 0..n {
                    label[j] = GLabel::NonOuter(-1);
                }
                return true;
            }
            else if let GLabel::Vertex(_) = label[y] {
                assign_label(mate,label,first, x, y, e.id() as i32,&mut que);
            }
            else if let GLabel::Edge(_) = label[y] {
                assign_label(mate,label,first, x, y, e.id() as i32,&mut que);
            }
            else if let GLabel::NonOuter(_) = label[mate[y]] {
                label[mate[y]] = GLabel::Vertex(x);
                first[mate[y]] = y;
                que.push_back(mate[y]);
            }
        }
    }

    false
}


pub fn gabow_e_algorithm<'a,G>(ug : &'a G) -> Vec<(G::VType, G::VType)> 
where G: Undirected<'a> {
    let mut ans = Vec::new();
    let n = ug.v_size();

    let mut g = UndirectedGraph::<usize,(usize,usize)>::new(n+1);

    for ref e in ug.edges() {
        g.add_edge((e.from().id() + 1, e.to().id() + 1));
    }

    let mut mate = vec![0;n + 1];
    let mut label = vec![GLabel::NonOuter(-1); n + 1];
    let mut first = vec![0;n + 1];


    for i in 1..n+1 {
        augment_check(&g,&mut mate, &mut label, &mut first,i);
    }

    for e in ug.edges() {
        if mate[e.from().id() + 1] == e.to().id() + 1 {
            ans.push((e.from().clone(), e.to().clone()));
            mate[e.from().id() + 1] = 0;
            mate[e.to().id() + 1] = 0;
        }
    }

    ans
}


#[test]
fn hopcroft_karp_test() {
    use graph::graph::BipartiteGraph;

    {
        let mut g = BipartiteGraph::new(7);
        g.add_edge((0, 0 + 3));
        g.add_edge((0, 2 + 3));
        g.add_edge((0, 3 + 3));
        g.add_edge((1, 1 + 3));
        g.add_edge((2, 1 + 3));
        g.add_edge((2, 3 + 3));
        assert!(gabow_e_algorithm(&g).len() == 3);
    }

    {
        let mut g = UndirectedGraph::new(10);
        g.add_edge((0, 9));
        g.add_edge((0, 1));
        g.add_edge((0, 2));
        g.add_edge((1, 2));
        g.add_edge((2, 8));
        g.add_edge((2, 3));
        g.add_edge((3, 6));
        g.add_edge((3, 7));
        g.add_edge((4, 5));
        g.add_edge((4, 8));
        g.add_edge((5, 6));
        g.add_edge((6, 7));
        assert!(gabow_e_algorithm(&g).len() == 5);
    }

}
