pub type Seconds = u32;

pub struct CacheConfig {
    /// The TTL of the cache value.
    ttl: Seconds,
}
