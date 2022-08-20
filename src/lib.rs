//! # generic_scanner
//!
//! This crate is for educational purposes. It's based on the [Lyn crate] and the blog post
//! "[A Beginner's Guide to Parsing in Rust]". A scanner is used to read the source one character
//! at a time, this characters can then be converted into tokens.
//!
//! This scanner won't turn group of characters into tokens, it's sole purpose is to aid in
//! updating and keeping the state of the scan. It was built because I wanted a small and simple
//! scanner for arrays of different types, to use in personal projects.
//!
//! [Lyn crate]: https://crates.io/crates/lyn
//! [A Beginner's Guide to Parsing in Rust]: https://depth-first.com/articles/2021/12/16/a-beginners-guide-to-parsing-in-rust/

//! # Examples
//! ```
//! use generic_scanner::Scanner;
//! fn main() {
//!     // Create a new scanner
//!     let mut scanner = Scanner::new(&[2,3,4,5,6]);
//!     assert_eq!(Some(&2), scanner.peek());
//!     assert_eq!(Some(&2), scanner.pop());
//!     assert_eq!(false, scanner.take(&5));
//!     assert_eq!(true, scanner.take(&3));
//!
//!     // Create a new scanner
//!     let mut scanner = Scanner::new(b"GET ");
//!     assert_eq!(Some(&71), scanner.peek());
//!     assert_eq!(Some(&71), scanner.pop());
//!     assert_eq!(false, scanner.take(&84));
//!     assert_eq!(true, scanner.take(&69));
//!
//!     // Create a new scanner
//!     let mut scanner = Scanner::new(&['2','3','4','5','6']);
//!     assert_eq!(Some(&'2'), scanner.peek());
//!     assert_eq!(Some(&'2'), scanner.pop());
//!     assert_eq!(false, scanner.take(&'5'));
//!     assert_eq!(true, scanner.take(&'3'));
//! }
//! ```
//!
//! Using the Scanner to find an http request method.
//! ```
//! use generic_scanner::Scanner;
//! fn main() {
//!     // Create a new scanner
//!     let mut scanner = Scanner::new(b"GET / HTTP/1.1\r\n");
//!     let metod: Vec<u8> = get_method(&mut scanner);
//!
//!     assert_eq!(b"GET".to_vec(), metod);
//!     assert_eq!([71, 69, 84].to_vec(), metod);
//!
//!     // Create a new scanner
//!     let mut scanner = Scanner::new(b"OPTIONS / HTTP/1.1\r\n");
//!     let metod: Vec<u8> = get_method(&mut scanner);
//!
//!     assert_eq!(b"OPTIONS".to_vec(), metod);
//!     assert_eq!([79, 80, 84, 73, 79, 78, 83].to_vec(), metod);
//! }
//!
//! fn get_method(scanner: &mut Scanner<u8>) -> Vec<u8>{
//!     let mut metod: Vec<u8> = vec![];
//!     // Loop until a whitespace is found
//!     while !scanner.take(&32) {
//!         match scanner.pop() {
//!             Some(value) => metod.push(*value),
//!             None => {},
//!         }
//!     }
//!     metod
//! }
//! ```

pub struct Scanner<'a, T> {
    cursor: usize,
    buffer: &'a [T]
}

impl<'a, T: std::cmp::PartialEq> Scanner<'a, T> {
    pub fn new(buffer:&'a [T]) -> Self {
        Self {
            cursor: 0,
            buffer: buffer,
        }
    }

    /// Returns the current cursor. Useful for reporting errors.
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Returns true if further progress is not possible.
    pub fn is_done(&self) -> bool {
        self.cursor == self.buffer.len()
    }

    /// Returns the next character without advancing the cursor.
    /// AKA "lookahead"
    pub fn peek(&self) -> Option<&'a T> {
        self.buffer.get(self.cursor)
    }

    /// Returns the next character (if available) and advances the cursor.
    pub fn pop(&mut self) -> Option<&'a T> {
        match self.buffer.get(self.cursor) {
            Some(character) => {
                self.cursor += 1;

                Some(character)
            }
            None => None,
        }
    }

    /// Returns true if the `target` is found at the current cursor position,
    /// and advances the cursor.
    /// Otherwise, returns false leaving the cursor unchanged.
    pub fn take(&mut self, target: &T) -> bool {
        match self.buffer.get(self.cursor) {
            Some(element) => {
                if target == element {
                    self.cursor += 1;

                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    /// Returns true if the `target` is found at the current cursor position,
    /// and advances the cursor.
    /// Otherwise, returns false leaving the cursor unchanged.
    pub fn contains(&mut self, target: &'a [T]) -> bool {
        match self.buffer.get(self.cursor..target.len()) {
            Some(stream) => {
                if target == stream {
                    self.cursor += target.len();
                    true
                }
                else {
                    false
                }
            },
            None => false,
        }
    }

    /// Returns the requested slice from the buffer without advancing the cursor
    pub fn slice(&self, start: usize, end: usize) -> Option<&'a [T]> {
        self.buffer.get(start..end)
    }
}
