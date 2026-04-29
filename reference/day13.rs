pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

pub struct ChunkedIterator<I: Iterator> {
    inner: I,
    chunk_size: usize,
}

impl<I: Iterator> ChunkedIterator<I> {
    pub fn new(iter: I, chunk_size: usize) -> Self {
        ChunkedIterator {
            inner: iter,
            chunk_size,
        }
    }
}

impl<I: Iterator> Iterator for ChunkedIterator<I> {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = Vec::with_capacity(self.chunk_size);
        for _ in 0..self.chunk_size {
            match self.inner.next() {
                Some(item) => chunk.push(item),
                None => break,
            }
        }
        if chunk.is_empty() {
            None
        } else {
            Some(chunk)
        }
    }
}
