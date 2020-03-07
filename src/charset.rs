use std::prelude::v1::*; // needed for std-compat

use std::borrow::Cow;
use std::ops::Index;
use std::ops::RangeInclusive;
use std::slice::Iter;

/// The charset representation for bruteforce
///
/// # Example
///
/// ```rust
/// use bruteforce::charset::Charset;
///
/// let uppercase_alphabeth = Charset::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
/// ```
#[derive(Debug, Clone)]
pub struct Charset<'a> {
    chars: Cow<'a, [char]>,
}

impl<'a> Charset<'a> {
    /// This creates a new charset by a defined slice of chars in compile-time
    ///
    /// # Arguments
    ///
    /// * `charset` - A char slice which contains the chars
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bruteforce::charset::Charset;
    ///
    /// Charset::new(&['A', 'B', 'C', 'D', 'E']);
    /// ```
    pub const fn new(charset: &[char]) -> Charset {
        assert!(
            !charset.is_empty(),
            "The [char] must contain at least one character"
        );
        Charset {
            chars: Cow::Borrowed(charset),
        }
    }

    /// This function creates a new charset by a defined char range
    ///
    /// # Arguments
    ///
    /// * `range` - A char range
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bruteforce::charset::Charset;
    ///
    /// Charset::by_char_range('a'..='z'); // = abcdefghijklmnopqrstuvwxyz
    /// ```
    pub fn by_char_range(range: RangeInclusive<char>) -> Charset<'a> {
        let start: u32 = *range.start() as u32;
        let end: u32 = *range.end() as u32;
        let mut vec: Vec<char> = Vec::with_capacity((end - start) as usize);
        (start..end).for_each(|i| {
            if let Some(ch) = std::char::from_u32(i) {
                vec.push(ch)
            }
        });
        Charset {
            chars: Cow::Owned(vec),
        }
    }

    /// This function creates a new charset by a defined char range ignoring utf-validity
    ///
    /// # Arguments
    ///
    /// * `range` - A unchecked char range
    ///
    /// # Safety
    ///
    /// This function is unsafe, as it may construct invalid `char` values.
    ///
    /// For a safe version of this function, see the [`by_char_range`] function.
    ///
    /// [`by_char_range`]: fn.by_char_range.html
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bruteforce::charset::Charset;
    ///
    /// let charset = unsafe { Charset::by_char_range_unchecked('a'..='z') };
    /// ```
    pub unsafe fn by_char_range_unchecked(range: RangeInclusive<char>) -> Charset<'a> {
        let start: u32 = *range.start() as u32;
        let end: u32 = *range.end() as u32;
        let mut vec: Vec<char> = Vec::with_capacity((end - start) as usize);
        (start..end).for_each(|i| vec.push(std::char::from_u32_unchecked(i)));
        Charset {
            chars: Cow::Owned(vec),
        }
    }

    /// This function creates a charset by &str
    ///
    /// # Arguments
    ///
    /// * `s` - The char slice from `new` but easier to write
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bruteforce::charset::Charset;
    ///
    /// Charset::new_by_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    ///
    /// // But it is recommended to write:
    /// Charset::from("ABCDEFGHIJKLMNOPRSTUVWXYZ");
    /// ```
    pub fn new_by_str(s: &str) -> Charset<'a> {
        assert!(
            !s.is_empty(),
            "The string must contain at least one character"
        );
        let vec = s.chars().collect::<Vec<char>>();
        Charset {
            chars: Cow::Owned(vec),
        }
    }

    /// This function concat's 2 charsets
    ///
    /// # Arguments
    ///
    /// `self`
    ///
    /// `other` - Pointer to other charset
    ///
    /// # Example
    ///
    /// ```rust
    /// use bruteforce::charset::Charset;
    ///
    /// let foo = Charset::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let bar = Charset::from("abcdefghijklmnopqrstuvwxyz");
    /// let result = foo.concat(&bar);
    /// ```
    pub fn concat(&self, other: &Charset) -> Charset<'a> {
        let mut s = self.clone();
        for &ch in other.iter() {
            if !s.chars.contains(&ch) {
                s.chars.to_mut().push(ch);
            }
        }
        s
    }

    /// This function returns the length of the internal char slice
    ///
    /// # Example
    /// ```rust
    /// use bruteforce::charset::Charset;
    ///
    /// let charset = Charset::from("ABCDEF");
    /// charset.len(); // = 6
    /// ```
    pub fn len(&self) -> usize {
        self.chars.len()
    }

    /// If the length of the internal char slice is zero, this will return true
    ///
    /// This function returns in all cases true, because the minimum size of the internal slice is 1
    #[inline]
    #[cold]
    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

    /// This function returns the immutable iterator of the internal char slice
    pub fn iter(&self) -> Iter<'_, char> {
        self.chars.iter()
    }
}

impl Index<usize> for Charset<'_> {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.chars[index]
    }
}

impl<'a> From<&'a str> for Charset<'_> {
    fn from(input: &'a str) -> Self {
        Self::new_by_str(input)
    }
}

impl From<String> for Charset<'_> {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

impl std::fmt::Display for Charset<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.chars.iter().try_for_each(|&c| write!(fmt, "{}", c))
    }
}
