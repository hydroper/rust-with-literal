# With literal

Implicit struct initializer for Rust based on [this Rust post](https://internals.rust-lang.org/t/short-enum-variant-syntax-in-some-cases/13113/9?u=hydroper1).

In the future, this literal may allow omitting the `..` component if in the future Rust supports a way to infer the struct name.

## Limitations

* Requires a base object: either `..` for `Default::default()` or `..o` for `o`

## Usage

```rust
use with_literal::with;
let y = 0.0;
let o: S = with! { x: 0.0, y, .. };
```