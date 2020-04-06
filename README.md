# bruteforce

![Crates.io](https://img.shields.io/crates/v/bruteforce?style=flat-square)
![Crates.io](https://img.shields.io/crates/l/bruteforce?style=flat-square)
![Codacy grade](https://img.shields.io/codacy/grade/6d381bdf373e4205bfd0d23876acb07d?style=flat-square)

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/DeepRobin/bruteforce-rs/CI?style=flat-square)
![GitHub issues](https://img.shields.io/github/issues/DeepRobin/bruteforce-rs?style=flat-square)

This is a no_std-compatible zero-dependency* brute force/string generation rust-nightly library.

*   [Docs - docs.rs](https://docs.rs/bruteforce/)
*   [Crate information - crates.io](https://crates.io/crates/bruteforce/)

\* = zero-dependency if std is included

## Add to your dependencies

```toml

[dependencies]
bruteforce = "0.1.7"

```

## Example

```rust
use bruteforce::BruteForce;
let mut brute_forcer = BruteForce::new(bruteforce::UPPERCASE_CHARS);

const password: &'static str = "PASS";
for s in brute_forcer {
    if s == password.to_string() {
       println!("Password cracked");
       break;
    }
}
```

## Contribution  

If you want you can contribute. We need people, who write better documentation, optimize algorithms, implement more algorithms, finding bugs or submitting ideas.

## Bench Results

| Procession Unit                    | Charset Length | Benchmark Time (bench_raw_next) |
|------------------------------------|----------------|---------------------------------|
| Intel® Core™ i3-2100 CPU @ 3.10GHz | 90             | 26ns/iter (+/- 32)              |