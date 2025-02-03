use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, SystemTime};

struct CacheEntry<T> {
    value: T,
    timestamp: SystemTime,
}

impl<T> CacheEntry<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            timestamp: SystemTime::now(),
        }
    }
}

struct CacheOptions {
    //time to live
    ttl: Duration,

    max_size: usize,
}

struct Cache<K, V> {
    store: HashMap<K, CacheEntry<V>>,
    options: CacheOptions,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn new(options: CacheOptions) -> Self {
        Self {
            store: HashMap::new(),
            options,
        }
    }

    fn get(&mut self, key: K) -> Option<V> {
        let entry = self.store.get(&key);
        if let Some(entry) = entry {
            if self.is_expired(entry) {
                return None;
            } else {
                return Some(entry.value.clone());
            }
        }
        None
    }

    fn set(&mut self, key: K, value: V) {
        self.remove_expired();
        let is_full = self.store.len() == self.options.max_size;

        if is_full {
            self.remove_oldest();
        }
        let value = CacheEntry::new(value);
        self.store.insert(key, value);
    }

    fn clear(&mut self) {
        self.store.clear();
    }

    fn size(&self) -> usize {
        self.store.len()
    }

    fn is_expired(&self, entry: &CacheEntry<V>) -> bool {
        let now = SystemTime::now();

        if let Ok(elapsed) = now.duration_since(entry.timestamp) {
            elapsed >= self.options.ttl
        } else {
            false
        }
    }

    fn remove_oldest(&mut self) {
        //fix this
        if let Some(oldest_key) = self.find_oldest() {
            self.store.remove(&oldest_key);
        }
    }

    fn find_oldest(&self) -> Option<K> {
        self.store
            .iter()
            .min_by_key(|(_, entry)| entry.timestamp)
            .map(|(key, _)| key.clone())
    }

    fn remove_expired(&mut self) {
        let now = SystemTime::now();

        self.store.retain(|_, entry| {
            if let Ok(elapsed) = now.duration_since(entry.timestamp) {
                elapsed < self.options.ttl
            } else {
                false
            }
        });
    }
}

fn main() {
    let options = CacheOptions {
        ttl: Duration::from_secs(10),
        max_size: 3,
    };

    let mut cache: Cache<u32, &str> = Cache::new(options);
    let name1 = "Mike Willis";
    cache.set(1, name1);
    let value1 = cache.get(1);
    assert_eq!(value1.unwrap(), name1);

    assert_eq!(cache.size(), 1);
    cache.clear();

    println!("Done, world!");
}
