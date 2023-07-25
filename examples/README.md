# autd3-examples

## SOEM

```
cargo run --release --features soem --example soem
```

### For macOS and Linux users

This example requires root privileges.

```
cargo build --release --features soem --example soem && sudo ./target/release/examples/soem
```

## TwinCAT (Windows only)

```
cargo run --release --features twincat --example twincat
```

## Simulator

### Server

```
cargo run --release --features simulator_server --example simulator_server
```

### Client

```
cargo run --release --features simulator_client --example simulator_client
```

# Author

Shun Suzuki, 2022-2023
