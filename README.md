# bruteforce

[![Crates.io](https://img.shields.io/crates/v/bruteforce?style=flat-square)](https://crates.io/crates/bruteforce/)
[![Crates.io](https://img.shields.io/crates/l/bruteforce?style=flat-square)](LICENSE.md)
[![Codacy grade](https://img.shields.io/codacy/grade/6d381bdf373e4205bfd0d23876acb07d?style=flat-square)](https://app.codacy.com/manual/DeepRobin/bruteforce-rs/dashboard)

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/DeepRobin/bruteforce-rs/CI?style=flat-square)](https://github.com/deeprobin/bruteforce-rs/actions)
[![GitHub issues](https://img.shields.io/github/issues/DeepRobin/bruteforce-rs?style=flat-square)](https://github.com/deeprobin/bruteforce-rs/issues)
[![Discord](https://img.shields.io/discord/137221870452867072?style=flat-square)](https://discord.gg/mFHDMVe)

This is the fastest string generation library for brute-forcing or similar. (Supports no-std)

* [Docs - docs.rs](https://docs.rs/bruteforce/)
* [Crate information - crates.io](https://crates.io/crates/bruteforce/)

## Add to your dependencies

```toml

[dependencies]
bruteforce = "0.2.0"

```

## Example

```rust
use bruteforce::BruteForce;
let mut brute_forcer = BruteForce::new(charset!("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));

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
