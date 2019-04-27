pub struct UnionFindTree {
    par: Vec<Option<usize>>,
    rank: Vec<usize>,
}

impl UnionFindTree {
    pub fn new(n: usize) -> Self {
        UnionFindTree {
            par: vec![None; n],
            rank: vec![0usize; n],
        }
    }

    pub fn root(&mut self, i: usize) -> usize {
        match self.par[i] {
            Some(p) => {
                let r = self.root(p);
                self.par[i] = Some(r);
                r
            }
            None => i
        }
    }

    pub fn unite(&mut self, i: usize, j: usize) -> Option<(usize, usize)> {
        let x = self.root(i);
        let y = self.root(j);
        if x == y { None }
        else if self.rank[x] < self.rank[y] {
            self.par[x] = Some(y);
            Some((y, x))
        }
        else {
            self.par[y] = Some(x);
            if self.rank[x] == self.rank[y] { self.rank[x] += 1 }
            Some((x, y))
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}
