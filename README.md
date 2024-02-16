# rc4

Simple RC4 implementation in pure Rust

```rust
fn main() {
    let mut rc4 = Rc4::new(b"White Whistle");
    assert_eq!(
        rc4.apply(b"Made in Abyss"),
        [187, 159, 250, 115, 30, 113, 48, 212, 212, 69, 128, 99, 82]
    );
}
```
