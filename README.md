# generic_scanner

## generic_scanner

This crate is for educational purposes. It's based in the [Lyn crate] and the blog post
"[A Beginner's Guide to Parsing in Rust]". A scanner is used to read the source one character
at a time, this characters can then be converted into tokens.

This scanner won't turn group of characters into tokens, it's sole purpose is to aid in
updating and keeping the state of the scan. It was built because I wanted a small and simple
scanner for arrays of different types, to use in personal projects.

[Lyn crate]: https://crates.io/crates/lyn
[A Beginner's Guide to Parsing in Rust]: https://depth-first.com/articles/2021/12/16/a-beginners-guide-to-parsing-in-rust/
## Examples
```rust
fn main() {
    // Create a new scanner
    let mut scanner = Scanner::new(&[2,3,4,5,6]);
    assert_eq!(Some(&2), scanner.peek());
    assert_eq!(Some(&2), scanner.pop());
    assert_eq!(false, scanner.take(&5));
    assert_eq!(true, scanner.take(&3));

    // Create a new scanner
    let mut scanner = Scanner::new(b"GET ");
    assert_eq!(Some(&71), scanner.peek());
    assert_eq!(Some(&71), scanner.pop());
    assert_eq!(false, scanner.take(&84));
    assert_eq!(true, scanner.take(&69));
}
```

Using the Scanner to find an http request method.
```rust
fn main() {
    // Create a new scanner
    let mut scanner = Scanner::new(b"GET / HTTP/1.1\r\n");
    let metod: Vec<u8> = get_method(&mut scanner);

    assert_eq!(b"GET".to_vec(), metod);
    assert_eq!([71, 69, 84].to_vec(), metod);

    // Create a new scanner
    let mut scanner = Scanner::new(b"OPTIONS / HTTP/1.1\r\n");
    let metod: Vec<u8> = get_method(&mut scanner);

    assert_eq!(b"OPTIONS".to_vec(), metod);
    assert_eq!([79, 80, 84, 73, 79, 78, 83].to_vec(), metod);
}

fn get_method(scanner: &mut Scanner<u8>) -> Vec<u8>{
    let mut metod: Vec<u8> = vec![];
    // Loop until a whitespace is found
    while !scanner.take(&32) {
        metod.push(*scanner.pop().unwrap());
    }
    metod
}
```
