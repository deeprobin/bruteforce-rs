//! This is the documentation for the `bruteforce` crate
//! It has also no-std support.
//! It is ready to get implemented to your projects

#![crate_name = "bruteforce"]
#![feature(const_fn)]
#![feature(vec_leak)]
#![feature(test)]
#![feature(generators, generator_trait)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate test;

#[cfg(not(feature = "std"))]
extern crate no_std_compat as std;

use std::prelude::v1::*;

use charset::Charset;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

pub mod charset;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const BENCH_CHARS: Charset = Charset::new(&[
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '!', '\"', '\'', '?', '\\', '#', '$', '§', '%',
        '&', '/', '(', ')', '=', '[', ']', '{', '}', '´', '`', '<', '>', '€', ',', '.', '-', '_',
    ]);

    #[bench]
    fn bench_raw_next(b: &mut Bencher) {
        let mut brute_forcer = BruteForce::new(BENCH_CHARS);
        b.iter(|| {
            brute_forcer.raw_next();
        });
    }

    #[bench]
    fn bench_next(b: &mut Bencher) {
        let mut brute_forcer = BruteForce::new(BENCH_CHARS);
        b.iter(|| brute_forcer.next());
    }

    #[bench]
    fn bench_new(b: &mut Bencher) {
        b.iter(|| BruteForce::new(BENCH_CHARS));
    }

    #[bench]
    fn bench_charset_new(b: &mut Bencher) {
        b.iter(|| {
            Charset::new(&[
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
                'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '\"',
                '\'', '?', '\\', '#', '$', '§', '%', '&', '/', '(', ')', '=', '[', ']', '{', '}',
                '´', '`', '<', '>', '€', ',', '.', '-', '_',
            ])
        });
    }

    #[bench]
    fn bench_charset_new_by_str(b: &mut Bencher) {
        b.iter(|| Charset::new_by_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!\"\'?\\$§%&/()=[]{}´`<>€,.-_"));
    }

    #[bench]
    fn bench_charset_concat(b: &mut Bencher) {
        let c1 = Charset::new_by_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let c2 = Charset::new_by_str("abcdefghijklmnopqrstuvwxyz0123456789");

        b.iter(|| c1.concat(&c2));
    }

    #[bench]
    fn bench_charset_to_string(b: &mut Bencher) {
        b.iter(|| BENCH_CHARS.to_string());
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_multibyte_char() {
        //Subset of Basic Latin and Latin Extended-A
        const TEST_CHARS: Charset = Charset::new(&[
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'Ā', 'ā', 'Ă', 'ă', 'Ą', 'ą', 'Ć', 'ć', 'Ĉ', 'ĉ', 'Ċ', 'ċ', 'Č', 'č', 'Ď', 'ď', 'Đ',
        ]);
        let mut x = BruteForce::new(TEST_CHARS);
        let password = "EGĀĎ";

        for trys in 1.. {
            let out = x.raw_next();
            if out == password {
                println!(">>> SUCCESS ({} times)", trys);
                break;
            }
        }
    }

    #[cfg(all(feature = "constants", feature = "std"))]
    #[test]
    fn test_combined_charset() {
        let charset = UPPERCASE_CHARS
            .concat(&LOWERCASE_CHARS)
            .concat(&NUMBER_CHARS)
            .concat(&SPECIAL_CHARS);

        let mut x = BruteForce::new(charset);
        //Use length<=3 and start with an early character to finish quickly...
        let password = "Bb8";

        for trys in 1.. {
            let out = x.raw_next();
            println!("{},{}", out, trys);
            if out == password {
                println!(">>> SUCCESS ({} times)", trys);
                break;
            }
        }
    }
}

/// Uppercase characters from `A` to `Z`
#[cfg(feature = "constants")]
pub const UPPERCASE_CHARS: Charset = Charset::new(&[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
]);

/// Lowercase characters from `a` to `z`
#[cfg(feature = "constants")]
pub const LOWERCASE_CHARS: Charset = Charset::new(&[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
]);

/// Number characters from `0` to `9`
#[cfg(feature = "constants")]
pub const NUMBER_CHARS: Charset = Charset::new(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);

/// Most used special characters
#[cfg(feature = "constants")]
pub const SPECIAL_CHARS: Charset = Charset::new(&[
    '!', '\"', '\'', '?', '\\', '#', '$', '§', '%', '&', '/', '(', ')', '=', '[', ']', '{', '}',
    '´', '`', '<', '>', '€', ',', '.', '-', '_',
]);

/// Represents a brute-forcing instance
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
        // Generate self.current from self.raw_current
        // This doesn't allocate because it has no content.
        let mut temp = String::default();
        // Borrow splitting workaround. https://doc.rust-lang.org/nomicon/borrow-splitting.html
        std::mem::swap(&mut self.current, &mut temp);
        temp.clear();
        temp.extend(self.raw_current.iter().rev().map(|&i| {
            assert!(i < self.chars.len(), "Bug: Invalid character index");
            self.chars[i]
        }));
        self.current = temp;

        // "Add" 1 to self.raw_current
        let mut carryover = true;
        for i in self.raw_current.iter_mut() {
            *i += 1;
            if *i == self.chars.len() {
                *i = 0;
            } else {
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

impl Generator for Pin<&mut BruteForce<'_>> {
    type Yield = String;
    type Return = ();

    fn resume(self: Pin<&mut Self>) -> GeneratorState<Self::Yield, Self::Return> {
        GeneratorState::Yielded(self.get_mut().raw_next().to_string())
    }
}
