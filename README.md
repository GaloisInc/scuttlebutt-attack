# Build process

```sh
# Build the placeholder secrets library:
rustc --crate-type staticlib --crate-name secrets secrets_dummy.rs

# Build and run the trace recorder with placeholder secrets:
cargo run --bin recorder
# Generate Rust data structures matching the recording:
python3 convert_trace.py 2 2 16 512 recording.cbor >secrets.rs
# Build the real secrets library:
rustc --crate-type staticlib secrets.rs \
    --edition 2021 \
    --extern scuttlebutt_attack=$PWD/target/debug/libscuttlebutt_attack.rlib \
    -L target/debug/deps

# Build and run the victim (server) program:
cargo run --bin victim
# Build and run the attacker program:
cargo run --bin attacker_merged
```

