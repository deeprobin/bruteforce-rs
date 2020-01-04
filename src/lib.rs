//#![cfg_attr(not(feature = "std"), no_std)]
//extern crate no_std_compat as std;
//use std::prelude::v1::*;

#[cfg(feature = "constants")]
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut x = crate::BruteForce::new(crate::UPPERCASE_CHARS);
        let password = "PASS";
        let mut trys: i32 = 0;
        loop {
            trys = trys + 1;
            let out = x.next();
            if out == password.to_string() {
                println!(">>> SUCCESS ({} times)", trys);
                break;
            }
        }
        assert_eq!(2 + 2, 4);
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

pub struct BruteForce {
    chars: &'static [char],
    pub current: String,
}

impl BruteForce {
    pub fn new(charset: &'static [char]) -> BruteForce {
        BruteForce {
            chars: charset,
            current: charset[0].to_string(),
        }
    }
    pub fn new_start(charset: &'static [char], start: usize) -> BruteForce {
        let mut start_string = String::new();
        for _ in 0..start {
            start_string.push(charset[0]);
        }
        BruteForce {
            chars: charset,
            current: start_string,
        }
    }
    pub fn next(&mut self) -> String {
        let current_chars: Vec<char> = self.current.chars().collect();
        let mut s: String = String::new();
        let len: usize = *&current_chars.len();
        for n in 0..len {
            let c = &current_chars[n];
            if n != (len - 1) {
                if self.are_next_chars_last(&current_chars, n + 1) {
                    s.push(*self.next_char(c));
                } else {
                    s.push(*c);
                }
            } else if self.is_last_char(c) {
                if self.are_all_chars_last(&current_chars) {
                    s.push(*self.first_char());
                    s.push(*self.first_char());
                } else {
                    s.push(*self.first_char());
                }
            } else {
                s.push(*self.next_char(c));

            }
        }
        self.current = s.clone();
        return s;
    }

    fn are_next_chars_last(&self, chars: &Vec<char>, start: usize) -> bool {
        for n in start..chars.len() {
            if !self.is_last_char(&chars[n]) {
                return false;
            }
        }
        return true;
    }

    fn are_all_chars_last(&self, chars: &Vec<char>) -> bool {
        self.are_next_chars_last(chars, 0)
    }

    fn next_char(&self, c: &char) -> &char {
        let mut is_next: bool = false;
        for ch in self.chars {
            if is_next {
                return ch;
            } else if ch == c {
                is_next = true;
            }
        }
        return &self.chars[0];
    }

    fn is_last_char(&self, c: &char) -> bool {
        self.last_char() == c
    }

    fn first_char(&self) -> &char {
        &self.chars[0]
    }

    fn last_char(&self) -> &char {
        &self.chars[self.chars.len() - 1]
    }
}