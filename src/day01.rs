// Day 1: Word Frequency
//
// Implement a function that counts word occurrences in a string.
//
// Learning goals:
//   - String manipulation (split, collect)
//   - Vec and tuple usage
//   - Sorting with custom comparators
//   - Handling edge cases (empty input, punctuation, case)

/// Returns words and their frequency, sorted by frequency descending.
/// Ties are broken alphabetically ascending.
///
/// Rules:
///   - Punctuation characters `.,!?;:'\"()-` are delimiters, not part of words
///   - Whitespace also acts as a delimiter
///   - Words are lowercased before counting (case-insensitive)
///   - Words shorter than 2 characters are excluded
///   - Multiple consecutive delimiters do not produce empty words
pub fn word_frequency(text: &str) -> Vec<(String, usize)> {
    todo!("Implement word_frequency")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_returns_empty_vec() {
        assert_eq!(word_frequency(""), vec![]);
    }

    #[test]
    fn single_word_returns_count_one() {
        let result = word_frequency("hello");
        assert_eq!(result, vec![("hello".to_string(), 1)]);
    }

    #[test]
    fn multiple_occurrences_are_counted() {
        let result = word_frequency("the cat sat on the mat the cat");
        // "the": 3, "cat": 2, "mat": 1, "on": 1, "sat": 1
        assert_eq!(result[0], ("the".to_string(), 3));
        assert_eq!(result[1], ("cat".to_string(), 2));
        // Ties broken alphabetically
        let tied: Vec<&str> = result.iter().skip(2).map(|(w, _)| w.as_str()).collect();
        assert_eq!(tied, vec!["mat", "on", "sat"]);
    }

    #[test]
    fn punctuation_is_treated_as_delimiter() {
        let result = word_frequency("hello, world! hello? world.");
        assert_eq!(result[0], ("hello".to_string(), 2));
        assert_eq!(result[1], ("world".to_string(), 2));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn words_are_lowercased_before_counting() {
        let result = word_frequency("Hello HELLO hello HeLlO");
        assert_eq!(result, vec![("hello".to_string(), 4)]);
    }

    #[test]
    fn words_shorter_than_two_chars_are_excluded() {
        let result = word_frequency("a I be to go hi ok a I");
        // "a" (1 char) and "I" (1 char) are excluded
        // "be", "to", "go", "hi", "ok" all 2+ chars, each appears once
        assert_eq!(result.len(), 5);
        let words: Vec<&str> = result.iter().map(|(w, _)| w.as_str()).collect();
        assert_eq!(words, vec!["be", "go", "hi", "ok", "to"]);
    }

    #[test]
    fn consecutive_delimiters_do_not_produce_empty_words() {
        let result = word_frequency("hello,,,   ...world");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, "hello");
        assert_eq!(result[1].0, "world");
    }

    #[test]
    fn ties_sorted_alphabetically_ascending() {
        let result = word_frequency("zzz aaa yyy bbb");
        // All appear once, sorted alphabetically
        let words: Vec<&str> = result.iter().map(|(w, _)| w.as_str()).collect();
        assert_eq!(words, vec!["aaa", "bbb", "yyy", "zzz"]);
    }

    #[test]
    fn mixed_case_and_punctuation() {
        let result = word_frequency("The cat sat on the mat, The Cat!");
        // "the": 3, "cat": 2, "mat": 1, "on": 1, "sat": 1
        assert_eq!(result[0], ("the".to_string(), 3));
        assert_eq!(result[1], ("cat".to_string(), 2));
        let tied: Vec<&str> = result.iter().skip(2).map(|(w, _)| w.as_str()).collect();
        assert_eq!(tied, vec!["mat", "on", "sat"]);
    }

    #[test]
    fn numbers_and_digits_are_delimiters() {
        // Numbers/symbols not in the alphabet should be delimiters
        let result = word_frequency("abc123def 456 ghi");
        // "abc", "def", "ghi" are words; "123", "456" are numeric delimiters
        assert_eq!(result.len(), 3);
        let words: Vec<&str> = result.iter().map(|(w, _)| w.as_str()).collect();
        assert_eq!(words, vec!["abc", "def", "ghi"]);
    }
}
