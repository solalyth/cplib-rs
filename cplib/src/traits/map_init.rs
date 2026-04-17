use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

pub trait MapInit {
    type K;
    type V;
    fn init(&mut self, key: Self::K, init: Self::V) -> &mut Self::V;
    fn init_with(&mut self, key: Self::K, init: impl FnOnce() -> Self::V) -> &mut Self::V;
}

impl<K: Eq + Hash, V> MapInit for HashMap<K, V> {
    type K = K;
    type V = V;
    fn init(&mut self, key: K, init: V) -> &mut V {
        self.entry(key).or_insert(init)
    }
    fn init_with(&mut self, key: K, init: impl FnOnce() -> Self::V) -> &mut V {
        self.entry(key).or_insert_with(init)
    }
}

impl<K: Ord, V> MapInit for BTreeMap<K, V> {
    type K = K;
    type V = V;
    fn init(&mut self, key: K, init: V) -> &mut V {
        self.entry(key).or_insert(init)
    }
    fn init_with(&mut self, key: Self::K, init: impl FnOnce() -> Self::V) -> &mut Self::V {
        self.entry(key).or_insert_with(init)
    }
}
