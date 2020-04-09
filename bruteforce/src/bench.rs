use super::*;
use test::Bencher;

const BENCH_CHARS: Charset = Charset::new(&[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '!', '\"', '\'', '?', '\\', '#', '$', '§', '%', '&', '/', '(', ')',
    '=', '[', ']', '{', '}', '´', '`', '<', '>', '€', ',', '.', '-', '_',
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
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '\"', '\'', '?', '\\', '#',
            '$', '§', '%', '&', '/', '(', ')', '=', '[', ']', '{', '}', '´', '`', '<', '>', '€',
            ',', '.', '-', '_',
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
fn bench_charset_by_range(b: &mut Bencher) {
    b.iter(|| Charset::by_char_range('a'..='z'));
}

#[bench]
fn bench_charset_to_string(b: &mut Bencher) {
    b.iter(|| BENCH_CHARS.to_string());
}
