// Day 4: TextAnalyzer with Lifetimes
//
// Build an analyzer that borrows from a source string and returns slices
// instead of allocating new strings.
//
// Learning goals:
//   - Lifetime annotations (`'a`)
//   - Returning borrowed slices (`&'a str`)
//   - Working with string views instead of owned Strings
//   - Edge cases in text processing (empty strings, terminators)

pub struct TextAnalyzer<'a> {
    text: &'a str,
}

impl<'a> TextAnalyzer<'a> {
    pub fn new(text: &'a str) -> Self {
        todo!("Implement new")
    }

    /// Splits the text on ASCII whitespace and returns slices of the original string.
    pub fn words(&self) -> Vec<&'a str> {
        todo!("Implement words")
    }

    /// Returns the longest word, or None if the text is empty.
    /// If there is a tie, return the first one encountered.
    pub fn longest_word(&self) -> Option<&'a str> {
        todo!("Implement longest_word")
    }

    /// Returns only words that start with the given prefix (case-sensitive).
    /// If prefix is empty, returns all words.
    pub fn words_starting_with(&self, prefix: &str) -> Vec<&'a str> {
        todo!("Implement words_starting_with")
    }

    /// Counts the number of sentences. The characters `.`, `!`, `?` are
    /// sentence terminators. Consecutive terminators count as one sentence end.
    pub fn sentence_count(&self) -> usize {
        todo!("Implement sentence_count")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_splits_on_whitespace_only() {
        let text = "hello world  foo\tbar";
        let analyzer = TextAnalyzer::new(text);
        let result = analyzer.words();
        assert_eq!(result, vec!["hello", "world", "foo", "bar"]);
    }

    #[test]
    fn words_on_empty_string_returns_empty_vec() {
        let text = "";
        let analyzer = TextAnalyzer::new(text);
        assert_eq!(analyzer.words(), Vec::<&str>::new());
    }

    #[test]
    fn words_return_slices_of_original_string() {
        let text = "hello world";
        let analyzer = TextAnalyzer::new(text);
        let words = analyzer.words();
        // Verify these are slices into the original string, not new Strings
        assert_eq!(words.len(), 2);
        // Each word's pointer should be within the original text range
        let text_ptr = text.as_ptr();
        for word in &words {
            let word_ptr = word.as_ptr();
            assert!(word_ptr >= text_ptr);
            unsafe {
                assert!(word_ptr.offset(word.len() as isize) <= text_ptr.offset(text.len() as isize));
            }
        }
    }

    #[test]
    fn longest_word_on_empty_string_returns_none() {
        let text = "";
        let analyzer = TextAnalyzer::new(text);
        assert_eq!(analyzer.longest_word(), None);
    }

    #[test]
    fn longest_word_returns_actual_longest() {
        let text = "short looooong medium";
        let analyzer = TextAnalyzer::new(text);
        assert_eq!(analyzer.longest_word(), Some("looooong"));
    }

    #[test]
    fn longest_word_returns_first_on_tie() {
        let text = "aaa bbb ccc";
        let analyzer = TextAnalyzer::new(text);
        // All length 3, first is "aaa"
        assert_eq!(analyzer.longest_word(), Some("aaa"));
    }

    #[test]
    fn words_starting_with_filters_by_prefix() {
        let text = "apple apricot banana";
        let analyzer = TextAnalyzer::new(text);
        let result = analyzer.words_starting_with("ap");
        assert_eq!(result, vec!["apple", "apricot"]);
    }

    #[test]
    fn words_starting_with_is_case_sensitive() {
        let text = "Apple apricot APPLE";
        let analyzer = TextAnalyzer::new(text);
        let result = analyzer.words_starting_with("A");
        // Only "Apple" and "APPLE" start with uppercase A
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"Apple"));
        assert!(result.contains(&"APPLE"));
    }

    #[test]
    fn words_starting_with_empty_prefix_returns_all_words() {
        let text = "hello world";
        let analyzer = TextAnalyzer::new(text);
        let result = analyzer.words_starting_with("");
        assert_eq!(result, vec!["hello", "world"]);
    }

    #[test]
    fn sentence_count_counts_terminators() {
        let text = "Hello. How are you? I am fine!";
        let analyzer = TextAnalyzer::new(text);
        assert_eq!(analyzer.sentence_count(), 3);
    }

    #[test]
    fn sentence_count_consecutive_terminators_count_as_one() {
        let text = "Hello!! How are you??? Fine.";
        let analyzer = TextAnalyzer::new(text);
        // "Hello!!" -> 1, "How are you???" -> 1, "Fine." -> 1
        assert_eq!(analyzer.sentence_count(), 3);
    }

    #[test]
    fn sentence_count_no_terminators_returns_zero() {
        let text = "hello world no punctuation here";
        let analyzer = TextAnalyzer::new(text);
        assert_eq!(analyzer.sentence_count(), 0);
    }

    #[test]
    fn words_returns_slices_not_strings() {
        let text = String::from("hello world rust");
        let analyzer = TextAnalyzer::new(&text);
        let words = analyzer.words();
        // Verify these are &str references, not owned Strings
        // This is a compile-time test: the return type is Vec<&'a str>
        // We just confirm it compiles and returns the right values
        assert_eq!(words, vec!["hello", "world", "rust"]);
    }
}
