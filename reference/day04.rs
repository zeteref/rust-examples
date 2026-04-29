pub struct TextAnalyzer<'a> {
    text: &'a str,
}

impl<'a> TextAnalyzer<'a> {
    pub fn new(text: &'a str) -> Self {
        TextAnalyzer { text }
    }

    pub fn words(&self) -> Vec<&'a str> {
        self.text.split_whitespace().collect()
    }

    pub fn longest_word(&self) -> Option<&'a str> {
        self.words().into_iter().fold(None, |longest, word| match longest {
            None => Some(word),
            Some(l) if word.len() > l.len() => Some(word),
            _ => longest,
        })
    }

    pub fn words_starting_with(&self, prefix: &str) -> Vec<&'a str> {
        self.words()
            .into_iter()
            .filter(|w| w.starts_with(prefix))
            .collect()
    }

    pub fn sentence_count(&self) -> usize {
        let mut count = 0;
        let mut in_sentence = false;
        for ch in self.text.chars() {
            if ch == '.' || ch == '!' || ch == '?' {
                if !in_sentence {
                    count += 1;
                    in_sentence = true;
                }
            } else {
                in_sentence = false;
            }
        }
        count
    }
}
