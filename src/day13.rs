// Day 13: Declarative Macros
//
// Write two declarative macros: `assert_between!` for range assertions and
// `hashmap!` for concise HashMap construction.
//
// Learning goals:
//   - `macro_rules!` syntax
//   - Repetition patterns (`$()*`, `$(),*`)
//   - Macro hygiene and scoping
//   - Working with `std::fmt::Debug` and `std::cmp::PartialOrd` bounds

/// Asserts that a value is within an inclusive range [low, high].
///
/// Panics with a descriptive message if the value is outside the range.
///
/// # Examples
/// ```
/// assert_between!(5, 1, 10);  // passes
/// assert_between!(0, 1, 10);  // panics
/// ```
///
/// Works with any type implementing `PartialOrd + Debug`.
macro_rules! assert_between {
    ($val:expr, $low:expr, $high:expr) => {
        todo!("Implement assert_between! macro")
    };
}

/// Creates a HashMap from a list of key-value pairs.
///
/// # Examples
/// ```
/// let map = hashmap!{ "a" => 1, "b" => 2 };
/// assert_eq!(map["a"], 1);
/// ```
///
/// The result type is `HashMap<K, V>`, where K and V are inferred.
/// Supports empty maps: `hashmap!{}`
macro_rules! hashmap {
    {} => {
        todo!("Implement empty hashmap! case")
    };
    ($($key:expr => $val:expr),+ $(,)?) => {
        todo!("Implement hashmap! macro for key-value pairs")
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn assert_between_value_in_range_does_not_panic() {
        assert_between!(5, 1, 10);
    }

    #[test]
    fn assert_between_value_at_lower_bound_does_not_panic() {
        assert_between!(1, 1, 10);
    }

    #[test]
    fn assert_between_value_at_upper_bound_does_not_panic() {
        assert_between!(10, 1, 10);
    }

    #[test]
    #[should_panic]
    fn assert_between_below_range_panics() {
        assert_between!(0, 1, 10);
    }

    #[test]
    #[should_panic]
    fn assert_between_above_range_panics() {
        assert_between!(11, 1, 10);
    }

    #[test]
    fn assert_between_works_with_f64() {
        assert_between!(5.5f64, 1.0, 10.0);
    }

    #[test]
    #[should_panic]
    fn assert_between_fails_with_f64_below() {
        assert_between!(0.5f64, 1.0, 10.0);
    }

    #[test]
    fn hashmap_creates_correct_map() {
        let map: HashMap<&str, i32> = hashmap! { "a" => 1, "b" => 2 };
        assert_eq!(map.len(), 2);
        assert_eq!(map["a"], 1);
        assert_eq!(map["b"], 2);
    }

    #[test]
    fn hashmap_empty_creates_empty_map() {
        let map: HashMap<&str, i32> = hashmap! {};
        assert!(map.is_empty());
    }

    #[test]
    fn hashmap_works_with_string_keys() {
        let map: HashMap<String, i32> = hashmap! {
            String::from("hello") => 42,
            String::from("world") => 100,
        };
        assert_eq!(map[&String::from("hello")], 42);
        assert_eq!(map[&String::from("world")], 100);
    }

    #[test]
    fn hashmap_works_with_integer_keys() {
        let map: HashMap<i32, &str> = hashmap! { 1 => "one", 2 => "two", 3 => "three" };
        assert_eq!(map.len(), 3);
        assert_eq!(map[&1], "one");
    }

    #[test]
    fn hashmap_works_with_complex_values() {
        let map: HashMap<&str, Vec<&str>> = hashmap! {
            "nums" => vec!["1", "2", "3"],
            "letters" => vec!["a", "b", "c"],
        };
        assert_eq!(map["nums"], vec!["1", "2", "3"]);
        assert_eq!(map["letters"], vec!["a", "b", "c"]);
    }

    #[test]
    fn hashmap_type_is_hashmap_not_newtype() {
        let map = hashmap! { 'x' => 1 };
        // This must compile—verifies the type is HashMap, not a wrapper
        let _: HashMap<char, i32> = map;
    }

    #[test]
    fn hashmap_trailing_comma_is_accepted() {
        let map: HashMap<&str, i32> = hashmap! { "a" => 1, };
        assert_eq!(map.len(), 1);
        assert_eq!(map["a"], 1);
    }
}
