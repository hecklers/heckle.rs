# Heckle

This is a proof of concept Rust compiler plugin that enables code mutation.

# How to use

Keep in mind that this library is a just a hack at this stage.

Include the following attributes to your crate:
```rust
#![feature(custom_attribute, plugin)]
#![plugin(heckle)]
```

Add the `#[heckle]` attribute on units on which you want to apply mutations.

You can see an example in `examples/src/main.rs`

Run your tests using the `bin/test` script, but from your project's directory. For instance, in this project:

```bash
$ cd examples
$ ../bin/test
```

The script will apply different mutations, one at a time, to the units you annotated with `#[heckle]` and run your tests (using `cargo test`) after each mutation.
