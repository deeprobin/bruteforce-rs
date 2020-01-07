# bruteforce

This is a no_std-compatible zero-dependency* brute force/string generation rust-nightly library.
* [Docs - docs.rs](https://docs.rs/bruteforce/)
* [Crate information - crates.io](https://crates.io/crates/bruteforce/)

\* = zero-dependency if std is included  

## Add to your dependencies

```toml

[dependencies]
bruteforce = "0.1.3"

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

  

If you want you can contribute. We need people, who write a better documentation, optimize algorithms, implement more algorithms, finding bugs or submitting ideas.
You can see, we use emojis for our commits. Please read this [emoji guide](https://gitmoji.carloscuesta.me/) before you commit. Thank you.
