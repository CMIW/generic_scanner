use generic_scanner::Scanner;

#[test]
fn success_match_stream() {
    let request: &[u8] = b"OPTION /test/uri HTTP/1.1\r\n";
    let target: &[u8] = b"OPTION";
    let mut scanner = Scanner::new(request);
    assert_eq!(true,scanner.contains(target));
    assert_eq!(6,scanner.cursor());
}
