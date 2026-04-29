use std::collections::HashMap;
use std::hash::Hash;

pub struct KeyValueStore<K, V> {
    data: HashMap<K, Vec<(V, u64)>>,
    counter: u64,
}

impl<K: Eq + Hash + Clone, V: Clone> KeyValueStore<K, V> {
    pub fn new() -> Self {
        KeyValueStore {
            data: HashMap::new(),
            counter: 0,
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key).and_then(|entries| entries.last().map(|(v, _)| v))
    }

    pub fn set(&mut self, key: K, value: V) {
        self.counter += 1;
        self.data
            .entry(key)
            .or_insert_with(Vec::new)
            .push((value, self.counter));
    }

    pub fn history(&self, key: &K) -> Option<Vec<&V>> {
        self.data
            .get(key)
            .map(|entries| entries.iter().map(|(v, _)| v).collect())
    }

    pub fn rollback(&mut self, key: &K) -> Option<V> {
        if let Some(entries) = self.data.get_mut(key) {
            let (value, _) = entries.pop()?;
            if entries.is_empty() {
                self.data.remove(key);
            }
            Some(value)
        } else {
            None
        }
    }

    pub fn keys_modified_since(&self, timestamp: u64) -> Vec<&K> {
        self.data
            .iter()
            .filter(|(_, entries)| {
                entries.iter().any(|(_, ts)| *ts >= timestamp)
            })
            .map(|(k, _)| k)
            .collect()
    }
}

pub struct KeyValueIter<'a, K, V> {
    inner: std::collections::hash_map::Iter<'a, K, Vec<(V, u64)>>,
}

impl<'a, K: Eq + Hash + Clone, V: Clone> Iterator for KeyValueIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(k, entries)| (k, &entries.last().unwrap().0))
    }
}

impl<'a, K: Eq + Hash + Clone, V: Clone> IntoIterator for &'a KeyValueStore<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = KeyValueIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        KeyValueIter {
            inner: self.data.iter(),
        }
    }
}
