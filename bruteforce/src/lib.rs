//! This is the documentation for the no-std compatible `bruteforce` crate

#![crate_name = "bruteforce"]
#![feature(
    const_fn,
    test,
    generators,
    generator_trait,
    const_if_match,
    const_panic,
    proc_macro_hygiene
)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate test;

#[cfg(not(feature = "std"))]
extern crate no_std_compat as std;

#[cfg(feature = "bruteforce-macros")]
extern crate bruteforce_macros;

pub mod charset;

#[cfg(feature = "generators")]
use std::ops::{Generator, GeneratorState};
#[cfg(feature = "generators")]
use std::pin::Pin;
use std::prelude::v1::*;

use charset::Charset;

/// Represents a brute-forcing instance
#[derive(Debug, Clone)]
pub struct BruteForce<'a> {
    /// Represents the charset of the brute-forcer
    pub chars: Charset<'a>,

    /// This is the current string
    pub current: String,

    /// Reversed representation of current where each element is an index of charset
    raw_current: Vec<usize>,
}

impl<'a> BruteForce<'a> {
    /// Returns a brute forcer with default settings
    ///
    /// # Arguments
    ///
    /// * `charset` - A char array that contains all chars to be tried
    ///
    /// # Example
    ///
    /// ```rust
    /// use bruteforce::BruteForce;
    /// use bruteforce::charset::Charset;
    /// const CHARSET: Charset = Charset::new(&['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']);
    /// let mut brute_forcer = BruteForce::new(CHARSET);
    ///
    /// const password: &'static str = "PASS";
    /// for s in brute_forcer {
    /// if s == password.to_string() {
    ///        println!("Password cracked");
    ///        break;
    ///    }
    /// }
    /// ```
    pub fn new(charset: Charset) -> BruteForce {
        BruteForce {
            chars: charset,
            current: String::default(),
            // Maybe the answer is an empty string?
            raw_current: vec![],
        }
    }

    /// Returns a brute forcer skipping some letters
    ///
    /// # Arguments
    ///
    /// * `charset` - A char array that contains all chars to be tried
    /// * `start` - E.g. the known password length
    ///
    /// # Example
    ///
    /// ```rust
    /// // This example will take less time, because we know the password length
    /// use bruteforce::BruteForce;
    /// use bruteforce::charset::Charset;
    /// const CHARSET: Charset = Charset::new(&['A', 'B', 'C', 'P', 'S']); // all possible characters
    /// let mut brute_forcer = BruteForce::new_at(CHARSET, 4);
    ///
    /// const password: &'static str = "PASS";
    /// for s in brute_forcer {
    /// if s == password.to_string() {
    ///        println!("Password cracked");
    ///        break;
    ///    }
    /// }
    /// ```
    pub fn new_at(charset: Charset, start: usize) -> BruteForce {
        BruteForce {
            chars: charset,
            current: String::default(),
            raw_current: (0..start).map(|_| 0).collect::<Vec<usize>>(),
        }
    }

    /// Returns a brute forcer skipping some text
    ///
    /// # Arguments
    ///
    /// * `charset` - A char array that contains all chars to be tried
    /// * `start_string` - A string
    ///
    /// # Example
    ///
    /// ```rust
    /// // This could be useful if we want to save our brute force progress and resume it later
    /// use bruteforce::BruteForce;
    /// use bruteforce::charset::Charset;
    /// const CHARSET: Charset = Charset::new(&['A', 'B', 'C', 'P', 'S']); // all possible characters
    /// let mut brute_forcer = BruteForce::new_by_start_string(CHARSET, "CCCC".to_string());
    ///
    /// const password: &'static str = "PASS";
    /// for s in brute_forcer {
    /// if s == password.to_string() {
    ///        println!("Password cracked");
    ///        break;
    ///    }
    /// }
    /// ```
    pub fn new_by_start_string(charset: Charset, start_string: String) -> BruteForce {
        BruteForce {
            current: String::default(),
            raw_current: start_string
                .chars()
                .rev()
                .map(|c1| charset.iter().position(|&c2| c1 == c2))
                .collect::<Option<Vec<usize>>>()
                .expect("characters in start_string must exist in charset"),
            // assigning charset to chars must happen after it is used by .map()
            chars: charset,
        }
    }

    /// This returns the next element without unnecessary boxing in a Option
    pub fn raw_next(&mut self) -> &str {
        let cur = &mut self.current;
        let chars = &self.chars;

        cur.clear();
        cur.extend(self.raw_current.iter().rev().map(|&i| {
            assert!(i < chars.len(), "Bug: Invalid character index");
            chars[i]
        }));

        // "Add" 1 to self.raw_current
        let mut carryover = true;
        let chars_len_m = self.chars.len() - 1;
        for i in self.raw_current.iter_mut() {
            if *i == chars_len_m {
                *i = 0;
            } else {
                *i += 1;
                carryover = false;
                break;
            }
        }
        if carryover {
            self.raw_current.push(0);
        }

        &self.current
    }
}

impl<'a> Iterator for BruteForce<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        Some(self.raw_next().to_string())
    }
}

#[cfg(feature = "generators")]
impl Generator for Pin<&mut BruteForce<'_>> {
    type Yield = String;
    type Return = ();

    fn resume(self: Pin<&mut Self>) -> GeneratorState<Self::Yield, Self::Return> {
        GeneratorState::Yielded(self.get_mut().raw_next().to_string())
    }
}
