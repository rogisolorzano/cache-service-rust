## Cache Service Rust

This is a caching service built with Rust! This service is not intended to be production ready.
It's just a really fun excuse for me to finally try Rust.

The service uses a simple, thread-safe in-memory data store. The underlying data structures
are a hashmap and a doubly linked list. It is essentially an LRU cache, and achieves gets, sets
and deletions in constant time. Uses lazy expiration to enforce item expiry. This means that 
items aren't expired until they are read.

## Getting Started

1. Install Rust on your local machine. The recommended installation method is using rustup. The 
   [official installation guide](https://www.rust-lang.org/tools/install) gives a great overview.
2. Run `cargo run`. This installs dependencies, compiles everything and runs the service. You
   might need to manually set cargo into your path.
3. Go to [http://localhost:8080](http://localhost:8080). That's it!

## API

---
### Get
Get a value from cache for a key.
```http request
GET /:key
```
---
### Set
Set a value in cache for a key.
```http request
PUT /
```

**Request Body**
```json
{
   "key": "myKey",
   "value": "myValue"
}
```

---
### Delete
Delete a key from cache.
```http request
DELETE /:key
```

## Deployment

WIP
