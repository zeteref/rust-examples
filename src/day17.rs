// Day 14: KeyValueStore<K, V>
//
// Build a versioned key-value store that tracks history, supports rollback,
// and implements time-based querying.
//
// Learning goals:
//   - Complex generic types with multiple trait bounds
//   - Implementing `IntoIterator` for a reference to a custom type
//   - Custom iterator structs with lifetimes
//   - Timestamp-based version tracking
//   - Combining HashMap + Vec for versioned storage

use std::collections::HashMap;
use std::hash::Hash;

pub struct KeyValueStore<K, V> {
    // Define internal storage: a counter for timestamps and a hash map
    // mapping each key to a list of (value, timestamp) pairs.
    // The last entry in each list is the current value.
    _marker: std::marker::PhantomData<(K, V)>,
}

impl<K: Eq + Hash + Clone, V: Clone> KeyValueStore<K, V> {
    pub fn new() -> Self {
        todo!("Implement new")
    }

    /// Returns a reference to the current value for this key.
    pub fn get(&self, key: &K) -> Option<&V> {
        todo!("Implement get")
    }

    /// Sets the current value for this key, recording a new version.
    pub fn set(&mut self, key: K, value: V) {
        todo!("Implement set")
    }

    /// Returns all historical values for this key in insertion order,
    /// or None if the key does not exist.
    pub fn history(&self, key: &K) -> Option<Vec<&V>> {
        todo!("Implement history")
    }

    /// Removes the latest value for this key and returns it.
    /// If only one value remains, the key is fully removed and `get` will
    /// return None afterward.
    /// Returns None if the key does not exist.
    pub fn rollback(&mut self, key: &K) -> Option<V> {
        todo!("Implement rollback")
    }

    /// Returns all keys that have been modified at or after the given
    /// timestamp.
    pub fn keys_modified_since(&self, timestamp: u64) -> Vec<&K> {
        todo!("Implement keys_modified_since")
    }
}

/// An iterator that yields key-value pairs from a KeyValueStore.
pub struct KeyValueIter<'a, K, V> {
    // Define an iterator that wraps a HashMap iterator and yields (&K, &V)
    _marker: std::marker::PhantomData<(&'a K, &'a V)>,
}

impl<'a, K: Eq + Hash + Clone, V: Clone> Iterator for KeyValueIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Implement Iterator::next for KeyValueIter")
    }
}

impl<'a, K: Eq + Hash + Clone, V: Clone> IntoIterator for &'a KeyValueStore<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = KeyValueIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        todo!("Implement IntoIterator for &KeyValueStore")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_on_nonexistent_key_returns_none() {
        let store: KeyValueStore<String, i32> = KeyValueStore::new();
        assert_eq!(store.get(&"foo".to_string()), None);
    }

    #[test]
    fn set_then_get_returns_latest_value() {
        let mut store = KeyValueStore::new();
        store.set("key".to_string(), 42);
        assert_eq!(store.get(&"key".to_string()), Some(&42));
    }

    #[test]
    fn set_same_key_twice_returns_second_value() {
        let mut store = KeyValueStore::new();
        store.set("key".to_string(), 10);
        store.set("key".to_string(), 20);
        assert_eq!(store.get(&"key".to_string()), Some(&20));
    }

    #[test]
    fn history_on_nonexistent_key_returns_none() {
        let store: KeyValueStore<String, i32> = KeyValueStore::new();
        assert_eq!(store.history(&"foo".to_string()), None);
    }

    #[test]
    fn history_returns_values_in_insertion_order() {
        let mut store = KeyValueStore::new();
        store.set("key".to_string(), 1);
        store.set("key".to_string(), 2);
        store.set("key".to_string(), 3);

        let hist = store.history(&"key".to_string()).unwrap();
        assert_eq!(hist, vec![&1, &2, &3]);
    }

    #[test]
    fn rollback_on_nonexistent_key_returns_none() {
        let mut store: KeyValueStore<String, i32> = KeyValueStore::new();
        assert_eq!(store.rollback(&"foo".to_string()), None);
    }

    #[test]
    fn rollback_removes_latest_value_and_returns_it() {
        let mut store = KeyValueStore::new();
        store.set("key".to_string(), 1);
        store.set("key".to_string(), 2);
        store.set("key".to_string(), 3);

        let rolled = store.rollback(&"key".to_string());
        assert_eq!(rolled, Some(3));
        assert_eq!(store.get(&"key".to_string()), Some(&2));
    }

    #[test]
    fn rollback_on_key_with_one_value_makes_get_return_none() {
        let mut store = KeyValueStore::new();
        store.set("key".to_string(), 42);

        let rolled = store.rollback(&"key".to_string());
        assert_eq!(rolled, Some(42));
        assert_eq!(store.get(&"key".to_string()), None);
    }

    #[test]
    fn rollback_on_key_with_two_values_makes_get_return_first() {
        let mut store = KeyValueStore::new();
        store.set("key".to_string(), 100);
        store.set("key".to_string(), 200);

        store.rollback(&"key".to_string());
        assert_eq!(store.get(&"key".to_string()), Some(&100));

        store.rollback(&"key".to_string());
        assert_eq!(store.get(&"key".to_string()), None);
    }

    #[test]
    fn keys_modified_since_returns_keys_with_activity_after_timestamp() {
        let mut store = KeyValueStore::new();
        store.set("a".to_string(), 1);
        store.set("b".to_string(), 2);
        store.set("c".to_string(), 3);

        // All keys should have timestamps >= 0
        let recent = store.keys_modified_since(0);
        assert_eq!(recent.len(), 3);
        assert!(recent.contains(&&"a".to_string()));
        assert!(recent.contains(&&"b".to_string()));
        assert!(recent.contains(&&"c".to_string()));
    }

    #[test]
    fn keys_modified_since_high_timestamp_returns_empty() {
        let mut store: KeyValueStore<String, i32> = KeyValueStore::new();
        store.set("a".to_string(), 1);

        let recent = store.keys_modified_since(1_000_000);
        assert!(recent.is_empty());
    }

    #[test]
    fn iterator_yields_one_pair_per_key_current_value_only() {
        let mut store = KeyValueStore::new();
        store.set("a".to_string(), 1);
        store.set("b".to_string(), 2);
        store.set("c".to_string(), 3);

        let mut pairs: Vec<(&String, &i32)> = vec![];
        for (k, v) in &store {
            pairs.push((k, v));
        }
        pairs.sort_by_key(|(k, _)| k.as_str());

        assert_eq!(pairs.len(), 3);
        assert_eq!(*pairs[0].1, 1);
        assert_eq!(*pairs[1].1, 2);
        assert_eq!(*pairs[2].1, 3);
    }

    #[test]
    fn iterator_does_not_yield_fully_rolled_back_keys() {
        let mut store = KeyValueStore::new();
        store.set("a".to_string(), 1);
        store.set("b".to_string(), 2);
        store.rollback(&"a".to_string()); // "a" is fully removed

        let mut keys: Vec<&String> = vec![];
        for (k, _) in &store {
            keys.push(k);
        }
        assert_eq!(keys.len(), 1);
        assert_eq!(*keys[0], "b".to_string());
    }

    #[test]
    fn generic_works_with_different_types() {
        // Must compile and work with String keys and i32 values
        let mut store1: KeyValueStore<String, i32> = KeyValueStore::new();
        store1.set("test".to_string(), 42);
        assert_eq!(store1.get(&"test".to_string()), Some(&42));

        // Must compile and work with i32 keys and String values
        let mut store2: KeyValueStore<i32, String> = KeyValueStore::new();
        store2.set(1, "one".to_string());
        assert_eq!(store2.get(&1), Some(&"one".to_string()));

        // Must compile with String keys and String values
        let mut store3: KeyValueStore<String, String> = KeyValueStore::new();
        store3.set("greeting".to_string(), "hello".to_string());
        assert_eq!(store3.get(&"greeting".to_string()), Some(&"hello".to_string()));
    }
}
