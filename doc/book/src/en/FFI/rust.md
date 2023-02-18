# Rust

[Rust-autd](https://github.com/shinolab/autd3/tree/master/rust) provides a Rust version of the library.

The Rust version of the library is not a wrapping of the C++ version but a re-implementation in Rust.
Therefore, some features may be different.

## Installation

The Rust version is available at [crate.io](https://crates.io/crates/autd3), so you can install it as follows.

```
[dependencies]
autd3 = "8.1.2"
```

Also, you can add links, gains, etc., to dependencies as needed since they are available as separate crates.

```
[dependencies]
autd3-link-soem = "8.1.2"
autd3-link-twincat = "8.1.2"
autd3-link-simulator = "8.1.2"
autd3-gain-holo = "8.1.2"
```

## Usage

Basically, this is designed to be the same as the C++ version.

For example, the following code is equivalent to [Getting Started](../Users_Manual/getting_started.md).

```rust
{{#include ../../../samples/rust/sample/src/main.rs}}
```

Note that the Rust version of the `send` function takes only one argument. 
If you want to send header and body data at the same time, chain `send`; otherwise, call `flush`.
```rust
    autd.send(&mut m).flush().unwrap();
```

See [rust-autd example](https://github.com/shinolab/autd3/tree/master/rust/autd3-examples) for a more detailed sample.

## Troubleshooting

If you have any questions, please send them to [GitHub issue](https://github.com/shinolab/autd3/issues).
