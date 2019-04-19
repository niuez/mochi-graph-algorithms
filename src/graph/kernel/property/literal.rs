pub trait Zero {
    fn zero() -> Self;
}

impl Zero for usize { fn zero() -> Self { 0 } }
impl Zero for u64 { fn zero() -> Self { 0 } }
impl Zero for u32 { fn zero() -> Self { 0 } }
impl Zero for u16 { fn zero() -> Self { 0 } }
impl Zero for u8 { fn zero() -> Self { 0 } }
impl Zero for isize { fn zero() -> Self { 0 } }
impl Zero for i64 { fn zero() -> Self { 0 } }
impl Zero for i32 { fn zero() -> Self { 0 } }
impl Zero for i16 { fn zero() -> Self { 0 } }
impl Zero for i8 { fn zero() -> Self { 0 } }

pub trait IsNN {}

impl IsNN for usize {}
impl IsNN for u64 {}
impl IsNN for u32 {}
impl IsNN for u16 {}
impl IsNN for u8 {}

pub trait IsNum: ToNNeg + ToArb { }
impl<N: ToNNeg + ToArb> IsNum for N { }

pub trait ToNNeg {
    type Output: Zero + IsNum + IsNN + std::ops::Add<Output=Self::Output> + std::ops::Sub<Output=Self::Output> + std::cmp::Ord + Copy;
    fn to_nneg(&self) -> Self::Output;
}

impl ToNNeg for usize {
    type Output = usize;
    fn to_nneg(&self) -> Self::Output { self.clone() }
}

impl ToNNeg for u64 {
    type Output = u64;
    fn to_nneg(&self) -> Self::Output { self.clone() }
}

impl ToNNeg for u32 {
    type Output = u32;
    fn to_nneg(&self) -> Self::Output { self.clone() }
}

impl ToNNeg for u16 {
    type Output = u16;
    fn to_nneg(&self) -> Self::Output { self.clone() }
}

impl ToNNeg for u8 {
    type Output = u8;
    fn to_nneg(&self) -> Self::Output { self.clone() }
}

impl ToNNeg for isize {
    type Output = usize;
    fn to_nneg(&self) -> Self::Output {
        match self.clone() {
            num if num >= 0 => num as Self::Output,
            _ => unreachable!()
        }
    }
}

impl ToNNeg for i64 {
    type Output = u64;
    fn to_nneg(&self) -> Self::Output {
        match self.clone() {
            num if num >= 0 => num as Self::Output,
            _ => unreachable!()
        }
    }
}

impl ToNNeg for i32 {
    type Output = u32;
    fn to_nneg(&self) -> Self::Output {
        match self.clone() {
            num if num >= 0 => num as Self::Output,
            _ => unreachable!()
        }
    }
}
impl ToNNeg for i16 {
    type Output = u16;
    fn to_nneg(&self) -> Self::Output {
        match self.clone() {
            num if num >= 0 => num as Self::Output,
            _ => unreachable!()
        }
    }
}

impl ToNNeg for i8 {
    type Output = u8;
    fn to_nneg(&self) -> Self::Output {
        match self.clone() {
            num if num >= 0 => num as Self::Output,
            _ => unreachable!()
        }
    }
}

pub trait ToArb {
    type Output: Zero + IsNum + std::ops::Add<Output=Self::Output> + std::ops::Sub<Output=Self::Output> + std::cmp::Ord + Copy;
    fn to_arb(&self) -> Self::Output;
}

impl ToArb for isize {
    type Output = isize;
    fn to_arb(&self) -> Self::Output { self.clone() }
}

impl ToArb for i64 {
    type Output = i64;
    fn to_arb(&self) -> Self::Output { self.clone() }
}

impl ToArb for i32 {
    type Output = i32;
    fn to_arb(&self) -> Self::Output { self.clone() }
}

impl ToArb for i16 {
    type Output = i16;
    fn to_arb(&self) -> Self::Output { self.clone() }
}

impl ToArb for i8 {
    type Output = i8;
    fn to_arb(&self) -> Self::Output { self.clone() }
}

impl ToArb for usize {
    type Output = isize;
    fn to_arb(&self) -> Self::Output {
        self.clone() as isize
    }
}

impl ToArb for u64 {
    type Output = i64;
    fn to_arb(&self) -> Self::Output {
        self.clone() as i64
    }
}

impl ToArb for u32 {
    type Output = i32;
    fn to_arb(&self) -> Self::Output {
        self.clone() as i32
    }
}

impl ToArb for u16 {
    type Output = i16;
    fn to_arb(&self) -> Self::Output {
        self.clone() as i16
    }
}

impl ToArb for u8 {
    type Output = i8;
    fn to_arb(&self) -> Self::Output {
        self.clone() as i8
    }
}

pub trait Integer: Sized + std::ops::Shl<usize, Output=Self> + std::ops::Shr<usize, Output=Self> {}

impl Integer for usize {}
impl Integer for u64 {}
impl Integer for u32 {}
impl Integer for u16 {}
impl Integer for u8 {}
impl Integer for isize {}
impl Integer for i64 {}
impl Integer for i32 {}
impl Integer for i16 {}
impl Integer for i8 {}
