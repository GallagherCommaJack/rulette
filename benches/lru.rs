use lru::LruCache;
use rand::{distributions::Distribution, Rng};
use rulette::Rulette;
use std::{hash::Hash, time::Duration};
use zipf::ZipfDistribution;

trait Cache<K, V> {
    fn write(&mut self, k: K, v: V);
    fn read(&mut self, k: &K) -> Option<&V>;
    fn del(&mut self, k: &K) -> Option<V>;
    fn del_all(&mut self);
}

impl<K: Hash + Eq, V> Cache<K, V> for Rulette<K, V> {
    fn write(&mut self, k: K, v: V) {
        self.insert(k, v);
    }
    fn read(&mut self, k: &K) -> Option<&V> {
        self.get(k)
    }
    fn del(&mut self, k: &K) -> Option<V> {
        self.remove(k).map(|(_, v)| v)
    }
    fn del_all(&mut self) {
        self.clear()
    }
}

impl<K: Hash + Eq, V> Cache<K, V> for LruCache<K, V> {
    fn write(&mut self, k: K, v: V) {
        self.put(k, v);
    }
    fn read(&mut self, k: &K) -> Option<&V> {
        self.get(k)
    }
    fn del(&mut self, k: &K) -> Option<V> {
        self.pop(k)
    }
    fn del_all(&mut self) {
        self.clear()
    }
}

fn do_inserts<K, V, C: Cache<K, V>, I: IntoIterator<Item = (K, V)>>(cache: &mut C, pairs: I) {
    for (k, v) in pairs {
        cache.write(k, v);
    }
}

struct Stats {
    misses: usize,
    hits: usize,
}

fn do_lookups<K, V, C: Cache<K, V>, I: IntoIterator<Item = K>, F: FnMut(&K) -> V>(
    cache: &mut C,
    keys: I,
    mut f: F,
) -> Stats {
    let mut out = Stats { misses: 0, hits: 0 };
    for k in keys {
        if cache.read(&k).is_some() {
            out.hits += 1;
        } else {
            out.misses += 1;
            let v = f(&k);
            cache.write(k, v);
        }
    }
    out
}

fn run_bench_sleep<C: Cache<usize, usize>>(
    cache: &mut C,
    sleep_dur: Duration,
    items: &[usize],
) -> Stats {
    let mut rng = rand::thread_rng();
    let mk_val = move |_: &usize| {
        std::thread::sleep(sleep_dur);
        rng.gen::<usize>()
    };
    do_lookups(cache, items.iter().map(|k| *k), mk_val)
}

fn zipf_gen_items(
    num_elements: usize,
    exponent: f64,
    width: usize,
    depth: usize,
) -> Vec<Vec<usize>> {
    let mut rng = rand::thread_rng();
    let zipf = ZipfDistribution::new(num_elements, exponent).expect("failed to create zipf distr");
    let mut out = Vec::with_capacity(width);
    for _ in 0..width {
        let mut column = Vec::with_capacity(depth);
        for _ in 0..depth {
            column.push(zipf.sample(&mut rng));
        }
        out.push(column);
    }
    out
}
