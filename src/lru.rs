use crate::{DoublyLL, NodeRef};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// A least recently used (LRU) cache.
///
/// Stores key-value pairs and keeps track of their usage order.
/// When the cache reaches its capacity, the least recently used
/// entry is evicted to make room for a new entry.
///
/// Internally, it uses a `HashMap` for key lookups and a doubly
/// linked list to maintain the usage order.
pub struct LRUCache<K, V> {
    map: HashMap<K, NodeRef<K, V>>,
    doubly_ll: DoublyLL<K, V>,
    size: usize,
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Creates a new LRU cache with the specified capacity.
    ///
    /// # Arguments
    /// * `size` - Maximum number of entries the cache can hold.
    ///
    /// # Returns
    /// A new, empty `LRUCache` instance.
    ///
    /// A capacity of `0` creates a cache that does not store any entries.
    pub fn new(size: usize) -> Self {
        return Self {
            map: HashMap::new(),
            doubly_ll: DoublyLL::new(),
            size,
        };
    }

    /// Retrieves the value associated with the given key.
    ///
    /// If the key exists, its entry is marked as recently used by
    /// moving it to the front of the usage list.
    ///
    /// # Arguments
    /// * `key` - Reference to the key whose value should be retrieved.
    ///
    /// # Returns
    /// * `Some(V)` - A clone of the value associated with the key.
    /// * `None` - If the key does not exist in the cache.
    pub fn get(&mut self, key: &K) -> Option<V> {
        let node = self.map.get(key)?.clone();
        self.doubly_ll.move_to_front(&node);
        let value = node.borrow().value.clone();

        return Some(value);
    }

    /// Inserts or updates a key-value pair in the cache.
    ///
    /// If the key already exists, its value is updated and the entry
    /// is marked as recently used.
    ///
    /// If the key does not exist and the cache is at capacity,
    /// the least recently used entry is evicted before insertion.
    ///
    /// # Arguments
    /// * `key` - Key of the entry to insert or update.
    /// * `value` - Value associated with the key.
    pub fn put(&mut self, key: K, value: V) {
        if self.size == 0 {
            return;
        }

        if let Some(node) = self.map.get(&key).cloned() {
            node.borrow_mut().value = value;
            self.doubly_ll.move_to_front(&node);
            return;
        }

        if self.map.len() >= self.size
            && let Some(last) = self.doubly_ll.remove_last()
        {
            let last_key = last.borrow().key.clone();
            self.map.remove(&last_key);
        }

        let node = self.doubly_ll.add_to_front(key.clone(), value);
        self.map.insert(key, node);
    }

    /// Prints the current cache entries to standard output.
    ///
    /// Entries are printed in usage order, with the most recently
    /// used entry appearing first.
    ///
    /// # Note
    /// This method requires both keys and values to implement `Debug`.
    pub fn print_cache(&self)
    where
        K: Debug,
        V: Debug,
    {
        println!("{:?}", self.doubly_ll.ll_to_array());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_cache(size: usize) -> LRUCache<String, i32> {
        LRUCache::new(size)
    }

    fn key(value: &str) -> String {
        value.to_string()
    }

    #[test]
    fn test_new_empty_cache() {
        let mut cache = create_cache(3);

        assert_eq!(cache.map.len(), 0);
        assert_eq!(cache.get(&key("A")), None);
    }

    #[test]
    fn test_put_and_get() {
        let mut cache = create_cache(3);

        cache.put(key("A"), 10);

        assert_eq!(cache.get(&key("A")), Some(10));
    }

    #[test]
    fn test_get_missing_key() {
        let mut cache = create_cache(3);

        assert_eq!(cache.get(&key("A")), None);
    }

    #[test]
    fn test_put_multiple_entries() {
        let mut cache = create_cache(3);

        cache.put(key("A"), 10);
        cache.put(key("B"), 20);
        cache.put(key("C"), 30);

        assert_eq!(cache.get(&key("A")), Some(10));
        assert_eq!(cache.get(&key("B")), Some(20));
        assert_eq!(cache.get(&key("C")), Some(30));
    }

    #[test]
    fn test_update_existing_key() {
        let mut cache = create_cache(3);

        cache.put(key("A"), 10);
        cache.put(key("A"), 100);

        assert_eq!(cache.get(&key("A")), Some(100));
        assert_eq!(cache.map.len(), 1);
        assert_eq!(cache.doubly_ll.ll_to_array(), vec![(key("A"), 100)]);
    }

    #[test]
    fn test_get_moves_entry_to_front() {
        let mut cache = create_cache(3);

        cache.put(key("A"), 10);
        cache.put(key("B"), 20);
        cache.put(key("C"), 30);

        cache.get(&key("A"));

        assert_eq!(
            cache.doubly_ll.ll_to_array(),
            vec![(key("A"), 10), (key("C"), 30), (key("B"), 20),]
        );
    }

    #[test]
    fn test_update_moves_entry_to_front() {
        let mut cache = create_cache(3);

        cache.put(key("A"), 10);
        cache.put(key("B"), 20);
        cache.put(key("C"), 30);

        cache.put(key("B"), 200);

        assert_eq!(
            cache.doubly_ll.ll_to_array(),
            vec![(key("B"), 200), (key("C"), 30), (key("A"), 10),]
        );
    }

    #[test]
    fn test_evict_least_recently_used() {
        let mut cache = create_cache(3);

        cache.put(key("A"), 10);
        cache.put(key("B"), 20);
        cache.put(key("C"), 30);

        // A is the least recently used
        cache.put(key("D"), 40);

        assert_eq!(cache.get(&key("A")), None);
        assert_eq!(cache.get(&key("B")), Some(20));
        assert_eq!(cache.get(&key("C")), Some(30));
        assert_eq!(cache.get(&key("D")), Some(40));

        assert_eq!(cache.map.len(), 3);
    }

    #[test]
    fn test_get_changes_eviction_order() {
        let mut cache = create_cache(3);

        cache.put(key("A"), 10);
        cache.put(key("B"), 20);
        cache.put(key("C"), 30);

        // A becomes most recently used
        cache.get(&key("A"));

        // B is now least recently used
        cache.put(key("D"), 40);

        assert_eq!(cache.get(&key("A")), Some(10));
        assert_eq!(cache.get(&key("B")), None);
        assert_eq!(cache.get(&key("C")), Some(30));
        assert_eq!(cache.get(&key("D")), Some(40));
    }

    #[test]
    fn test_update_does_not_increase_cache_size() {
        let mut cache = create_cache(2);

        cache.put(key("A"), 10);
        cache.put(key("B"), 20);
        cache.put(key("A"), 100);

        assert_eq!(cache.map.len(), 2);
        assert_eq!(cache.get(&key("A")), Some(100));
        assert_eq!(cache.get(&key("B")), Some(20));
    }

    #[test]
    fn test_cache_with_capacity_one() {
        let mut cache = create_cache(1);

        cache.put(key("A"), 10);
        cache.put(key("B"), 20);

        assert_eq!(cache.get(&key("A")), None);
        assert_eq!(cache.get(&key("B")), Some(20));
        assert_eq!(cache.map.len(), 1);
    }

    #[test]
    fn test_zero_capacity_cache() {
        let mut cache = create_cache(0);

        cache.put(key("A"), 10);

        assert_eq!(cache.get(&key("A")), None);
        assert_eq!(cache.map.len(), 0);
        assert!(cache.doubly_ll.head.is_none());
        assert!(cache.doubly_ll.tail.is_none());
    }

    #[test]
    fn test_repeated_get_does_not_duplicate_entries() {
        let mut cache = create_cache(3);

        cache.put(key("A"), 10);
        cache.put(key("B"), 20);
        cache.put(key("C"), 30);

        cache.get(&key("A"));
        cache.get(&key("A"));
        cache.get(&key("A"));

        assert_eq!(cache.map.len(), 3);
        assert_eq!(
            cache.doubly_ll.ll_to_array(),
            vec![(key("A"), 10), (key("C"), 30), (key("B"), 20),]
        );
    }

    #[test]
    fn test_multiple_evictions() {
        let mut cache = create_cache(2);

        cache.put(key("A"), 10);
        cache.put(key("B"), 20);
        cache.put(key("C"), 30);
        cache.put(key("D"), 40);

        assert_eq!(cache.get(&key("A")), None);
        assert_eq!(cache.get(&key("B")), None);
        assert_eq!(cache.get(&key("C")), Some(30));
        assert_eq!(cache.get(&key("D")), Some(40));

        assert_eq!(cache.map.len(), 2);
    }

    #[test]
    fn test_map_and_linked_list_remain_consistent() {
        let mut cache = create_cache(3);

        cache.put(key("A"), 10);
        cache.put(key("B"), 20);
        cache.put(key("C"), 30);

        cache.get(&key("A"));
        cache.put(key("D"), 40);
        cache.put(key("B"), 200);

        let list = cache.doubly_ll.ll_to_array();

        assert_eq!(cache.map.len(), list.len());

        for (key, value) in &list {
            let node = cache.map.get(key).unwrap();
            let node_ref = node.borrow();

            assert_eq!(&node_ref.key, key);
            assert_eq!(&node_ref.value, value);
        }
    }
}
