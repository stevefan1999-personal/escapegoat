use core::borrow::Borrow;
use core::fmt;
use core::iter::{FusedIterator, Peekable};
use core::ops::RangeBounds;

use arrayvec::ArrayVec;

use crate::map::SgMap;
use crate::tree::{
    Idx, IntoIter as TreeIntoIter, Iter as TreeIter, IterMut as TreeIterMut, SmallNode,
};

// General Iterators ---------------------------------------------------------------------------------------------------

/// An iterator over the entries of a [`SgMap`][crate::map::SgMap].
///
/// This `struct` is created by the [`iter`][crate::map::SgMap::iter] method on [`SgMap`][crate::map::SgMap].
/// documentation for more.
///
pub struct Iter<'a, T: Ord, V, const N: usize> {
    ref_iter: TreeIter<'a, T, V, N>,
}

impl<'a, K: Ord, V, const N: usize> Iter<'a, K, V, N> {
    /// Construct reference iterator.
    pub(crate) fn new(map: &'a SgMap<K, V, N>) -> Self {
        Iter {
            ref_iter: TreeIter::new(&map.bst),
        }
    }
}

impl<'a, K: Ord, V, const N: usize> Iterator for Iter<'a, K, V, N> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.ref_iter.next()
    }
}

impl<'a, K: Ord, V, const N: usize> ExactSizeIterator for Iter<'a, K, V, N> {
    fn len(&self) -> usize {
        self.ref_iter.len()
    }
}

impl<'a, K: Ord, V, const N: usize> FusedIterator for Iter<'a, K, V, N> {}

/// An owning iterator over the entries of a [`SgMap`][crate::map::SgMap].
///
/// This `struct` is created by the [`into_iter`][crate::map::SgMap::into_iter] method on [`SgMap`][crate::map::SgMap].
/// documentation for more.
pub struct IntoIter<K: Ord, V, const N: usize> {
    cons_iter: TreeIntoIter<K, V, N>,
}

impl<K: Ord, V, const N: usize> IntoIter<K, V, N> {
    /// Construct owning iterator.
    pub(crate) fn new(map: SgMap<K, V, N>) -> Self {
        IntoIter {
            cons_iter: TreeIntoIter::new(map.bst),
        }
    }
}

impl<K: Ord, V, const N: usize> Iterator for IntoIter<K, V, N> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.cons_iter.next()
    }
}

impl<K: Ord, V, const N: usize> ExactSizeIterator for IntoIter<K, V, N> {
    fn len(&self) -> usize {
        self.cons_iter.len()
    }
}

impl<K: Ord, V, const N: usize> FusedIterator for IntoIter<K, V, N> {}

/// An mutable iterator over the entries of a [`SgMap`][crate::map::SgMap].
///
/// This `struct` is created by the [`iter_mut`][crate::map::SgMap::iter_mut] method on [`SgMap`][crate::map::SgMap].
/// documentation for more.
pub struct IterMut<'a, K: Ord, V, const N: usize> {
    mut_iter: TreeIterMut<'a, K, V, N>,
}

impl<'a, K: Ord, V, const N: usize> IterMut<'a, K, V, N> {
    /// Construct owning iterator.
    pub(crate) fn new(map: &'a mut SgMap<K, V, N>) -> Self {
        IterMut {
            mut_iter: TreeIterMut::new(&mut map.bst),
        }
    }
}

impl<'a, K: Ord, V, const N: usize> Iterator for IterMut<'a, K, V, N> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.mut_iter.next()
    }
}

impl<'a, K: Ord, V, const N: usize> ExactSizeIterator for IterMut<'a, K, V, N> {
    fn len(&self) -> usize {
        self.mut_iter.len()
    }
}

impl<'a, K: Ord, V, const N: usize> FusedIterator for IterMut<'a, K, V, N> {}

// Key Iterators -------------------------------------------------------------------------------------------------------

// TODO: these need more trait implementations for full compatibility

/// An iterator over the keys of a [`SgMap`][crate::map::SgMap].
///
/// This `struct` is created by the [`keys`][crate::map::SgMap::keys] method on [`SgMap`][crate::map::SgMap].
/// See its documentation for more.
pub struct Keys<'a, K: Ord, V, const N: usize> {
    pub(crate) inner: Iter<'a, K, V, N>,
}

impl<'a, K: Ord, V, const N: usize> Iterator for Keys<'a, K, V, N> {
    type Item = &'a K;

    fn next(&mut self) -> Option<&'a K> {
        self.inner.next().map(|(k, _)| k)
    }
}

impl<'a, K: Ord, V, const N: usize> ExactSizeIterator for Keys<'a, K, V, N> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K: Ord, V, const N: usize> FusedIterator for Keys<'a, K, V, N> {}

/// An owning iterator over the keys of a [`SgMap`][crate::map::SgMap].
///
/// This `struct` is created by the [`into_keys`][crate::map::SgMap::into_keys] method on [`SgMap`][crate::map::SgMap].
/// See its documentation for more.
pub struct IntoKeys<K: Ord, V, const N: usize> {
    pub(crate) inner: IntoIter<K, V, N>,
}

impl<K: Ord, V, const N: usize> Iterator for IntoKeys<K, V, N> {
    type Item = K;

    fn next(&mut self) -> Option<K> {
        self.inner.next().map(|(k, _)| k)
    }
}

impl<K: Ord, V, const N: usize> ExactSizeIterator for IntoKeys<K, V, N> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K: Ord, V, const N: usize> FusedIterator for IntoKeys<K, V, N> {}

// Value Iterators -----------------------------------------------------------------------------------------------------

// TODO: these need more trait implementations for full compatibility

/// An iterator over the values of a [`SgMap`][crate::map::SgMap].
///
/// This `struct` is created by the [`values`][crate::map::SgMap::values] method on [`SgMap`][crate::map::SgMap].
/// See its documentation for more.
pub struct Values<'a, K: Ord, V, const N: usize> {
    pub(crate) inner: Iter<'a, K, V, N>,
}

impl<'a, K: Ord, V, const N: usize> Iterator for Values<'a, K, V, N> {
    type Item = &'a V;

    fn next(&mut self) -> Option<&'a V> {
        self.inner.next().map(|(_, v)| v)
    }
}

impl<'a, K: Ord, V, const N: usize> ExactSizeIterator for Values<'a, K, V, N> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K: Ord, V, const N: usize> FusedIterator for Values<'a, K, V, N> {}

/// An owning iterator over the values of a [`SgMap`][crate::map::SgMap].
///
/// This `struct` is created by the [`into_values`][crate::map::SgMap::into_values] method on [`SgMap`][crate::map::SgMap].
/// See its documentation for more.
pub struct IntoValues<K: Ord, V, const N: usize> {
    pub(crate) inner: IntoIter<K, V, N>,
}

impl<K: Ord, V, const N: usize> Iterator for IntoValues<K, V, N> {
    type Item = V;

    fn next(&mut self) -> Option<V> {
        self.inner.next().map(|(_, v)| v)
    }
}

impl<K: Ord, V, const N: usize> ExactSizeIterator for IntoValues<K, V, N> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K: Ord, V, const N: usize> FusedIterator for IntoValues<K, V, N> {}

/// A mutable iterator over the values of a [`SgMap`][crate::map::SgMap].
///
/// This `struct` is created by the [`values_mut`][crate::map::SgMap::values_mut] method on [`SgMap`][crate::map::SgMap].
/// See its documentation for more.
pub struct ValuesMut<'a, K: Ord, V, const N: usize> {
    pub(crate) inner: IterMut<'a, K, V, N>,
}

impl<'a, K: Ord, V, const N: usize> Iterator for ValuesMut<'a, K, V, N> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<&'a mut V> {
        self.inner.next().map(|(_, v)| v)
    }
}

impl<'a, K: Ord, V, const N: usize> ExactSizeIterator for ValuesMut<'a, K, V, N> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K: Ord, V, const N: usize> FusedIterator for ValuesMut<'a, K, V, N> {}

// Entry APIs ----------------------------------------------------------------------------------------------------------

/// A view into a single entry in a map, which may either be vacant or occupied.
///
/// This `enum` is constructed from the [`SgMap::entry`] method on [`SgMap`].
pub enum Entry<'a, K: Ord, V, const N: usize> {
    /// A vacant entry.
    Vacant(VacantEntry<'a, K, V, N>),
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, K, V, N>),
}

impl<'a, K: Ord, V, const N: usize> Entry<'a, K, V, N> {
    /// Ensures a value is in the entry by inserting the default if empty, and returns a mutable
    /// reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use escapegoat::SgMap;
    ///
    /// let mut map = SgMap::<&str, usize, 10>::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// assert_eq!(map["poneyland"], 12);
    /// ```
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default function if empty, and returns a mutable
    /// reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use escapegoat::SgMap;
    ///
    /// let mut map = SgMap::<&str, usize, 10>::new();
    /// let x = 42;
    /// map.entry("poneyland").or_insert_with(|| x);
    ///
    /// assert_eq!(map["poneyland"], 42);
    /// ```
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }

    /// Ensures a value is in the entry by inserting, if empty, the result of the default function.
    /// This method allows for generating key-derived values for insertion by providing the default
    /// function a reference to the key that was moved during the `.entry(key)` method call.
    ///
    /// The reference to the moved key is provided so that cloning or copying the key is
    /// unnecessary, unlike with `.or_insert_with(|| ... )`.
    ///
    /// # Examples
    ///
    /// ```
    /// use escapegoat::SgMap;
    ///
    /// let mut map = SgMap::<&str, usize, 10>::new();
    ///
    /// map.entry("poneyland").or_insert_with_key(|key| key.chars().count());
    ///
    /// assert_eq!(map["poneyland"], 9);
    /// ```
    pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let value = default(entry.key());
                entry.insert(value)
            }
        }
    }

    /// Returns a reference to this entry's key.
    ///
    /// # Examples
    ///
    /// ```
    /// use escapegoat::SgMap;
    ///
    /// let mut map = SgMap::<&str, usize, 10>::new();
    /// assert_eq!(map.entry("poneyland").key(), &"poneyland");
    /// ```
    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(entry) => entry.key(),
            Entry::Vacant(entry) => entry.key(),
        }
    }

    /// Provides in-place mutable access to an occupied entry before any
    /// potential inserts into the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use escapegoat::SgMap;
    ///
    /// let mut map = SgMap::<&str, usize, 10>::new();
    ///
    /// map.entry("poneyland")
    ///    .and_modify(|e| { *e += 1 })
    ///    .or_insert(42);
    /// assert_eq!(map["poneyland"], 42);
    ///
    /// map.entry("poneyland")
    ///    .and_modify(|e| { *e += 1 })
    ///    .or_insert(42);
    /// assert_eq!(map["poneyland"], 43);
    /// ```
    pub fn and_modify<F: FnOnce(&mut V)>(self, f: F) -> Entry<'a, K, V, N> {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }
}

impl<'a, K: Ord, V: Default, const N: usize> Entry<'a, K, V, N> {
    /// Ensures a value is in the entry by inserting the default value if empty,
    /// and returns a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use escapegoat::SgMap;
    ///
    /// let mut map = SgMap::<&str, Option<usize>, 10>::new();
    /// map.entry("poneyland").or_default();
    ///
    /// assert_eq!(map["poneyland"], None);
    /// ```
    pub fn or_default(self) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(Default::default()),
        }
    }
}

/// A view into a vacant entry in a [`SgMap`][crate::map::SgMap].
/// It is part of the [`Entry`] enum.
pub struct VacantEntry<'a, K: Ord, V, const N: usize> {
    pub(super) key: K,
    pub(super) table: &'a mut SgMap<K, V, N>,
}

impl<'a, K: Ord, V, const N: usize> VacantEntry<'a, K, V, N> {
    /// Gets a reference to the key that would be used when inserting a value
    /// through the [`VacantEntry`][crate::map_types::VacantEntry].
    ///
    /// # Examples
    ///
    /// ```
    /// use escapegoat::SgMap;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    /// assert_eq!(map.entry("poneyland").key(), &"poneyland");
    /// ```
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Take ownership of the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use escapegoat::SgMap;
    /// use escapegoat::map_types::Entry;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    ///
    /// if let Entry::Vacant(v) = map.entry("poneyland") {
    ///     v.into_key();
    /// }
    /// ```
    pub fn into_key(self) -> K {
        self.key
    }

    /// Sets the value of the entry with the [`VacantEntry`][crate::map_types::VacantEntry]'s key,
    /// and returns a mutable reference to it.
    ///
    /// # Examples
    ///
    /// ```
    /// use escapegoat::SgMap;
    /// use escapegoat::map_types::Entry;
    ///
    /// let mut map = SgMap::<&str, u32, 2>::new();
    ///
    /// if let Entry::Vacant(o) = map.entry("poneyland") {
    ///     o.insert(37);
    /// }
    /// assert_eq!(map["poneyland"], 37);
    /// ```
    pub fn insert(self, value: V) -> &'a mut V {
        let (_, new_node_idx) = self
            .table
            .bst
            .internal_balancing_insert::<Idx>(self.key, value);

        self.table.bst.arena[new_node_idx].get_mut().1
    }
}

/// A view into an occupied entry in a [`SgMap`][crate::map::SgMap].
/// It is part of the [`Entry`] enum.
pub struct OccupiedEntry<'a, K: Ord, V, const N: usize> {
    pub(super) node_idx: usize,
    pub(super) table: &'a mut SgMap<K, V, N>,
}

impl<'a, K: Ord, V, const N: usize> OccupiedEntry<'a, K, V, N> {
    /// Gets a reference to the key in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use escapegoat::SgMap;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    /// map.entry("poneyland").or_insert(12);
    /// assert_eq!(map.entry("poneyland").key(), &"poneyland");
    /// ```
    pub fn key(&self) -> &K {
        self.table.bst.arena[self.node_idx].key()
    }

    /// Gets a reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use escapegoat::SgMap;
    /// use escapegoat::map_types::Entry;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     assert_eq!(o.get(), &12);
    /// }
    /// ```
    pub fn get(&self) -> &V {
        self.table.bst.arena[self.node_idx].val()
    }

    /// Gets a mutable reference to the value in the entry.
    ///
    /// If you need a reference to the `OccupiedEntry` that may outlive the
    /// destruction of the `Entry` value, see [`into_mut`].
    ///
    /// [`into_mut`]: OccupiedEntry::into_mut
    ///
    /// # Examples
    ///
    /// ```
    /// use escapegoat::SgMap;
    /// use escapegoat::map_types::Entry;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// assert_eq!(map["poneyland"], 12);
    /// if let Entry::Occupied(mut o) = map.entry("poneyland") {
    ///     *o.get_mut() += 10;
    ///     assert_eq!(*o.get(), 22);
    ///
    ///     // We can use the same Entry multiple times.
    ///     *o.get_mut() += 2;
    /// }
    /// assert_eq!(map["poneyland"], 24);
    /// ```
    pub fn get_mut(&mut self) -> &mut V {
        self.table.bst.arena[self.node_idx].get_mut().1
    }

    /// Converts the entry into a mutable reference to its value.
    ///
    /// If you need multiple references to the `OccupiedEntry`, see [`get_mut`].
    ///
    /// [`get_mut`]: OccupiedEntry::get_mut
    ///
    /// # Examples
    ///
    /// ```
    /// use escapegoat::SgMap;
    /// use escapegoat::map_types::Entry;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// assert_eq!(map["poneyland"], 12);
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     *o.into_mut() += 10;
    /// }
    /// assert_eq!(map["poneyland"], 22);
    /// ```
    pub fn into_mut(self) -> &'a mut V {
        self.table.bst.arena[self.node_idx].get_mut().1
    }

    /// Sets the value of the entry with the `OccupiedEntry`'s key,
    /// and returns the entry's old value.
    ///
    /// # Examples
    ///
    /// ```
    /// use escapegoat::SgMap;
    /// use escapegoat::map_types::Entry;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(mut o) = map.entry("poneyland") {
    ///     assert_eq!(o.insert(15), 12);
    /// }
    /// assert_eq!(map["poneyland"], 15);
    /// ```
    pub fn insert(&mut self, value: V) -> V {
        core::mem::replace(self.get_mut(), value)
    }

    /// Take ownership of the key and value from the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use escapegoat::SgMap;
    /// use escapegoat::map_types::Entry;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     // We delete the entry from the map.
    ///     o.remove_entry();
    /// }
    ///
    /// // If now try to get the value, it will panic:
    /// // println!("{}", map["poneyland"]);
    /// ```
    pub fn remove_entry(self) -> (K, V) {
        self.table
            .bst
            .priv_remove_by_idx(self.node_idx)
            .expect("Must be occupied")
    }

    /// Takes the value of the entry out of the map, and returns it.
    ///
    /// # Examples
    ///
    /// ```
    /// use escapegoat::SgMap;
    /// use escapegoat::map_types::Entry;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     assert_eq!(o.remove(), 12);
    /// }
    /// // If we try to get "poneyland"'s value, it'll panic:
    /// // println!("{}", map["poneyland"]);
    /// ```
    pub fn remove(self) -> V {
        self.remove_entry().1
    }
}

/// The error returned by [`try_insert_std`](SgMap::try_insert_std) when the key already exists.
///
/// Contains the occupied entry, and the value that was not inserted.
pub struct OccupiedError<'a, K: 'a + Ord, V: 'a, const N: usize> {
    /// The entry in the map that was already occupied.
    pub entry: OccupiedEntry<'a, K, V, N>,
    /// The value which was not inserted, because the entry was already occupied.
    pub value: V,
}

impl<K: fmt::Debug + Ord, V: fmt::Debug, const N: usize> fmt::Debug for OccupiedError<'_, K, V, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OccupiedError")
            .field("key", self.entry.key())
            .field("old_value", self.entry.get())
            .field("new_value", &self.value)
            .finish()
    }
}

impl<'a, K: fmt::Debug + Ord, V: fmt::Debug, const N: usize> fmt::Display
    for OccupiedError<'a, K, V, N>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to insert {:?}, key {:?} already exists with value {:?}",
            self.value,
            self.entry.key(),
            self.entry.get(),
        )
    }
}

// Range APIs ----------------------------------------------------------------------------------------------------------

/// An iterator over a sub-range of entries in a [`SgMap`].
///
/// This `struct` is created by the [`range`][`crate::map::SgMap::range`] method on [`SgMap`][crate::map::SgMap]. See its
/// documentation for more.
pub struct Range<'a, K: Ord, V, const N: usize> {
    pub(crate) table: &'a SgMap<K, V, N>,
    pub(crate) node_idx_iter: <ArrayVec<usize, N> as IntoIterator>::IntoIter,
}

impl<'a, K: Ord, V, const N: usize> Range<'a, K, V, N> {
    fn to_node_ref(&self, idx: usize) -> (&'a K, &'a V) {
        let node = &self.table.bst.arena[idx];
        (node.key(), node.val())
    }
}

impl<'a, K: Ord, V, const N: usize> Iterator for Range<'a, K, V, N> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let node_idx = self.node_idx_iter.next()?;
        Some(self.to_node_ref(node_idx))
    }
}

impl<'a, K: Ord, V, const N: usize> DoubleEndedIterator for Range<'a, K, V, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let node_idx = self.node_idx_iter.next_back()?;
        Some(self.to_node_ref(node_idx))
    }
}

impl<'a, K: Ord, V, const N: usize> FusedIterator for Range<'a, K, V, N> {}

/// A mutable iterator over a sub-range of entries in a [`SgMap`].
///
/// This `struct` is created by the [`range_mut`] method on [`SgMap`]. See its
/// documentation for more.
///
/// [`range_mut`]: SgMap::range_mut
pub struct RangeMut<'a, K: Ord, V, const N: usize> {
    inner: RangeMutPeekable<'a, K, V, N>,
    last: Option<RangeMutLast<'a, K, V, N>>,
    total_cnt: usize,
    spent_cnt: usize,
}

type RangeMutLast<'a, K, V, const N: usize> =
    <Peekable<TreeIterMut<'a, K, V, N>> as Iterator>::Item;

type RangeMutPeekable<'a, K, V, const N: usize> = Peekable<TreeIterMut<'a, K, V, N>>;

impl<'a, K, V, const N: usize> RangeMut<'a, K, V, N>
where
    K: Ord,
{
    // Constructor
    pub(crate) fn new<T, R>(map: &'a mut SgMap<K, V, N>, range: &R) -> Self
    where
        T: Ord + ?Sized,
        K: Borrow<T> + Ord,
        R: RangeBounds<T>,
    {
        let len = RangeMut::compute_len(map, range);

        let (iter, last) = RangeMut::init_iter_mut(map, range);
        Self {
            inner: iter,
            last,
            total_cnt: len,
            spent_cnt: 0,
        }
    }

    // Compute amount of items to return
    fn compute_len<T, R>(map: &SgMap<K, V, N>, range: &R) -> usize
    where
        T: Ord + ?Sized,
        K: Borrow<T> + Ord,
        R: RangeBounds<T>,
    {
        let mut peekable = map.bst.iter().peekable();
        let mut len = 0;

        // Advance immutable iter to start
        while let Some(node) = peekable.peek() {
            if range.contains(node.0.borrow()) {
                break;
            }

            peekable.next();
        }

        // Count remaining
        for node in peekable {
            if range.contains(node.0.borrow()) {
                len += 1;
            } else {
                break;
            }
        }

        len
    }

    // Prepare mutable iterator to return first item in range
    fn init_iter_mut<T, R>(
        map: &'a mut SgMap<K, V, N>,
        range: &R,
    ) -> (
        RangeMutPeekable<'a, K, V, N>,
        Option<RangeMutLast<'a, K, V, N>>,
    )
    where
        T: Ord + ?Sized,
        K: Borrow<T> + Ord,
        R: RangeBounds<T>,
    {
        let mut peekable = map.bst.iter_mut().peekable();
        let mut last = None;

        // Advance mutable iter to start
        while let Some(node) = peekable.peek() {
            if range.contains(node.0.borrow()) {
                break;
            }

            peekable.next();
        }

        while let Some(node) = peekable.next_back() {
            if range.contains(node.0.borrow()) {
                last = Some(node);
                break;
            }
        }

        (peekable, last)
    }
}

impl<'a, K, V, const N: usize> Iterator for RangeMut<'a, K, V, N>
where
    K: Ord,
{
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.spent_cnt < self.total_cnt {
            self.spent_cnt += 1;
            match self.inner.next() {
                Some(node) => Some(node),
                None => self.last.take(),
            }
        } else {
            None
        }
    }
}

impl<'a, K, V, const N: usize> DoubleEndedIterator for RangeMut<'a, K, V, N>
where
    K: Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.spent_cnt < self.total_cnt {
            self.spent_cnt += 1;
            match self.last.take() {
                Some(node) => Some(node),
                None => self.inner.next_back(),
            }
        } else {
            None
        }
    }
}

impl<'a, K: Ord, V, const N: usize> FusedIterator for RangeMut<'a, K, V, N> {}

/*
// TODO: does commit to this interface limit potential optimizations?
impl<'a, K, V, const N: usize> ExactSizeIterator for RangeMut<'a, K, V, N>
where
    K: Ord ,
    V,
{
    fn len(&self) -> usize {
        debug_assert!(self.spent_cnt <= self.total_cnt);
        self.total_cnt - self.spent_cnt
    }
}
*/
