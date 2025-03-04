//! A UTF-8–encoded, growable string.

use core::{
    cmp::Ordering,
    fmt, hash, iter, ops,
    str::{self, Utf8Error},
};

use crate::Vec;

#[cfg(feature = "alloc")]
type Inner<const N: usize> = alloc::string::String;
#[cfg(not(feature = "alloc"))]
type Inner<const N: usize> = heapless::String<N>;

/// A UTF-8–encoded, growable string.
///
/// This provides the same API as `heapless::String`.
///
/// When `heapless` feature is enabled, this is wrapper around `heapless::String`. Otherwise, this
/// is a wrapper around `alloc::string::String`, setting the initial capacity to `N`. All fallible
/// operations are in reality infallible and all unsafe methods are safe in the latter case.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct String<const N: usize>(Inner<N>);

impl<const N: usize> String<N> {
    /// Constructs a new, empty `String` with a capacity of `N` bytes.
    ///
    /// Note: Unlike, `heapless::string::String::new`, this method is currently not `const`.
    #[inline]
    pub fn new() -> Self {
        #[cfg(feature = "alloc")]
        {
            Self(Inner::with_capacity(N))
        }
        #[cfg(not(feature = "alloc"))]
        {
            Self(Inner::new())
        }
    }

    /// Convert UTF-8 bytes into a `String`.
    #[inline]
    pub fn from_utf8(vec: Vec<u8, N>) -> Result<Self, Utf8Error> {
        let res = Inner::from_utf8(vec.into_inner()).map(Self);
        #[cfg(feature = "alloc")]
        {
            res.map_err(|e| e.utf8_error())
        }
        #[cfg(not(feature = "alloc"))]
        {
            res
        }
    }

    /// Convert UTF-8 bytes into a `String`, without checking that the string
    /// contains valid UTF-8.
    ///
    /// # Safety
    ///
    /// The bytes passed in must be valid UTF-8.
    #[inline]
    pub unsafe fn from_utf8_unchecked(vec: Vec<u8, N>) -> Self {
        Self(Inner::from_utf8_unchecked(vec.into_inner()))
    }

    /// Converts a `String` into a byte vector.
    ///
    /// This consumes the `String`, so we do not need to copy its contents.
    #[inline]
    pub fn into_bytes(self) -> crate::Vec<u8, N> {
        crate::Vec::from(self.0.into_bytes())
    }

    /// Extracts a string slice containing the entire string.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Converts a `String` into a mutable string slice.
    #[inline]
    pub fn as_mut_str(&mut self) -> &mut str {
        self.0.as_mut_str()
    }

    /// Returns a mutable reference to the contents of this `String`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that the bytes passed
    /// to it are valid UTF-8. If this constraint is violated, it may cause
    /// memory unsafety issues with future users of the `String`, as the rest of
    /// the library assumes that `String`s are valid UTF-8.
    #[inline]
    pub unsafe fn as_mut_vec(&mut self) -> &mut crate::vec::Inner<u8, N> {
        self.0.as_mut_vec()
    }

    /// Appends a given string slice onto the end of this `String`.
    #[inline]
    pub fn push_str(&mut self, string: &str) -> Result<(), ()> {
        #[cfg(feature = "alloc")]
        {
            self.0.push_str(string);
            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            self.0.push_str(string)
        }
    }

    /// Returns the maximum number of elements the `String` can hold.
    ///
    /// When `alloc` feature is enabled, this is the current capacity of the `String`.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Appends the given [`char`] to the end of this `String`.
    #[inline]
    pub fn push(&mut self, c: char) -> Result<(), ()> {
        #[cfg(feature = "alloc")]
        {
            self.0.push(c);
            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            self.0.push(c)
        }
    }

    /// Shortens this `String` to the specified length.
    ///
    /// If `new_len` is greater than the string's current length, this has no
    /// effect.
    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        self.0.truncate(new_len)
    }

    /// Removes the last character from the string buffer and returns it.
    ///
    /// Returns [`None`] if this `String` is empty.
    #[inline]
    pub fn pop(&mut self) -> Option<char> {
        self.0.pop()
    }

    /// Removes a [`char`] from this `String` at a byte position and returns it.
    ///
    /// Note: Because this shifts over the remaining elements, it has a
    /// worst-case performance of *O*(*n*).
    ///
    /// # Panics
    ///
    /// Panics if `idx` is larger than or equal to the `String`'s length,
    /// or if it does not lie on a [`char`] boundary.
    #[inline]
    pub fn remove(&mut self, index: usize) -> char {
        self.0.remove(index)
    }

    /// Truncates this `String`, removing all contents.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear()
    }
}

impl<const N: usize> Default for String<N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, const N: usize> TryFrom<&'a str> for String<N> {
    type Error = ();
    #[inline]
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        <Self as core::str::FromStr>::from_str(s)
    }
}

impl<const N: usize> str::FromStr for String<N> {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Inner::from_str(s).map(Self).map_err(|_| ())
    }
}

impl<const N: usize> From<String<N>> for Vec<u8, N> {
    #[inline]
    fn from(s: String<N>) -> Self {
        s.into_bytes()
    }
}

impl<const N: usize> From<String<N>> for Inner<N> {
    #[inline]
    fn from(s: String<N>) -> Self {
        s.0
    }
}

impl<const N: usize> From<Inner<N>> for String<N> {
    #[inline]
    fn from(s: Inner<N>) -> Self {
        Self(s)
    }
}

impl<const N: usize> iter::FromIterator<char> for String<N> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        Self(Inner::from_iter(iter))
    }
}

impl<'a, const N: usize> iter::FromIterator<&'a char> for String<N> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = &'a char>>(iter: T) -> Self {
        Self(Inner::from_iter(iter))
    }
}

impl<'a, const N: usize> iter::FromIterator<&'a str> for String<N> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Self(Inner::from_iter(iter))
    }
}

impl<const N: usize> fmt::Display for String<N> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<const N: usize> hash::Hash for String<N> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.0.hash(hasher)
    }
}

impl<const N: usize> fmt::Write for String<N> {
    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.0.write_str(s)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> Result<(), fmt::Error> {
        self.0.write_char(c)
    }
}

impl<const N: usize> ops::Deref for String<N> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize> ops::DerefMut for String<N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl<const N: usize> AsRef<str> for String<N> {
    #[inline]
    fn as_ref(&self) -> &str {
        self
    }
}

impl<const N: usize> AsRef<[u8]> for String<N> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<const N1: usize, const N2: usize> PartialEq<String<N2>> for String<N1> {
    #[inline]
    fn eq(&self, rhs: &String<N2>) -> bool {
        self.0.eq(&rhs.0)
    }
}

// String<N> == str
impl<const N: usize> PartialEq<str> for String<N> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

// String<N> == &'str
impl<const N: usize> PartialEq<&str> for String<N> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.0.eq(other)
    }
}

// str == String<N>
impl<const N: usize> PartialEq<String<N>> for str {
    #[inline]
    fn eq(&self, other: &String<N>) -> bool {
        self.eq(&other.0)
    }
}

// &'str == String<N>
impl<const N: usize> PartialEq<String<N>> for &str {
    #[inline]
    fn eq(&self, other: &String<N>) -> bool {
        self.eq(&other.0)
    }
}

impl<const N: usize> Eq for String<N> {}

impl<const N1: usize, const N2: usize> PartialOrd<String<N2>> for String<N1> {
    #[inline]
    fn partial_cmp(&self, other: &String<N2>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<const N: usize> Ord for String<N> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

macro_rules! impl_try_from_num {
    ($num:ty, $size:expr) => {
        impl<const N: usize> core::convert::TryFrom<$num> for String<N> {
            type Error = ();
            #[inline]
            fn try_from(s: $num) -> Result<Self, Self::Error> {
                #[cfg(feature = "alloc")]
                {
                    Ok(Self(alloc::string::ToString::to_string(&s)))
                }
                #[cfg(not(feature = "alloc"))]
                {
                    Inner::try_from(s).map(Self)
                }
            }
        }
    };
}

impl_try_from_num!(i8, 4);
impl_try_from_num!(i16, 6);
impl_try_from_num!(i32, 11);
impl_try_from_num!(i64, 20);

impl_try_from_num!(u8, 3);
impl_try_from_num!(u16, 5);
impl_try_from_num!(u32, 10);
impl_try_from_num!(u64, 20);
