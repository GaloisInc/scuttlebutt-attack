# Build process

```sh
# Build the attacker program:
./build_microram.sh attacker
# TODO: compile to microram; commit to the attacker program and step count
commitment=00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff
seed=00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff
# TODO: generate constants/lib.rs

# Run the trace recorder:
cargo run --features std,constants --bin recorder
# Generate Rust data structures matching the recording:
python3 convert_trace.py 2 2 16 512 recording.cbor >secrets/lib.rs
# Build the secrets library for MicroRAM:
./build_microram.sh secrets secrets
# It's not necessary to explicitly the library for native; Cargo will handle it
# automatically.

# Test native builds of the three main binaries:
cargo run --features constants,secrets,inline-secrets --bin victim
cargo run --features constants,secrets,inline-secrets --bin attacker
cargo run --features constants,secrets,inline-secrets --bin checker

# Build the main binaries and the kernel for MicroRAM:
WITH_CONSTANTS=1 ./build_microram.sh victim
# attacker binary was already built
./build_microram.sh checker
./build_microram.sh attacker_kernel
```


## Build configurations

* `std`, no `secrets`, default target: used to build `recorder` to generate the
  secrets file.
* `std`, `secrets`, default target: used for offline testing.  The three main
  binaries (`victim`, `attacker`, and `checker`) can be built in this mode.
* `microram`, `secrets`, rv64 target: used for the actual MicroRAM/RISC-V
  build.  The three main binaries and also the `kernel` can be built in this
  mode.
