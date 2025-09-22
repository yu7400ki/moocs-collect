use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Generic cache entry with TTL (Time To Live)
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    value: T,
    expires_at: Instant,
}

impl<T> CacheEntry<T> {
    fn new(value: T, ttl: Duration) -> Self {
        Self {
            value,
            expires_at: Instant::now() + ttl,
        }
    }

    fn is_expired(&self) -> bool {
        Instant::now() > self.expires_at
    }
}

/// Thread-safe cache with TTL support
#[derive(Debug)]
pub struct Cache<K, V> {
    storage: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    default_ttl: Duration,
}

impl<K, V> Clone for Cache<K, V> {
    fn clone(&self) -> Self {
        Self {
            storage: Arc::clone(&self.storage),
            default_ttl: self.default_ttl,
        }
    }
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone + std::fmt::Debug,
    V: Clone,
{
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
            default_ttl,
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let mut storage = self.storage.write().ok()?;

        if let Some(entry) = storage.get(key) {
            if entry.is_expired() {
                storage.remove(key);
                None
            } else {
                Some(entry.value.clone())
            }
        } else {
            None
        }
    }

    pub fn insert(&self, key: K, value: V) {
        self.insert_with_ttl(key, value, self.default_ttl);
    }

    pub fn insert_with_ttl(&self, key: K, value: V, ttl: Duration) {
        if let Ok(mut storage) = self.storage.write() {
            storage.insert(key, CacheEntry::new(value, ttl));
        }
    }

    pub fn remove(&self, key: &K) -> Option<V> {
        self.storage
            .write()
            .ok()
            .and_then(|mut storage| storage.remove(key))
            .map(|entry| entry.value)
    }

    pub fn clear(&self) {
        if let Ok(mut storage) = self.storage.write() {
            storage.clear();
        }
    }

    pub fn cleanup_expired(&self) {
        if let Ok(mut storage) = self.storage.write() {
            storage.retain(|_, entry| !entry.is_expired());
        }
    }

    pub fn len(&self) -> usize {
        self.storage
            .read()
            .map(|storage| storage.len())
            .unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_cache_basic_operations() {
        let cache: Cache<String, i32> = Cache::new(Duration::from_millis(100));

        // Insert and get
        cache.insert("key1".to_string(), 42);
        assert_eq!(cache.get(&"key1".to_string()), Some(42));

        // Non-existent key
        assert_eq!(cache.get(&"nonexistent".to_string()), None);

        // Remove
        assert_eq!(cache.remove(&"key1".to_string()), Some(42));
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_ttl_expiration() {
        let cache: Cache<String, i32> = Cache::new(Duration::from_millis(50));

        cache.insert("key1".to_string(), 42);
        assert_eq!(cache.get(&"key1".to_string()), Some(42));

        // Wait for expiration
        thread::sleep(Duration::from_millis(60));
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_cleanup() {
        let cache: Cache<String, i32> = Cache::new(Duration::from_millis(50));

        cache.insert("key1".to_string(), 42);
        cache.insert("key2".to_string(), 84);
        assert_eq!(cache.len(), 2);

        thread::sleep(Duration::from_millis(60));
        cache.cleanup_expired();
        assert_eq!(cache.len(), 0);
    }
}
