use super::*;

impl<K, V> LruCache<K, V>
where
    K: Clone + Eq + Hash,
{
    /// Returns the current number of entries in the
    /// cache.
    ///
    /// # Returns
    ///
    /// - `usize` - The number of items in the collection.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if the cache is empty.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns `true` if the cache is at capacity.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_full(&self) -> bool {
        self.map.len() >= self.capacity
    }

    /// Returns `true` if the cache contains a value for
    /// the given key. Does NOT update the recency (use
    /// `get` for that).
    ///
    /// # Arguments
    ///
    /// - `&K` - Shared reference to a `K`.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    /// Returns the value for the given key, updating the
    /// recency so the entry becomes the most-recently-
    /// used.
    ///
    /// Returns `None` if the key is not in the cache.
    ///
    /// # Arguments
    ///
    /// - `&K` - Shared reference to a `K`.
    ///
    /// # Returns
    ///
    /// - `Option<V>` - The current value (or a snapshot thereof).
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // Promote the key to the front of the
            // order deque. Remove its existing position
            // first (if any) to avoid duplicates.
            self.order.retain(|k: &K| k != key);
            self.order.push_front(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }

    /// Returns the value for the given key without
    /// updating the recency. Useful for "is this cached?"
    /// checks that should not affect eviction order.
    ///
    /// # Arguments
    ///
    /// - `&K` - Shared reference to a `K`.
    ///
    /// # Returns
    ///
    /// - `Option<V>` - `Some(...)` on success, `None` otherwise.
    pub fn peek(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    /// Inserts a key-value pair into the cache. If the
    /// key is already present, the existing value is
    /// replaced (and the entry becomes the most-
    /// recently-used). If the cache is at capacity and
    /// the key is new, the least-recently-used entry is
    /// evicted first.
    ///
    /// Returns the evicted entry, if any.
    ///
    /// # Arguments
    ///
    /// - `K: Clone + Eq + Hash` - A generic type parameter.
    /// - `V` - A `V` parameter.
    ///
    /// # Returns
    ///
    /// - `Option<(K, V)>` - `Some(...)` on success, `None` otherwise.
    pub fn put(&mut self, key: K, value: V) -> Option<(K, V)> {
        // Capacity of 0 — silently drop.
        if self.capacity == 0 {
            return None;
        }
        // Updating an existing key.
        if self.map.contains_key(&key) {
            self.map.insert(key.clone(), value);
            // Promote the key to the front of the
            // order deque. Remove its existing position
            // first (if any) to avoid duplicates.
            self.order.retain(|k: &K| k != &key);
            self.order.push_front(key);
            return None;
        }
        // Inserting a new key. Evict if at capacity.
        let evicted: Option<(K, V)> = if self.map.len() >= self.capacity {
            let victim_key: K = self.order.pop_back()?;
            let victim_value: V = self.map.remove(&victim_key)?;
            Some((victim_key, victim_value))
        } else {
            None
        };
        self.map.insert(key.clone(), value);
        self.order.push_front(key);
        evicted
    }

    /// Removes the entry for the given key, returning
    /// the removed value if any.
    ///
    /// # Arguments
    ///
    /// - `&K` - Shared reference to a `K`.
    ///
    /// # Returns
    ///
    /// - `Option<V>` - `Some(...)` on success, `None` otherwise.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.order.retain(|k: &K| k != key);
        self.map.remove(key)
    }

    /// Removes every entry from the cache.
    pub fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
    }

    /// Returns an iterator over the entries in
    /// most-recently-used-first order.
    ///
    /// # Returns
    ///
    /// - `impl Iterator<Item` - A `impl Iterator<Item` value.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        // We can't return the VecDeque order directly
        // because the entries would be in order-deque
        // order, not MRU-first order. Actually they
        // ARE in MRU-first order — the VecDeque's
        // front is MRU. So iterating and mapping through
        // the map gives us MRU-first order.
        self.order
            .iter()
            .filter_map(|k: &K| self.map.get_key_value(k))
    }

    /// Returns an iterator over the keys in
    /// most-recently-used-first order.
    ///
    /// # Returns
    ///
    /// - `impl Iterator<Item` - A `impl Iterator<Item` value.
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.order.iter()
    }

    /// Returns an iterator over the values in
    /// most-recently-used-first order.
    ///
    /// # Returns
    ///
    /// - `impl Iterator<Item` - A `impl Iterator<Item` value.
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.order.iter().filter_map(|k: &K| self.map.get(k))
    }

    /// Resizes the cache to a new capacity.
    ///
    /// If the new capacity is smaller than the current
    /// size, the least-recently-used entries are
    /// evicted until the cache fits.
    ///
    /// # Arguments
    ///
    /// - `usize` - A non-negative integer (`usize`).
    pub fn resize(&mut self, new_capacity: usize) {
        self.set_capacity(new_capacity);
        while self.map.len() > self.capacity {
            if let Some(victim_key) = self.order.pop_back() {
                self.map.remove(&victim_key);
            } else {
                break;
            }
        }
    }
}
