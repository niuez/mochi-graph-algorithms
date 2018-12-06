use graph::*;
use graph::property::*;
use graph::undirected_graph::*;

use std::collections::VecDeque;

#[derive(Clone,Copy,PartialEq,Eq)]
enum GLabel {
    Vertex(Vertex),
    Edge(Edge),
    NonOuter(i32),
}

fn eval_first(label: &Vec<GLabel>, first: &mut Vec<Vertex>, x: Vertex) -> Vertex {
    if let GLabel::NonOuter(_) = label[first[x.0].0] { first[x.0].clone() }
    else {
        let v = first[x.0];
        first[x.0] = eval_first(label, first, v);
        first[x.0].clone()
    }
}

fn rematch(mate: &mut Vec<Vertex>, label: &Vec<GLabel>, v: Vertex, w: Vertex) {
    let t = mate[v.0].clone();
    mate[v.0] = w;
    if mate[t.0] != v {  }
    else if let GLabel::Vertex(s) = label[v.0] {
        mate[t.0] = s;
        rematch(mate,label,s,t);
    }
    else if let GLabel::Edge(e) = label[v.0] {
        rematch(mate,label,e.from,e.to);
        rematch(mate,label,e.to,e.from);
    }
}

fn assign_label(mate: & Vec<Vertex>, label: &mut Vec<GLabel>, first: &mut Vec<Vertex>, x: Vertex, y: Vertex, e: Edge, que: &mut VecDeque<Vertex>) {
    let mut r = eval_first(label,first,x);
    let mut s = eval_first(label,first,y);
    let num = e.index as i32;
    label[r.0] = GLabel::NonOuter(num);
    label[s.0] = GLabel::NonOuter(num);
    if r != s {
        let join;
        loop { 
            if s != Vertex(0) { std::mem::swap(&mut r, &mut s); }
            if let GLabel::Vertex(temp) = label[mate[r.0].0] {
                r = eval_first(label,first,temp);
                if GLabel::NonOuter(num) == label[r.0] {
                    join = r;
                    break;
                }
                label[r.0] = GLabel::NonOuter(num);
            }
        }
        let mut v = first[x.0];
        while v != join {
            que.push_back(v);
            label[v.0] = GLabel::Edge(e);
            first[v.0] = join;
            if let GLabel::Vertex(lm) = label[mate[v.0].0] {
                v = first[lm.0];
            }
        }
        let mut v = first[y.0];
        while v != join {
            que.push_back(v);
            label[v.0] = GLabel::Edge(e);
            first[v.0] = join;
            if let GLabel::Vertex(lm) = label[mate[v.0].0] {
                v = first[lm.0];
            }
        }
    }
}

fn augment_check<'a,VP: Property,EP: Property,G: Undirected<'a,VP,EP>>(g: &'a G, mate: &mut Vec<Vertex>, label: &mut Vec<GLabel>, first: &mut Vec<Vertex>,u : Vertex) -> bool {

    let n = g.vertices_cnt();
    if mate[u.0] != Vertex(0) {
        return false;
    }
    first[u.0] = Vertex(0);
    label[u.0] = GLabel::Vertex(Vertex(0));

    let mut que = VecDeque::<Vertex>::new();

    que.push_back(u);

    loop {
        if let Some(x) = que.pop_front() {
            for e in g.delta(&x) {
                let y = e.to;
                if mate[y.0] == Vertex(0) && y != u {
                    mate[y.0] = x;
                    rematch(mate,label, x, y);
                    for j in 0..n {
                        label[j] = GLabel::NonOuter(-1);
                    }
                    return true;
                }
                else if let GLabel::Vertex(_) = label[y.0] {
                    assign_label(mate,label,first, x, y, e.clone(),&mut que);
                }
                else if let GLabel::Edge(_) = label[y.0] {
                    assign_label(mate,label,first, x, y, e.clone(),&mut que);
                }
                else if let GLabel::NonOuter(_) = label[mate[y.0].0] {
                    label[mate[y.0].0] = GLabel::Vertex(x);
                    first[mate[y.0].0] = y;
                    que.push_back(mate[y.0]);
                }
            }
        }
        else { break; }
    }

    false
}


pub fn gabow_e_algorithm<'a,VP,EP,G>(ug : &'a G) -> Vec<(Vertex,Vertex)> where VP: Property , EP: Property, G: Undirected<'a,VP,EP> + StaticGraph<'a,VP,EP>{
    let mut ans = Vec::<(Vertex,Vertex)>::new();
    let n = ug.vertices_cnt();

    let mut g = UndirectedGraph::<usize,usize>::new(n+1,0);

    for v in 0..n {
        for e in ug.delta(&Vertex(v)) {
            g.add_edge(&Vertex(v + 1) , &Vertex(e.to.0 + 1) , 0);
        }
    }

    let mut mate = vec![Vertex(0);n + 1];
    let mut label = vec![GLabel::NonOuter(-1); n + 1];
    let mut first = vec![Vertex(0);n + 1];


    for i in 1..=n {
        augment_check(&g,&mut mate, &mut label, &mut first,Vertex(i));
    }

    for i in 1..=n {
        if i < mate[i].0 {
            ans.push((Vertex(i - 1),Vertex(mate[i].0 - 1)));
        }
    }

    ans
}
