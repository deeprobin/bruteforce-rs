use std::prelude::v1::*; // needed for std-compat

use std::convert::From;
use std::ops::Index;
use std::slice::Iter;
use std::string::ToString;

/// The charset representation for bruteforce
#[derive(Debug, Copy, Clone)]
pub struct Charset<'a> {
    chars: &'a [char],
}

impl<'a> Charset<'a> {
    /// This creates a new charset by a defined slice of chars in compile-time
    pub const fn new(charset: &[char]) -> Charset {
        Charset { chars: charset }
    }

    /// This function creates a charset by &str
    pub fn new_by_str(s: &'a str) -> Charset<'a> {
        let vec = s.chars().collect::<Vec<char>>();
        Charset {
            chars: Vec::leak(vec),
        }
    }

    /// This function concat's 2 charsets
    pub fn concat(&mut self, other: Charset) -> Charset<'a> {
        Charset::from(concat_charset_strings(self.to_string(), other.to_string()))
    }

    /// This function returns the length of the internal char slice
    pub const fn len(&self) -> usize {
        self.chars.len()
    }

    /// If the length of the internal char slice is zero, this will return true
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
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

impl<'a> From<&'a str> for Charset<'a> {
    fn from(input: &'a str) -> Self {
        Charset::new_by_str(input)
    }
}

impl From<String> for Charset<'_> {
    fn from(s: String) -> Self {
        Charset::new_by_str(Box::leak(s.into_boxed_str()))
    }
}

impl ToString for Charset<'_> {
    fn to_string(&self) -> String {
        let mut s = String::default();
        for ch in self.chars {
            s.push(*ch);
        }
        s
    }
}

fn concat_charset_strings(s1: String, s2: String) -> String {
    let mut s = s1;
    for ch in s2.chars() {
        if !s.contains(ch) {
            s.push(ch);
        }
    }
    s
}
