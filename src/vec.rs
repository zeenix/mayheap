#![allow(unused_mut)]

//! Defines [`Vec`] and associated types.

use core::{cmp::Ordering, fmt, hash, iter::FromIterator, ops, slice};

/// A contiguous growable array type.
///
/// This provides the same API as `heapless::Vec`.
///
/// When `heapless` feature is enabled, this is just an alias to `heapless::Vec`.
///
/// When `heapless` feature is disabled, this is a simple wrapper around `alloc::vec::Vec`,
/// setting the initial capacity to `N`. All fallible operations are in reality infallible and all
/// unsafe methods are safe.
pub struct Vec<T, const N: usize>(alloc::vec::Vec<T>);

impl<T, const N: usize> Vec<T, N> {
    /// Constructs a new, empty vector with a capacity of `N`.
    ///
    /// Note: Unlike, `heapless::vec::Vec::new`, this method is currently not `const`.
    pub fn new() -> Self {
        Self(alloc::vec::Vec::with_capacity(N))
    }

    /// Constructs a new vector with a capacity of `N` and fills it with the provided slice.
    #[inline]
    pub fn from_slice(other: &[T]) -> Result<Self, ()>
    where
        T: Clone,
    {
        let mut v = Self::new();
        v.clone_from_slice(other);

        Ok(v)
    }

    /// Returns a raw pointer to the vector’s buffer.
    pub fn as_ptr(&self) -> *const T {
        self.0.as_ptr()
    }

    /// Returns a raw pointer to the vector’s buffer, which may be mutated through.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.0.as_mut_ptr()
    }

    /// Extracts a slice containing the entire vector.
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    /// Returns the contents of the vector as an array of length `M` if the length
    /// of the vector is exactly `M`, otherwise returns `Err(self)`.
    pub fn into_array<const M: usize>(self) -> Result<[T; M], Self> {
        self.0.try_into().map_err(Self)
    }

    /// Extracts a mutable slice containing the entire vector.
    ///
    /// Equivalent to `&mut s[..]`.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.0
    }

    /// the current capacity of the vector.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Clears the vector, removing all values.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Extends the vec from an iterator.
    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.0.extend(iter)
    }

    /// Clones and appends all elements in a slice to the `Vec`.
    ///
    /// Iterates over the slice `other`, clones each element, and then appends
    /// it to this `Vec`. The `other` vector is traversed in-order.
    pub fn extend_from_slice(&mut self, other: &[T]) -> Result<(), ()>
    where
        T: Clone,
    {
        self.0.extend_from_slice(other);

        Ok(())
    }

    /// Removes the last element from a vector and returns it, or `None` if it's empty
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    /// Appends an `item` to the back of the collection
    pub fn push(&mut self, item: T) -> Result<(), T> {
        self.0.push(item);

        Ok(())
    }

    /// Removes the last element from a vector and returns it.
    ///
    /// # Safety
    ///
    /// This assumes the vec to have at least one element.
    pub unsafe fn pop_unchecked(&mut self) -> T {
        self.0.pop().unwrap()
    }

    /// Appends an `item` to the back of the collection.
    pub unsafe fn push_unchecked(&mut self, item: T) {
        self.0.push(item)
    }

    /// Shortens the vector, keeping the first `len` elements and dropping the rest.
    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len)
    }

    /// Resizes the Vec in-place so that len is equal to new_len.
    ///
    /// If new_len is greater than len, the Vec is extended by the
    /// difference, with each additional slot filled with value. If
    /// new_len is less than len, the Vec is simply truncated.
    ///
    /// See also [`resize_default`](Self::resize_default).
    pub fn resize(&mut self, new_len: usize, value: T) -> Result<(), ()>
    where
        T: Clone,
    {
        self.0.resize(new_len, value);

        Ok(())
    }

    /// Resizes the `Vec` in-place so that `len` is equal to `new_len`.
    ///
    /// If `new_len` is greater than `len`, the `Vec` is extended by the
    /// difference, with each additional slot filled with `Default::default()`.
    /// If `new_len` is less than `len`, the `Vec` is simply truncated.
    ///
    /// See also [`resize`](Self::resize).
    pub fn resize_default(&mut self, new_len: usize) -> Result<(), ()>
    where
        T: Clone + Default,
    {
        self.resize(new_len, T::default())
    }

    /// Removes an element from the vector and returns it.
    pub fn swap_remove(&mut self, index: usize) -> T {
        self.0.swap_remove(index)
    }

    /// Removes an element from the vector and returns it.
    #[inline]
    pub unsafe fn swap_remove_unchecked(&mut self, index: usize) -> T {
        self.swap_remove(index)
    }

    /// Returns true if the vec is at full capacity.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.0.len() == self.0.capacity()
    }

    /// Returns true if the vec is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if `needle` is a prefix of the Vec.
    #[inline]
    pub fn starts_with(&self, needle: &[T]) -> bool
    where
        T: PartialEq,
    {
        self.0.starts_with(needle)
    }

    /// Returns `true` if `needle` is a suffix of the Vec.
    ///
    /// Always returns `true` if `needle` is an empty slice.
    #[inline]
    pub fn ends_with(&self, needle: &[T]) -> bool
    where
        T: PartialEq,
    {
        self.0.ends_with(needle)
    }

    /// Inserts an element at position `index` within the vector, shifting all
    /// elements after it to the right.
    pub fn insert(&mut self, index: usize, element: T) -> Result<(), T> {
        self.0.insert(index, element);

        Ok(())
    }

    /// Removes and returns the element at position `index` within the vector,
    /// shifting all elements after it to the left.
    pub fn remove(&mut self, index: usize) -> T {
        self.0.remove(index)
    }

    /// Retains only the elements specified by the predicate.
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.0.retain(f)
    }

    /// Retains only the elements specified by the predicate, passing a mutable reference to it.
    pub fn retain_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        self.0.retain_mut(f)
    }
}

// Trait implementations

impl<T, const N: usize> Default for Vec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> fmt::Debug for Vec<T, N>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<const N: usize> fmt::Write for Vec<u8, N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.extend_from_slice(s.as_bytes());

        Ok(())
    }
}

impl<'a, T: Clone, const N: usize> TryFrom<&'a [T]> for Vec<T, N> {
    type Error = ();

    fn try_from(slice: &'a [T]) -> Result<Self, Self::Error> {
        Vec::from_slice(slice)
    }
}

impl<T, const N: usize> Extend<T> for Vec<T, N> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.extend(iter)
    }
}

impl<'a, T, const N: usize> Extend<&'a T> for Vec<T, N>
where
    T: 'a + Copy,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        self.extend(iter.into_iter().cloned())
    }
}

impl<T, const N: usize> hash::Hash for Vec<T, N>
where
    T: core::hash::Hash,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        hash::Hash::hash(&self.0, state);
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Vec<T, N> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut Vec<T, N> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, const N: usize> FromIterator<T> for Vec<T, N> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(<alloc::vec::Vec<T> as FromIterator<T>>::from_iter(iter))
    }
}

/// An iterator that moves out of an [`Vec`][`Vec`].
///
/// This struct is created by calling the `into_iter` method on [`Vec`][`Vec`].
#[derive(Debug)]
pub struct IntoIter<T, const N: usize>(alloc::vec::IntoIter<T>);

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<T, const N: usize> Clone for IntoIter<T, N>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T, const N: usize> IntoIterator for Vec<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}

impl<A, B, const N1: usize, const N2: usize> PartialEq<Vec<B, N2>> for Vec<A, N1>
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &Vec<B, N2>) -> bool {
        self.0.eq(&other.0)
    }
}

impl<A, B, const N: usize> PartialEq<[B]> for Vec<A, N>
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &[B]) -> bool {
        self.0.eq(other)
    }
}

impl<A, B, const N: usize> PartialEq<Vec<A, N>> for [B]
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &Vec<A, N>) -> bool {
        other.0.eq(self)
    }
}

impl<A, B, const N: usize> PartialEq<&[B]> for Vec<A, N>
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &&[B]) -> bool {
        self.0.eq(*other)
    }
}

impl<A, B, const N: usize> PartialEq<Vec<A, N>> for &[B]
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &Vec<A, N>) -> bool {
        other.0.eq(self)
    }
}

impl<A, B, const N: usize> PartialEq<&mut [B]> for Vec<A, N>
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &&mut [B]) -> bool {
        self.0.eq(*other)
    }
}

impl<A, B, const N: usize> PartialEq<Vec<A, N>> for &mut [B]
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &Vec<A, N>) -> bool {
        other.0.eq(self)
    }
}

impl<A, B, const N: usize, const M: usize> PartialEq<[B; M]> for Vec<A, N>
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &[B; M]) -> bool {
        self.0.eq(other)
    }
}

impl<A, B, const N: usize, const M: usize> PartialEq<Vec<A, N>> for [B; M]
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &Vec<A, N>) -> bool {
        other.0.eq(self)
    }
}

impl<A, B, const N: usize, const M: usize> PartialEq<&[B; M]> for Vec<A, N>
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &&[B; M]) -> bool {
        self.0.eq(*other)
    }
}

impl<A, B, const N: usize, const M: usize> PartialEq<Vec<A, N>> for &[B; M]
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &Vec<A, N>) -> bool {
        other.0.eq(self)
    }
}

// Implements Eq if underlying data is Eq
impl<T, const N: usize> Eq for Vec<T, N> where T: Eq {}

impl<T, const N1: usize, const N2: usize> PartialOrd<Vec<T, N2>> for Vec<T, N1>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Vec<T, N2>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T, const N: usize> Ord for Vec<T, N>
where
    T: Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T, const N: usize> ops::Deref for Vec<T, N> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const N: usize> ops::DerefMut for Vec<T, N> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, const N: usize> AsRef<Vec<T, N>> for Vec<T, N> {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T, const N: usize> AsMut<Vec<T, N>> for Vec<T, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<T, const N: usize> AsRef<[T]> for Vec<T, N> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self
    }
}

impl<T, const N: usize> AsMut<[T]> for Vec<T, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T, const N: usize> Clone for Vec<T, N>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
