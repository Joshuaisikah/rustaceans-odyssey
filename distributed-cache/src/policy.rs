// ─── EvictionPolicy trait ────────────────────────────────────────────────────
//
// Abstraction over eviction strategies.  LruCache uses this internally.
// Implementing a new strategy (LFU, ARC, FIFO…) only requires implementing
// this trait — the cache itself stays unchanged.

pub trait EvictionPolicy<K> {
    /// Record that `key` was accessed (hit or insert).
    fn touch(&mut self, key: &K);

    /// Remove and return the key that should be evicted next.
    /// Returns None only if the tracker is empty.
    fn evict(&mut self) -> Option<K>;

    /// Remove tracking for `key` (called on explicit remove/TTL expiry).
    fn remove(&mut self, key: &K);

    /// Drop all tracked state.
    fn clear(&mut self);

    fn len(&self)      -> usize;
    fn is_empty(&self) -> bool { self.len() == 0 }
}

// ─── LruPolicy ───────────────────────────────────────────────────────────────
//
// A Least-Recently-Used eviction policy backed by an ordered structure.
// Hint: VecDeque<K> where the front is the LRU candidate and the back is MRU.
//       On touch(): remove the key if present, push to the back.
//       On evict(): pop from the front.

pub struct LruPolicy<K> {
    // TODO: order: VecDeque<K>
    _p: std::marker::PhantomData<K>,
}

impl<K: Eq + Clone> LruPolicy<K> {
    pub fn new() -> Self {
        todo!()
    }
}

impl<K: Eq + Clone> EvictionPolicy<K> for LruPolicy<K> {
    fn touch(&mut self, key: &K)   { todo!() }
    fn evict(&mut self) -> Option<K> { todo!() }
    fn remove(&mut self, key: &K)  { todo!() }
    fn clear(&mut self)            { todo!() }
    fn len(&self) -> usize         { todo!() }
}

impl<K: Eq + Clone> Default for LruPolicy<K> {
    fn default() -> Self { Self::new() }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // A new policy is empty.
    #[test]
    fn test_new_policy_is_empty() {
        let p = LruPolicy::<&str>::new();
        assert!(p.is_empty());
        assert_eq!(p.len(), 0);
    }

    // touch() tracks a key.
    #[test]
    fn test_touch_increases_len() {
        let mut p = LruPolicy::<&str>::new();
        p.touch(&"a");
        assert_eq!(p.len(), 1);
    }

    // evict() returns the LRU key (first inserted if no accesses).
    #[test]
    fn test_evict_returns_lru() {
        let mut p = LruPolicy::<&str>::new();
        p.touch(&"a");
        p.touch(&"b");
        p.touch(&"c");
        assert_eq!(p.evict(), Some("a")); // "a" is LRU
    }

    // touch() on an existing key moves it to MRU position.
    #[test]
    fn test_touch_moves_to_mru() {
        let mut p = LruPolicy::<&str>::new();
        p.touch(&"a");
        p.touch(&"b");
        p.touch(&"a"); // refresh "a" — "b" becomes LRU
        assert_eq!(p.evict(), Some("b"));
    }

    // remove() stops tracking a key.
    #[test]
    fn test_remove_decreases_len() {
        let mut p = LruPolicy::<&str>::new();
        p.touch(&"x");
        p.remove(&"x");
        assert!(p.is_empty());
    }

    // clear() empties the policy.
    #[test]
    fn test_clear_empties_policy() {
        let mut p = LruPolicy::<&str>::new();
        p.touch(&"a"); p.touch(&"b");
        p.clear();
        assert!(p.is_empty());
    }

    // evict() on an empty policy returns None.
    #[test]
    fn test_evict_empty_returns_none() {
        let mut p = LruPolicy::<&str>::new();
        assert_eq!(p.evict(), None);
    }
}
