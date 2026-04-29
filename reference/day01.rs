pub fn word_frequency(text: &str) -> Vec<(String, usize)> {
    use std::collections::HashMap;

    let mut counts: HashMap<String, usize> = HashMap::new();
    for word in text.split(|c: char| !c.is_alphabetic()) {
        if word.len() >= 2 {
            *counts.entry(word.to_lowercase()).or_insert(0) += 1;
        }
    }
    let mut result: Vec<(String, usize)> = counts.into_iter().collect();
    result.sort_by(|(w1, c1), (w2, c2)| c2.cmp(c1).then_with(|| w1.cmp(w2)));
    result
}
