use criterion::{Criterion, black_box, criterion_group, criterion_main};
use bruteforce::charset::Charset;
use bruteforce::BruteForce;

const BENCH_CHARS: Charset = Charset::new(&[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '!', '\"', '\'', '?', '\\', '#', '$', '§', '%', '&', '/', '(', ')',
    '=', '[', ']', '{', '}', '´', '`', '<', '>', '€', ',', '.', '-', '_',
]);

fn bench_raw_next(c: &mut Criterion) {
    c.bench_function("bench_raw_next", |b| {
        let mut brute_forcer = BruteForce::new(black_box(BENCH_CHARS));
        b.iter(|| {
            brute_forcer.raw_next();
        });
    });
}

fn bench_next(c: &mut Criterion) {
    c.bench_function("bench_next", |b| {
        let mut brute_forcer = BruteForce::new(black_box(BENCH_CHARS));
        b.iter(|| brute_forcer.next());
    });
}

fn bench_new(c: &mut Criterion) {
    c.bench_function("bench_new", |b| {
        b.iter(|| BruteForce::new(black_box(BENCH_CHARS)));
    });
}

fn bench_charset_new(c: &mut Criterion) {
    c.bench_function("bench_charset_new", |b| {
        b.iter(|| {
            Charset::new(black_box(&[
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
                'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
                'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
                'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '\"', '\'', '?', '\\', '#',
                '$', '§', '%', '&', '/', '(', ')', '=', '[', ']', '{', '}', '´', '`', '<', '>', '€',
                ',', '.', '-', '_',
            ]));
        })
    });
}

fn bench_charset_new_by_str(c: &mut Criterion) {
    c.bench_function("bench_charset_new_by_str", |b| {
        b.iter(|| Charset::new_by_str(
            black_box(
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!\"\'?\\$§%&/()=[]{}´`<>€,.-_"
            )
        ));
    });
}

fn bench_charset_concat(c: &mut Criterion) {
    c.bench_function("bench_charset_concat", |b| {
        let c1 = Charset::new_by_str(black_box("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
        let c2 = Charset::new_by_str(black_box("abcdefghijklmnopqrstuvwxyz0123456789"));

        b.iter(|| c1.concat(&c2));
    });
}

fn bench_charset_by_range(c: &mut Criterion) {
    c.bench_function("bench_charset_by_range", |b| {
        b.iter(|| Charset::by_char_range(black_box('a'..='z')));
    });
}

fn bench_charset_to_string(c: &mut Criterion) {
    c.bench_function("bench_charset_to_string", |b| {
        b.iter(|| black_box(BENCH_CHARS).to_string());
    });
}

criterion_group!(
    all,
    bench_raw_next,
    bench_next,
    bench_new,
    bench_charset_new,
    bench_charset_new_by_str,
    bench_charset_concat,
    bench_charset_by_range,
    bench_charset_to_string,
);

criterion_main!(all);
