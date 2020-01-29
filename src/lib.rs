use std::{collections::hash_map::RandomState, hash::*};

pub struct Rulette<K: Eq + Hash, V, S = RandomState> {
    storage: Vec<Option<(K, V)>>,
    hasher: S,
}

impl<K: Eq + Hash, V, S: BuildHasher + Default> Rulette<K, V, S> {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut storage = Vec::with_capacity(capacity);
        for _ in 0..storage.capacity() {
            storage.push(None);
        }
        let hasher = S::default();
        Rulette { storage, hasher }
    }

    pub fn clear(&mut self) {
        for e in self.storage.iter_mut() {
            *e = None;
        }
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> Rulette<K, V, S> {
    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        let mut storage = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            storage.push(None);
        }
        Rulette { storage, hasher }
    }

    fn get_ix(&self, k: &K) -> usize {
        let mut hasher = self.hasher.build_hasher();
        k.hash(&mut hasher);
        let hash = hasher.finish();
        (hash % (self.storage.len() as u64)) as usize
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        let ix = self.get_ix(k);
        self.storage[ix]
            .as_ref()
            .and_then(|(ik, v)| if k == ik { Some(v) } else { None })
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        let ix = self.get_ix(k);
        self.storage[ix]
            .as_mut()
            .and_then(|(ik, v)| if k == ik { Some(v) } else { None })
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<(K, V)> {
        let ix = self.get_ix(&k);
        self.storage[ix].replace((k, v))
    }

    pub fn remove(&mut self, k: &K) -> Option<(K, V)> {
        let ix = self.get_ix(&k);
        let (ik, _) = self.storage[ix].as_ref()?;
        if ik == k {
            self.storage[ix].take()
        } else {
            None
        }
    }
}
