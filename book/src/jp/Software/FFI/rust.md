# Rust

[rust-autd](https://github.com/shinolab/rust-autd)ではRust版のライブラリを提供している.

Rust版のライブラリはC++版をラップしたものではなく, Rustで再実装している.
そのため, 一部機能が異なる可能性がある.

## Installation

[crate.io](https://crates.io/crates/autd3)で公開しているので, `Cargo.toml`にて以下のようにすれば良い.

```
[dependencies]
autd3 = "2.2.1"
```

また, 各種Link, Gain等は別crateとして公開しているので必要に応じてdependenciesに追加すること.
```
[dependencies]
autd3-link-soem = "2.2.1"
autd3-link-twincat = "2.2.1"
autd3-link-emulator = "2.2.1"
autd3-gain-holo = "2.2.1"
```

## Usage

基本的には, C++版と同じになるように設計している.

たとえば, [Getting Started](../Users_Manual/getting_started.md)と等価なコードは以下のようになる.

```rust
use autd3::prelude::*;
use autd3_link_soem::{Config, EthernetAdapters, SOEM};
use std::io::{self, Write};

fn get_adapter() -> String {
    let adapters: EthernetAdapters = Default::default();
    adapters
        .into_iter()
        .enumerate()
        .for_each(|(index, adapter)| {
            println!("[{}]: {}", index, adapter);
        });

    let i: usize;
    loop {
        let mut s = String::new();
        print!("Choose number: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut s).unwrap();
        match s.trim().parse() {
            Ok(num) if num < adapters.len() => {
                i = num;
                break;
            }
            _ => continue,
        };
    }
    let adapter = &adapters[i];
    adapter.name.to_string()
}

fn main() {
    let mut geometry = GeometryBuilder::new().legacy_mode().build();
    geometry.add_device(Vector3::zeros(), Vector3::zeros());

    let ifname = get_adapter();
    let config = Config {
        cycle_ticks: 1,
        high_precision_timer: true,
    };
    let link = SOEM::new(&ifname, geometry.num_devices() as u16, config, |msg| {
        eprintln!("unrecoverable error occurred: {}", msg);
        std::process::exit(-1);
    });
    let mut autd = Controller::open(geometry, link).expect("Failed to open");

    autd.clear().unwrap();

    autd.synchronize().unwrap();

    println!("***** Firmware information *****");
    autd.firmware_infos().unwrap().iter().for_each(|firm_info| {
        println!("{}", firm_info);
    });
    println!("********************************");

    let silencer_config = SilencerConfig::default();
    autd.config_silencer(silencer_config).unwrap();

    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0);

    let mut g = Focus::new(center);
    let mut m = Sine::new(150);

    autd.send(&mut m).send(&mut g).unwrap();

    let mut _s = String::new();
    io::stdin().read_line(&mut _s).unwrap();

    autd.close().unwrap();
}
```

注意点として, Rust版の`send`関数は引数を一つしか取らない. 
ヘッダーとボディーデータを同時に送りたいときは`send`を続けて呼び出し, そうでない場合は, `flush`を呼ぶこと.
```rust
    autd.send(&mut m).flush().unwrap();
```

より詳細なサンプルは[rust-autdのexample](https://github.com/shinolab/rust-autd/tree/master/autd3-examples)を参照されたい.

## Trouble shooting

質問があれば[GitHubのissue](https://github.com/shinolab/rust-autd/issues)にてお願いします.
