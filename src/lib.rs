#![crate_name = "bruteforce"]
#![feature(const_fn)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate no_std_compat as std;

use std::prelude::v1::*;

#[cfg(test)]
mod tests {

    #[cfg(feature = "constants")]
    #[test]
    fn test_cracker() {
        let mut x = crate::BruteForce::new(crate::UPPERCASE_CHARS);
        let password = "PASS";
        let mut trys: i32 = 0;

        loop {
            trys = trys + 1;
            let out = x.raw_next();
            if out == password.to_string() {
                println!(">>> SUCCESS ({} times)", trys);
                break;
            }
        }
    }
}

#[cfg(feature = "constants")]
pub const UPPERCASE_CHARS: &'static [char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[cfg(feature = "constants")]
pub const LOWERCASE_CHARS: &'static [char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

#[cfg(feature = "constants")]
pub const NUMBER_CHARS: &'static [char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[cfg(feature = "constants")]
pub const SPECIAL_CHARS: &'static [char] = &[
    '!', '\"', '\'', '?', '\\', '#', '$', '§', '%', '&', '/', '(', ')', '=', '[', ']', '(', ')',
    '{', '}', '´', '`', '<', '>', '€', ',', '.', '-', '_',
];

/// Represents a brute-forcing instance
pub struct BruteForce {
    /// Represents the charset of the brute-forcer
    pub chars: &'static [char],

    /// This is the current string
    pub current: String,
}

impl BruteForce {
    /// Returns a brute forcer with default settings
    ///
    /// # Arguments
    ///
    /// * `charset` - A static char array that contains all chars to be tried
    ///
    /// # Example
    ///
    /// ```
    /// use bruteforce::BruteForce;
    /// let mut brute_forcer = BruteForce::new(bruteforce::UPPERCASE_CHARS);
    ///
    /// const password: &'static str = "PASS";
    /// for s in brute_forcer {
    /// if s == password.to_string() {
    ///        println!("Password cracked");
    ///        break;
    ///    }
    /// }
    /// ```

    pub fn new(charset: &'static [char]) -> BruteForce {
        BruteForce {
            chars: charset,
            current: charset[0].to_string(),
        }
    }

    /// Returns a brute forcer skipping some letters
    /// ///
    /// # Arguments
    ///
    /// * `charset` - A static char array that contains all chars to be tried
    /// * `start` - E.g. the known password length
    ///
    /// # Example
    ///
    /// ```
    /// // This example will take less time, because we know the password length
    /// use bruteforce::BruteForce;
    /// let mut brute_forcer = BruteForce::new_at(bruteforce::UPPERCASE_CHARS, 4);
    ///
    /// const password: &'static str = "PASS";
    /// for s in brute_forcer {
    /// if s == password.to_string() {
    ///        println!("Password cracked");
    ///        break;
    ///    }
    /// }
    /// ```

    pub fn new_at(charset: &'static [char], start: usize) -> BruteForce {
        let mut start_string = String::new();

        for _ in 0..start {
            start_string.push(charset[0]);
        }

        BruteForce {
            chars: charset,
            current: start_string,
        }
    }

    /// Returns a brute forcer skipping some text
    /// ///
    /// # Arguments
    ///
    /// * `charset` - A static char array that contains all chars to be tried
    /// * `start_string` - A string
    ///
    /// # Example
    ///
    /// ```
    /// // This could be useful if we want to save our brute force progress and resume it later
    /// use bruteforce::BruteForce;
    /// let mut brute_forcer = BruteForce::new_by_start_string(bruteforce::UPPERCASE_CHARS, "CCCC".to_string());
    ///
    /// const password: &'static str = "PASS";
    /// for s in brute_forcer {
    /// if s == password.to_string() {
    ///        println!("Password cracked");
    ///        break;
    ///    }
    /// }
    /// ```

    pub const fn new_by_start_string(charset: &'static [char], start_string: String) -> BruteForce {
        BruteForce {
            chars: charset,
            current: start_string,
        }
    }

    /// This returns the next element without unnecessary boxing in a Option

    pub fn raw_next(&mut self) -> String {
        let current_chars = &self.current;
        let mut s: String = String::new();
        let len: usize = current_chars.len();

        for (n,c) in current_chars.chars().enumerate() {
            if n != (len - 1) {
                if self.are_next_chars_last(current_chars, n + 1) {
                    s.push(self.next_char(c));
                } else {
                    s.push(c);
                }
            } else if self.is_last_char(c) {
                if self.are_all_chars_last(current_chars) {
                    s.push(self.first_char());

                    s.push(self.first_char());
                } else {
                    s.push(self.first_char());
                }
            } else {
                s.push(self.next_char(c));
            }
        }

        self.current = s.clone();
        return s;
    }

    fn are_next_chars_last(&self, chars: &String, start: usize) -> bool {
        chars.chars().skip(start).all(|c| self.is_last_char(c))
    }

    fn are_all_chars_last(&self, chars: &String) -> bool {
        self.are_next_chars_last(chars, 0)
    }

    fn next_char(&self, c: char) -> char {
        if let Some(&ch)=self.chars.iter()
            .skip_while(|&&ch| ch!=c).nth(1)
        {ch} else {self.chars[0]}
    }

    fn is_last_char(&self, c: char) -> bool {
        self.last_char() == c
    }

    const fn first_char(&self) -> char {
        self.chars[0]
    }

    const fn last_char(&self) -> char {
        self.chars[self.chars.len() - 1]
    }
}

impl Iterator for BruteForce {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        Some(self.raw_next())
    }
}
