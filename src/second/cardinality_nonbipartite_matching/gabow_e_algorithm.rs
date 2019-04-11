use second::*;
use second::undirected_graph::*;

use std::collections::VecDeque;

#[derive(Clone,Copy,PartialEq,Eq)]
enum GLabel {
    Vertex(Vite),
    Edge((Vite,Vite)),
    NonOuter(i32),
}

fn eval_first(label: &Vec<GLabel>, first: &mut Vec<Vite>, x: Vite) -> Vite {
    if let GLabel::NonOuter(_) = label[first[x.0].0] { first[x.0].clone() }
    else {
        let v = first[x.0];
        first[x.0] = eval_first(label, first, v);
        first[x.0].clone()
    }
}

fn rematch(mate: &mut Vec<Vite>, label: &Vec<GLabel>, v: Vite, w: Vite) {
    let t = mate[v.0].clone();
    mate[v.0] = w;
    if mate[t.0] != v {  }
    else if let GLabel::Vertex(s) = label[v.0] {
        mate[t.0] = s;
        rematch(mate,label,s,t);
    }
    else if let GLabel::Edge(e) = label[v.0] {
        rematch(mate,label,e.0,e.1);
        rematch(mate,label,e.1,e.0);
    }
}

fn assign_label(mate: & Vec<Vite>, label: &mut Vec<GLabel>, first: &mut Vec<Vite>, x: Vite, y: Vite, num: i32, que: &mut VecDeque<Vite>) {
    let mut r = eval_first(label,first,x);
    let mut s = eval_first(label,first,y);
    label[r.0] = GLabel::NonOuter(num);
    label[s.0] = GLabel::NonOuter(num);
    if r != s {
        let join;
        loop { 
            if s != Vite(0) { std::mem::swap(&mut r, &mut s); }
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
            label[v.0] = GLabel::Edge((x,y));
            first[v.0] = join;
            if let GLabel::Vertex(lm) = label[mate[v.0].0] {
                v = first[lm.0];
            }
        }
        let mut v = first[y.0];
        while v != join {
            que.push_back(v);
            label[v.0] = GLabel::Edge((x,y));
            first[v.0] = join;
            if let GLabel::Vertex(lm) = label[mate[v.0].0] {
                v = first[lm.0];
            }
        }
    }
}

fn augment_check<'a,V,E,G>(g: &'a G, mate: &mut Vec<Vite>, label: &mut Vec<GLabel>, first: &mut Vec<Vite>,u : Vite) -> bool 
    where V: Vertex, E: Edge, G: Undirected<'a,V,E> {

    let n = g.v_size();
    if mate[u.0] != Vite(0) {
        return false;
    }
    first[u.0] = Vite(0);
    label[u.0] = GLabel::Vertex(Vite(0));

    let mut que = VecDeque::<Vite>::new();

    que.push_back(u);

    while let Some(x) = que.pop_front() {
        for e in g.delta(&x) {
            let y = to(x,g.edge(e));
            if mate[y.0] == Vite(0) && y != u {
                mate[y.0] = x;
                rematch(mate,label, x, y);
                for j in 0..n {
                    label[j] = GLabel::NonOuter(-1);
                }
                return true;
            }
            else if let GLabel::Vertex(_) = label[y.0] {
                assign_label(mate,label,first, x, y, e.0 as i32,&mut que);
            }
            else if let GLabel::Edge(_) = label[y.0] {
                assign_label(mate,label,first, x, y, e.0 as i32,&mut que);
            }
            else if let GLabel::NonOuter(_) = label[mate[y.0].0] {
                label[mate[y.0].0] = GLabel::Vertex(x);
                first[mate[y.0].0] = y;
                que.push_back(mate[y.0]);
            }
        }
    }

    false
}


pub fn gabow_e_algorithm_cnbm<'a,V,E,G>(ug : &'a G) -> Vec<(Vite,Vite)> 
where V: Vertex , E: Edge, G: Undirected<'a,V,E> {
    let mut ans = Vec::<(Vite,Vite)>::new();
    let n = ug.v_size();

    let mut g = UndirectedGraph::<usize,(usize,usize)>::new(n+1);

    for v in 0..n {
        for e in ug.delta(&Vite(v)) {
            g.add_edge((v + 1, ug.edge(e).to().0 + 1));
        }
    }

    let mut mate = vec![Vite(0);n + 1];
    let mut label = vec![GLabel::NonOuter(-1); n + 1];
    let mut first = vec![Vite(0);n + 1];


    for i in 1..n+1 {
        augment_check(&g,&mut mate, &mut label, &mut first,Vite(i));
    }

    for i in 1..n+1 {
        if i < mate[i].0 {
            ans.push((Vite(i - 1),Vite(mate[i].0 - 1)));
        }
    }

    ans
}
