// Day 11: Fibonacci Iterator and ChunkedIterator
//
// Implement custom iterators: an infinite Fibonacci sequence generator and a
// chunking adapter that groups elements from any iterator.
//
// Learning goals:
//   - Implementing the `Iterator` trait for custom types
//   - Working with `Item` associated type
//   - Generic iterator adapters
//   - Combining multiple iterators

/// An iterator that yields Fibonacci numbers starting from 0, 1.
/// Sequence: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        todo!("Implement new")
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Implement Iterator::next for Fibonacci")
    }
}

/// An iterator adapter that groups the output of another iterator into chunks
/// of the given size. If the inner iterator has fewer elements left than the
/// chunk size, the final chunk contains the remaining elements.
///
/// An empty inner iterator yields no chunks (not even an empty one).
pub struct ChunkedIterator<I: Iterator> {
    // Define your fields here
    inner: I,
    chunk_size: usize,
}

impl<I: Iterator> ChunkedIterator<I> {
    pub fn new(iter: I, chunk_size: usize) -> Self {
        todo!("Implement new")
    }
}

impl<I: Iterator> Iterator for ChunkedIterator<I> {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Implement Iterator::next for ChunkedIterator")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_first_ten() {
        let fib: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn fibonacci_first_two() {
        let fib: Vec<u64> = Fibonacci::new().take(2).collect();
        assert_eq!(fib, vec![0, 1]);
    }

    #[test]
    fn fibonacci_is_infinite() {
        // Should be able to take many elements without panicking
        // (Fibonacci with u64 overflows around element 94, so we use 90)
        let count = Fibonacci::new().take(90).count();
        assert_eq!(count, 90);
    }

    #[test]
    fn chunked_iterator_basic() {
        let data = vec![1, 2, 3, 4, 5];
        let chunks: Vec<Vec<i32>> = ChunkedIterator::new(data.into_iter(), 3).collect();
        assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5]]);
    }

    #[test]
    fn chunk_size_of_one_yields_single_element_vecs() {
        let data = vec!["a", "b", "c", "d"];
        let chunks: Vec<Vec<&str>> = ChunkedIterator::new(data.into_iter(), 1).collect();
        assert_eq!(chunks, vec![vec!["a"], vec!["b"], vec!["c"], vec!["d"]]);
    }

    #[test]
    fn chunk_size_larger_than_input_yields_one_chunk() {
        let data = vec![1, 2, 3];
        let chunks: Vec<Vec<i32>> = ChunkedIterator::new(data.into_iter(), 10).collect();
        assert_eq!(chunks, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn empty_iterator_yields_no_chunks() {
        let data: Vec<i32> = vec![];
        let chunks: Vec<Vec<i32>> = ChunkedIterator::new(data.into_iter(), 3).collect();
        assert_eq!(chunks, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn chunk_iterator_works_with_vec_iterator() {
        // Verify it works with std::slice::Iter
        let data = vec!['a', 'b', 'c', 'd', 'e', 'f'];
        let chunks: Vec<Vec<&char>> = ChunkedIterator::new(data.iter(), 2).collect();
        assert_eq!(chunks, vec![
            vec![&'a', &'b'],
            vec![&'c', &'d'],
            vec![&'e', &'f'],
        ]);
    }

    #[test]
    fn combined_fibonacci_chunked_by_three() {
        let fib_nums: Vec<u64> = Fibonacci::new().take(10).collect();
        let chunked = ChunkedIterator::new(fib_nums.into_iter(), 3);
        let chunks: Vec<Vec<u64>> = chunked.collect();
        assert_eq!(
            chunks,
            vec![
                vec![0, 1, 1],
                vec![2, 3, 5],
                vec![8, 13, 21],
                vec![34],
            ]
        );
    }

    #[test]
    fn chunked_iterator_exact_multiple() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let chunks: Vec<Vec<i32>> = ChunkedIterator::new(data.into_iter(), 2).collect();
        assert_eq!(chunks, vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
    }
}
