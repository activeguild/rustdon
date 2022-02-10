struct FibIterator {
    a: usize,
    b: usize,
}

impl FibIterator {
    fn new() -> Self {
        Self { a: 1, b: 1 }
    }
}

impl Clone for FibIterator {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
        }
    }
}

impl Iterator for FibIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.a;
        self.a = self.b;
        self.b += tmp;
        return Some(self.a);
    }
}

fn main() {
    let fib_iter = FibIterator::new();
    let fib_iter2 = fib_iter.clone();
    for (i, n) in fib_iter.enumerate() {
        if i >= 10 {
            break;
        }
        print!("{},", n);
    }
    for (i, n) in fib_iter2.enumerate() {
        if i >= 10 {
            break;
        }
        print!("{},", n);
    }
}
