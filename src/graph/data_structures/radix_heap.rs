pub fn bsr32(x: u32) -> usize {
    if x == 0 { 0 }
    else {
        let mut t = 16usize;
        for i in (0..=3).rev() {
            if x & !((1u32 << t) - 1) != 0 { t += 1usize << i; }
            else { t -= 1usize << i; }
        }
        if x & !((1u32 << t) - 1) != 0 { t += 1; }
        t
    }
}

pub fn bsr64(x: u64) -> usize {
    if x == 0 { 0 }
    else {
        let mut t = 16usize;
        for i in (0..=4).rev() {
            if x & !((1u64 << t) - 1) != 0 { t += 1usize << i; }
            else { t -= 1usize << i; }
        }
        if x & !((1u64 << t) - 1) != 0 { t += 1; }
        t
    }
}

pub struct RadixHeap32<T> {
    v: Vec<Vec<(u32, T)>>,
    last: u32,
    size: usize, 
}

impl<T> RadixHeap32<T> {
    pub fn new() -> Self {
        let mut temp = Vec::new();
        for _ in 0..33 { temp.push(Vec::new()); }
        RadixHeap32 { v: temp, last: 0, size: 0 }
    }

    pub fn pop(&mut self) -> Option<(u32, T)> {
        if self.is_empty() {
            None
        }
        else {
            self.size -= 1;
            if self.v[0].is_empty() {
                let mut i = 1;
                while self.v[i].is_empty() { i += 1; }
                self.last = u32::max_value();
                for d in self.v[i].iter() { 
                    self.last = std::cmp::min(self.last, d.0.clone());
                }

                for d in std::mem::replace(&mut self.v[i], Vec::new()) {
                    self.v[bsr32(self.last ^ d.0)].push(d);
                }
            }
            self.v[0].pop()
        }
    }

    pub fn push(&mut self, x: (u32, T)) {
        assert!(self.last <= x.0);
        self.size += 1;
        self.v[bsr32(self.last ^ x.0)].push(x);
    }

    pub fn is_empty(&self) -> bool { self.size == 0 }

    pub fn size(&self) -> usize { self.size }
}

pub struct RadixHeap64<T> {
    v: Vec<Vec<(u64, T)>>,
    last: u64,
    size: usize, 
}

impl<T> RadixHeap64<T> {
    pub fn new() -> Self {
        let mut temp = Vec::new();
        for _ in 0..65 { temp.push(Vec::new()); }
        RadixHeap64 { v: temp, last: 0, size: 0 }
    }

    pub fn pop(&mut self) -> Option<(u64, T)> {
        if self.is_empty() {
            None
        }
        else {
            self.size -= 1;
            if self.v[0].is_empty() {
                let mut i = 1;
                while self.v[i].is_empty() { i += 1; }
                self.last = u64::max_value();
                for d in self.v[i].iter() { 
                    self.last = std::cmp::min(self.last, d.0.clone());
                }

                for d in std::mem::replace(&mut self.v[i], Vec::new()) {
                    self.v[bsr64(self.last ^ d.0)].push(d);
                }
            }
            self.v[0].pop()
        }
    }

    pub fn push(&mut self, x: (u64, T)) {
        assert!(self.last <= x.0);
        self.size += 1;
        self.v[bsr64(self.last ^ x.0)].push(x);
    }

    pub fn is_empty(&self) -> bool { self.size == 0 }

    pub fn size(&self) -> usize { self.size }
}

