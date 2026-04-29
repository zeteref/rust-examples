// Day 7: Iterator Combinators
//
// Write functions that use Rust's iterator API (map, filter, fold, collect,
// sort_by, etc.) to process collections.
//
// Learning goals:
//   - Iterator adapters: `filter`, `map`, `collect`
//   - Working with `HashMap`
//   - Generic functions with trait bounds
//   - Functional programming patterns in Rust

/// Takes a slice of numbers, keeps only even numbers, squares each,
/// and returns them sorted in descending order.
pub fn process_numbers(numbers: &[i32]) -> Vec<i32> {
    todo!("Implement process_numbers using iterator combinators")
}

/// Returns the top `n` item references, sorted by the given key function
/// in descending order. If n exceeds the number of items, returns all items.
pub fn top_n_by_key<'a, T, K: Ord>(items: &'a [T], n: usize, key_fn: impl Fn(&T) -> K) -> Vec<&'a T> {
    todo!("Implement top_n_by_key")
}

/// Groups words by their first character (lowercased).
/// Returns a HashMap where each key is a char and the value is a sorted Vec
/// of words (as Strings) that start with that character.
pub fn group_by_first_char(words: &[&str]) -> std::collections::HashMap<char, Vec<String>> {
    todo!("Implement group_by_first_char")
}

/// Returns the frequency of each element in a sorted iterator.
/// The output is a Vec of (element, count) in ascending order.
pub fn running_totals<I: Iterator<Item = i32>>(iter: I) -> Vec<(i32, i32)> {
    todo!("Implement running_totals combining consecutive equal values")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn process_numbers_basic() {
        // evens: 2,4,6 -> squares: 4,16,36 -> descending: 36,16,4
        assert_eq!(process_numbers(&[1, 2, 3, 4, 5, 6]), vec![36, 16, 4]);
    }

    #[test]
    fn process_numbers_empty_input() {
        assert_eq!(process_numbers(&[]), vec![]);
    }

    #[test]
    fn process_numbers_all_odd_returns_empty() {
        assert_eq!(process_numbers(&[3, 1, 5, 7, 9]), vec![]);
    }

    #[test]
    fn process_numbers_with_negatives() {
        // evens: -4, -2, 0, 2 -> squares: 16, 4, 0, 4 -> descending: 16, 4, 4, 0
        assert_eq!(process_numbers(&[-4, -3, -2, -1, 0, 1, 2]), vec![16, 4, 4, 0]);
    }

    #[test]
    fn top_n_by_key_basic() {
        let data = vec![
            ("apple", 10),
            ("banana", 30),
            ("cherry", 20),
            ("date", 40),
        ];
        let result = top_n_by_key(&data, 2, |&(_, score)| score);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], &("date", 40));
        assert_eq!(result[1], &("banana", 30));
    }

    #[test]
    fn top_n_exceeds_length_returns_all() {
        let data = vec!["x", "y", "z"];
        let result = top_n_by_key(&data, 10, |s| s.len());
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn top_n_empty_input() {
        let data: Vec<i32> = vec![];
        let result = top_n_by_key(&data, 5, |x| *x);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn group_by_first_char_basic() {
        let words = &["apple", "ant", "banana", "berry", "apricot"];
        let mut map = group_by_first_char(words);

        let mut a_words: Vec<String> = map.remove(&'a').unwrap();
        a_words.sort();
        assert_eq!(a_words, vec!["ant", "apple", "apricot"]);

        let b_words: Vec<String> = map.remove(&'b').unwrap();
        assert_eq!(b_words, vec!["banana", "berry"]);
    }

    #[test]
    fn group_by_first_char_empty_input() {
        let map = group_by_first_char(&[]);
        assert!(map.is_empty());
    }

    #[test]
    fn group_by_first_char_case_sensitive_first_char() {
        // "Apple" and "apple" have different first chars (A vs a)
        let words = &["Apple", "ant"];
        let map = group_by_first_char(words);
        assert!(map.contains_key(&'A'));
        assert!(map.contains_key(&'a'));
    }

    #[test]
    fn running_totals_basic() {
        let input = vec![1, 1, 1, 2, 2, 3, 3, 3, 3];
        let result = running_totals(input.into_iter());
        assert_eq!(result, vec![(1, 3), (2, 2), (3, 4)]);
    }

    #[test]
    fn running_totals_empty_iterator() {
        let input: Vec<i32> = vec![];
        let result = running_totals(input.into_iter());
        assert_eq!(result, vec![]);
    }

    #[test]
    fn running_totals_single_element() {
        let result = running_totals(vec![42].into_iter());
        assert_eq!(result, vec![(42, 1)]);
    }
}
