use graph::*;
use graph::property::*;

use std::cmp::Ordering;
use std::cmp::min;

struct DAryHeap<T> where T: Ord + Clone {
    heap: Vec<usize>,
    idx: Vec<usize>,
    val: Vec<Option<T>>,
    d: usize,
}

impl<T: Ord + Clone> DAryHeap<T> {
    fn new(d: usize, k: usize) -> DAryHeap<T> {
        DAryHeap { 
            heap: Vec::<usize>::new(),
            idx: vec![k+1;k],
            val: vec![None;k],
            d: d,
        }
    }
    fn up(&mut self, k: usize) {
        let mut i = self.idx[k];
        while i > 0 {
            if self.val[self.heap[i]].cmp(&self.val[self.heap[(i - 1) / self.d]]) == Ordering::Greater {
                self.idx.swap(self.heap[i], self.heap[(i - 1) / self.d]);
                self.heap.swap(i, (i - 1) / self.d);
            }
            i = (i - 1) / self.d;
        }
    }
    fn down(&mut self, k: usize) {
        let i = self.idx[k];
        let mut smi = i;
        for j in (i * self.d + 1)..min(i * self.d + self.d + 1, self.heap.len()) {
            if self.val[self.heap[j]].cmp(&self.val[self.heap[smi]]) == Ordering::Greater {
                smi = j;
            }
        }
        if smi != i {
            self.idx.swap(self.heap[i], self.heap[smi]);
            self.heap.swap(i, smi);
            let ii = self.heap[smi];
            self.down(ii);
        }
    }
    fn push(&mut self,k: usize, v: T) {
        if let Some(v2) = self.val[k].clone() {
            if v.cmp(&v2) == Ordering::Greater {
                self.val[k] = Some(v);
                self.up(k);
            }
        }
        else {
            self.val[k] = Some(v);
            self.idx[k] = self.heap.len();
            self.heap.push(k);
            self.up(k);
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.heap.len() == 0 { None }
        else {
            let l = self.heap.len();
            self.idx.swap(self.heap[0], self.heap[l - 1]);
            self.heap.swap(0, l - 1);
            let i = self.heap[0];
            let j = self.heap.pop().unwrap();
            let v = self.val[j].clone();
            self.val[j] = None;
            if self.heap.len() > 0 { self.down(i); }
            v
        }
    }
}

#[derive(Eq, Clone)]
struct DijkstraNode<W: NonNegativeWeighted> {
    dist: Option<W>,
    ver : Vite,
}

impl<W: NonNegativeWeighted> Ord for DijkstraNode<W> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}
impl<W: NonNegativeWeighted> PartialOrd for DijkstraNode<W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.dist.cmp(&self.dist))
    }
}
impl<W: NonNegativeWeighted> PartialEq for DijkstraNode<W> {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

pub fn d_ary_heap_dijkstra_s3p<'a,V,E,G,F>(g: &'a G, s: Vite,fp: F) -> Vec<Option<usize>>
where V: Vertex , E: Edge,G: Graph<'a,V,E>, F: Fn(&E) -> usize {
    let n = g.v_size();
    let m = g.e_size();
    let mut dist = vec![None ; n];
    dist[s.0] = Some(usize::zero());
    
    let mut heap = DAryHeap::new(2 + (m + n - 1) / n, n);

    heap.push(s.0, DijkstraNode::<usize>{ dist : dist[s.0] , ver : s});

    loop{
        if let Some(DijkstraNode::<usize>{dist : Some(d) , ver : v}) = heap.pop() {
            if let Some(now) = dist[v.0] {
                if now < d { continue }
            }
            for ei in g.delta(&v) {
                let e = g.edge(ei);
                let to = to(v,e);
                if match dist[to.0] {
                    Some(d2) => {
                        if fp(e) + d < d2 { true }
                        else { false }
                    }
                    None => true
                } {
                    dist[to.0] = Some(fp(e) + d);
                    heap.push(to.0, DijkstraNode::<usize>{ dist: dist[to.0] , ver : to.clone() });
                }
            }
        }
        else { break }
    }

    dist
}
