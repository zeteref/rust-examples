pub fn process_numbers(numbers: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    result.sort_by(|a, b| b.cmp(a));
    result
}

pub fn top_n_by_key<'a, T, K: Ord>(
    items: &'a [T],
    n: usize,
    key_fn: impl Fn(&T) -> K,
) -> Vec<&'a T> {
    let mut refs: Vec<&T> = items.iter().collect();
    refs.sort_by(|a, b| key_fn(b).cmp(&key_fn(a)));
    refs.truncate(n);
    refs
}

pub fn group_by_first_char(words: &[&str]) -> std::collections::HashMap<char, Vec<String>> {
    let mut map: std::collections::HashMap<char, Vec<String>> =
        std::collections::HashMap::new();
    for word in words {
        if let Some(first_char) = word.chars().next() {
            map.entry(first_char)
                .or_insert_with(Vec::new)
                .push((*word).to_string());
        }
    }
    for words in map.values_mut() {
        words.sort();
    }
    map
}

pub fn running_totals<I: Iterator<Item = i32>>(iter: I) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    let mut current_val: Option<i32> = None;
    let mut count: i32 = 0;

    for val in iter {
        match current_val {
            Some(cv) if cv == val => {
                count += 1;
            }
            _ => {
                if let Some(cv) = current_val {
                    result.push((cv, count));
                }
                current_val = Some(val);
                count = 1;
            }
        }
    }
    if let Some(cv) = current_val {
        result.push((cv, count));
    }
    result
}
