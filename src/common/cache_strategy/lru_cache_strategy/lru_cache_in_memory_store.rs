use std::sync::{Arc, Mutex, Once};
use std::{mem};

/// This is a simple, thread-safe in-memory data store. Uses O(n) space.
/// Using ARC for reference counting and providing shared ownership to the Mutexes.
/// Mutex handles locking, ensures thread-safety while accessing the in-memory store.
#[derive(Clone)]
pub struct LruCacheInMemoryStore {
    pub inner: Arc<Mutex<Vec<String>>>,
    // TODO: implement structures that will be used for LruCache
    // pub keyNodeMap: Arc<Mutex<HashMap<String, DoublyLinkedListNode>>>,
    // pub linkedList: Arc<Mutex<DoublyLinkedList>>, // (holds pointers to head and tail)
}

impl LruCacheInMemoryStore {
    // Get the LruCacheInMemoryStore singleton.
    pub fn shared_instance() -> LruCacheInMemoryStore {
        // "static" means that this variable will outlive the function - it will not be destroyed.
        // "mut" means that we can mutate the underlying data, however rust requires that we do this
        // in an "unsafe" block to ensure we are doing this very intentionally.
        //
        // "*const LruCacheInMemoryStore" means that the pointer address is modifiable, however the
        // underlying data is "constant" (It can still be modified unsafely, though).
        //
        // We set the data in this memory location to 0 bits by assigning to 0. We're basically
        // setting the value of STORE_SINGLETON to "nothing" here.
        static mut STORE_SINGLETON: *const LruCacheInMemoryStore = 0 as *const LruCacheInMemoryStore;
        static ONCE: Once = Once::new();

        unsafe {
            // We make sure initialization of the memory store only happens once across all threads
            // and for the lifetime of the entire process.
            ONCE.call_once(|| {
                let store = LruCacheInMemoryStore{
                    inner: Arc::new(Mutex:: new(Vec::new())),
                };

                // STORE_SINGLETON is currently a pointer that was initialized to nothing.
                // Here, we copy the raw bytes of a Box which contains 'store' into STORE_SINGLETON.
                STORE_SINGLETON = mem::transmute(Box::new(store));
            });

            // "Clones" but does not "copy" or double memory usage. Essentially creates a new, safe
            // reference to it. The underlying allocation/resources are reused. We can't return
            // *STORE_SINGLETON directly since it would attempt to move a "static" item.
            return (*STORE_SINGLETON).clone();
        }
    }

}