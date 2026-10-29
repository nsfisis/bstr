#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// A wrapper for `&[u8]` that provides convenient string oriented trait impls.
///
/// If you need ownership or a growable byte string buffer, then use
/// [`BString`](struct.BString.html).
///
/// Using a `&BStr` is just like using a `&[u8]`, since `BStr`
/// implements `Deref` to `[u8]`. So all methods available on `[u8]`
/// are also available on `BStr`.
///
/// # Representation
///
/// A `&BStr` has the same representation as a `&str`. That is, a `&BStr` is
/// a fat pointer which consists of a pointer to some bytes and a length.
///
/// # Trait implementations
///
/// The `BStr` type has a number of trait implementations, and in particular,
/// defines equality and ordinal comparisons between `&BStr`, `&str` and
/// `&[u8]` for convenience.
///
/// The `Debug` implementation for `BStr` shows its bytes as a normal string.
/// For invalid UTF-8, hex escape sequences are used.
///
/// The `Display` implementation for `BStr` has been removed in this fork
/// because it implicitly performs a lossy conversion.
#[repr(transparent)]
pub struct BStr {
    pub(crate) bytes: [u8],
}

impl BStr {
    /// Directly creates a `BStr` slice from anything that can be converted
    /// to a byte slice.
    ///
    /// This is very similar to the [`B`](crate::B) function, except this
    /// returns a `&BStr` instead of a `&[u8]`.
    ///
    /// This is a cost-free conversion.
    ///
    /// # Example
    ///
    /// You can create `BStr`'s from byte arrays, byte slices or even string
    /// slices:
    ///
    /// ```
    /// use bstr::BStr;
    ///
    /// let a = BStr::new(b"abc");
    /// let b = BStr::new(&b"abc"[..]);
    /// let c = BStr::new("abc");
    ///
    /// assert_eq!(a, b);
    /// assert_eq!(a, c);
    /// ```
    #[inline]
    pub fn new<B: ?Sized + AsRef<[u8]>>(bytes: &B) -> &BStr {
        BStr::from_bytes(bytes.as_ref())
    }

    #[inline]
    pub(crate) fn new_mut<B: ?Sized + AsMut<[u8]>>(
        bytes: &mut B,
    ) -> &mut BStr {
        BStr::from_bytes_mut(bytes.as_mut())
    }

    #[inline]
    pub(crate) fn from_bytes(slice: &[u8]) -> &BStr {
        unsafe { &*(slice as *const [u8] as *const BStr) }
    }

    #[inline]
    pub(crate) fn from_bytes_mut(slice: &mut [u8]) -> &mut BStr {
        unsafe { &mut *(slice as *mut [u8] as *mut BStr) }
    }

    #[inline]
    #[cfg(feature = "alloc")]
    pub(crate) fn from_boxed_bytes(slice: Box<[u8]>) -> Box<BStr> {
        unsafe { Box::from_raw(Box::into_raw(slice) as _) }
    }

    #[inline]
    #[cfg(feature = "alloc")]
    pub(crate) fn into_boxed_bytes(slice: Box<BStr>) -> Box<[u8]> {
        unsafe { Box::from_raw(Box::into_raw(slice) as _) }
    }

    #[inline]
    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Provides a escape hatch for lossily displaying byte strings as UTF-8.
    ///
    /// ```
    /// use bstr::BStr;
    ///
    /// let s = BStr::new(b"foo\xFFbar");
    /// assert_eq!(format!("{}", s.display_lossy()), "foo\u{FFFD}bar");
    /// assert_eq!(format!("{:-^9}", s.display_lossy()), "-foo\u{FFFD}bar-");
    /// ```
    ///
    /// ```compile_fail
    /// use bstr::BStr;
    ///
    /// let _ = BStr::new(b"foo").to_string();
    /// ```
    ///
    /// ```compile_fail
    /// use bstr::BStr;
    ///
    /// let _ = format!("{}", BStr::new(b"foo"));
    /// ```
    ///
    /// ```compile_fail
    /// use bstr::BString;
    ///
    /// let _ = BString::from("foo").to_string();
    /// ```
    ///
    /// ```compile_fail
    /// use bstr::BString;
    ///
    /// let _ = format!("{}", BString::from("foo"));
    /// ```
    #[inline]
    pub fn display_lossy(&self) -> DisplayLossy<'_> {
        DisplayLossy(self)
    }
}

/// Helper struct for lossily displaying byte strings as UTF-8.
#[derive(Clone, Copy, Debug)]
pub struct DisplayLossy<'a>(pub(crate) &'a BStr);
