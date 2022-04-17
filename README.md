# everybody_loops

A Replacement for the `-Z unpretty=everybody_loops` pass to rustc.

## Usage

Take a file with a lot of functions, you want to get rid of them.

```rust
fn main() {
    let x = 1;
    let y = 2;
    let z = x + y;
}
struct Foo;

impl std::fmt::Debug for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Foo")
    }
}

impl Foo {
    fn boo(&self) {
        println!("boo");
    }
}

const f: () = {
    fn noop() {
        let x = 8;
    }
}
```

Just run

```
everybody_loops file.rs
```

And now you have a with much simpler code.

```rust
fn main() {
    loop {}
}
struct Foo;
impl std::fmt::Debug for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        loop {}
    }
}
impl Foo {
    fn boo(&self) {
        loop {}
    }
}
const f: () = {
    fn noop() {
        loop {}
    }
};
```

## Installation

```
cargo install --frozen everybody_loops
```

## Limitations

This will not work for functions with `impl Trait` in the return type, as the
compiller needs to ascribe a concreat type.

## Prior Art

- This used to be a compiller feature, but it was removed in
  [#93913](https://github.com/rust-lang/rust/pull/93913)
- [This
  post](http://blog.pnkfx.org/blog/2019/11/18/rust-bug-minimization-patterns/#L..loopification.....via.pretty-printer)
  by [Felix S Klock II](http://pnkfx.org/pnkfelix/) first inroduced me to this
  technique.
- The code is a thin wrapper over `syn` and `prettyplease`, both by
  [dtolnay](https://github.com/dtolnay/), who diserves far more credit than me
  for making this happen.

  
#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>