use std::prelude::v1::*; // needed for std-compat

use std::borrow::Cow;
use std::ops::Index;
use std::slice::Iter;

/// The charset representation for bruteforce
#[derive(Debug, Clone)]
pub struct Charset<'a> {
    chars: Cow<'a, [char]>,
}

impl<'a> Charset<'a> {
    /// This creates a new charset by a defined slice of chars in compile-time
    pub const fn new(charset: &[char]) -> Charset {
        Charset {
            chars: Cow::Borrowed(charset),
        }
    }

    /// This function creates a charset by &str
    pub fn new_by_str(s: &str) -> Charset<'a> {
        let vec = s.chars().collect::<Vec<char>>();
        Charset {
            chars: Cow::Owned(vec),
        }
    }

    /// This function concat's 2 charsets
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
    pub fn len(&self) -> usize {
        self.chars.len()
    }

    /// If the length of the internal char slice is zero, this will return true
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

    /// This function returns the iterator of the internal char slice
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
