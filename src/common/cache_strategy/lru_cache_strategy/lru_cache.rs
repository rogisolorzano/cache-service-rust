use crate::common::cache_strategy::i_cache_strategy::{ICacheStrategy};
use crate::common::cache_strategy::lru_cache_strategy::lru_cache_in_memory_store::{LruCacheInMemoryStore};
use crate::common::cache_strategy::cache_config::CacheConfig;
use std::sync::MutexGuard;

pub struct LruCache;

/// A cache strategy that uses the Least Recently Used eviction policy. Uses a hashmap and
/// a doubly linked list under the hood to achieve O(1) gets, sets and deletes. Uses O(n) space.
///
/// Uses lazy expiration to enforce item expiry. This means that items aren't expired
/// until they are read.
impl ICacheStrategy for LruCache {
    fn new() -> LruCache {
        LruCache{}
    }

    fn get(&self, _key: &str) -> String {
        // TODO: finish implementation
        let memory_store = LruCacheInMemoryStore::shared_instance();
        let mut values: MutexGuard<Vec<String>> = memory_store.inner.lock().unwrap();
        let value = format!("Value #{}", (*values).len());

        (*values).push(value);

        return format!("{:?}", values);
    }

    fn set(&self, _key: &str, _value: &str, _config: CacheConfig) {
        // TODO: implement
    }

    fn delete(&self, _key: &str) {
        // TODO: implement
    }
}
