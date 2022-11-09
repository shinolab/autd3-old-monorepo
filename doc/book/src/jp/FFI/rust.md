# Rust

[rust-autd](https://github.com/shinolab/autd3/tree/master/rust)ではRust版のライブラリを提供している.

Rust版のライブラリはC++版をラップしたものではなく, Rustで再実装している.
そのため, 一部機能が異なる可能性がある.

## Installation

[crate.io](https://crates.io/crates/autd3)で公開しているので, `Cargo.toml`にて以下のようにすれば良い.

```
[dependencies]
autd3 = "2.6.0"
```

また, 各種Link, Gain等は別crateとして公開しているので必要に応じてdependenciesに追加すること.
```
[dependencies]
autd3-link-soem = "2.6.0"
autd3-link-twincat = "2.6.0"
autd3-link-simulator = "2.6.0"
autd3-gain-holo = "2.6.0"
```

## Usage

基本的には, C++版と同じになるように設計している.

たとえば, [Getting Started](../Users_Manual/getting_started.md)と等価なコードは以下のようになる.

```rust
{{#include ../../../samples/rust/sample/src/main.rs}}
```

注意点として, Rust版の`send`関数は引数を一つしか取らない. 
ヘッダーとボディーデータを同時に送りたいときは`send`を続けて呼び出し, そうでない場合は, `flush`を呼ぶこと.
```rust
    autd.send(&mut m).flush().unwrap();
```

より詳細なサンプルは[rust-autdのexample](https://github.com/shinolab/autd3/tree/master/rust/autd3-examples)を参照されたい.

## Trouble shooting

質問があれば[GitHubのissue](https://github.com/shinolab/autd3/issues)に送られたい.
