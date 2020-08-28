//! Mapping type based on a vec providing fast iteration
use std::mem;

/// Mapping type relying the ability to sort the keys.
///
/// # Notes
///
/// We do not use the Index, IndexMut traits since using the IndexMut would
/// require us to be able to create a new V, which we may not know how to do.
///
/// We also try to stay as close as possible to the API of std::collections::HashMap
pub struct AssocVec<K, V>
where
    K: Ord,
{
    vec: Vec<(K, V)>,
}

// The use if #[inline] follows the source for std::collections::HashMap
// Some functions get/get_mut/set could be made more generic (cf HashMap)
impl<K, V> AssocVec<K, V>
where
    K: Ord,
{
    /// Create a new empty AssocVector
    #[inline]
    pub fn new() -> AssocVec<K, V> {
        AssocVec { vec: vec![] }
    }

    /// Create a new AssocVector with a specific capacity
    #[inline]
    pub fn with_capacity(capacity: usize) -> AssocVec<K, V> {
        AssocVec {
            vec: Vec::with_capacity(capacity),
        }
    }

    /// Check if an AssocVector is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// Returns the number of element in the AssocVector
    #[inline]
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Returns the capacity of the AssocVector
    #[inline]
    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    /// Clear the content of the AssocVector
    #[inline]
    pub fn clear(&mut self) {
        self.vec.clear()
    }

    /// Check if a key is present in the AssocVector
    pub fn contains_key(&self, key: &K) -> bool {
        self.search(key).is_ok()
    }

    /// Get an immutable reference to the value corresponding to a key
    #[inline]
    pub fn get(&self, key: &K) -> Option<&V> {
        match self.search(key) {
            Ok(i) => Some(&self.vec[i].1),
            Err(_) => None,
        }
    }

    /// Get a mutable reference to the value corresponding to a key
    #[inline]
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.search(key) {
            Ok(i) => Some(&mut self.vec[i].1),
            Err(_) => None,
        }
    }

    /// Set and return the old value associated with a key if any
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.search(&key) {
            Ok(i) => {
                let (_k, v) = mem::replace(&mut self.vec[i], (key, value));
                Some(v)
            }
            Err(j) => {
                self.vec.insert(j, (key, value));
                None
            }
        }
    }

    /// Remove an entry from the AssocVector and return the value
    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.search(key) {
            Ok(i) => Some(self.vec.remove(i).1),
            Err(_) => None,
        }
    }

    /// Iter over the key, value pairs
    pub fn iter(&self) -> std::slice::Iter<(K, V)> {
        self.vec.iter()
    }

    /// Search for a key
    ///
    /// If found returns its index ortherwise the index at which to insert it
    /// to preserve the order
    #[inline]
    fn search(&self, key: &K) -> Result<usize, usize> {
        self.vec.binary_search_by(|(k, _v)| k.cmp(key))
    }
}

#[cfg(test)]
mod test {

    use super::AssocVec;

    #[test]
    fn test_insert() {
        let mut av = AssocVec::new();
        assert!(av.is_empty());
        assert_eq!(av.insert(1, 2), None);
        assert!(!av.is_empty());
        assert!(av.contains_key(&1));
        assert_eq!(av.insert(1, 3), Some(2));
    }

    #[test]
    fn test_remove() {
        let mut av = AssocVec::new();
        assert_eq!(av.insert(1, 2), None);
        assert_eq!(av.len(), 1);
        assert_eq!(av.remove(&1), Some(2));
        assert_eq!(av.len(), 0);
    }

    #[test]
    fn test_get() {
        let mut av = AssocVec::with_capacity(1);
        av.insert(1, 2);
        {
            let a = av.get(&1).unwrap();
            assert_eq!(*a, 2);
        }
        {
            let b = av.get_mut(&1).unwrap();
            *b += 1;
        }
        assert_eq!(*av.get(&1).unwrap(), 3);
    }
}
