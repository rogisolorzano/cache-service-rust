pub type UnixTimestampSeconds = String;

pub struct CacheConfig {
    /// The expiry timestamp in seconds.
    expiry_timestamp: UnixTimestampSeconds,
}

pub trait ICacheStrategy {
    /// Initializes and returns a new instance of the implementer.
    fn new() -> Self;

    /// Get the value for a key.
    fn get(&self, key: &str) -> String;

    /// Set the value for a key, with cache configuration.
    fn set(&self, key: &str, value: &str, config: CacheConfig);

    /// Delete the value for a key.
    fn delete(&self, key: &str);
}
