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

# Build RISC-V ASM for the victim (server) program:
RUSTC_BOOTSTRAP=1 cargo +stable rustc \
    --release -Z build-std=core --target ../../target-rv64.json -- \
    --emit llvm-bc -Z no-link --cfg microram
# The output filename contains a random hash.  Find it as follows:
find target -name \*victim\*.bc
cp target/target-rv64/release/deps/victim-7a6dee58ca8824bb.bc victim.bc
# Optimize bitcode and produce assembly:
opt-13 victim.bc -O3 -mattr=+m | \
    llc-13 -o victim.s -relocation-model=static -mattr=+m
# Compile the asm to an object file and check for undefined symbols:
clang -target riscv64-unknown-none-elf -c victim.s -o victim.o
riscv64-unknown-elf-nm victim.o | grep ' [uU] '
# This should list only __cc_* intrinsics, CC_SSB_* secret inputs, and memcmp,
# memcpy, and memset library functions.

```


## Build configurations

* `std`, no `secrets`, default target: used to build `recorder` to generate the
  secrets file.
* `std`, `secrets`, default target: used for offline testing.  The three main
  binaries (`victim`, `attacker`, and `checker`) can be built in this mode.
* `microram`, `secrets`, rv64 target: used for the actual MicroRAM/RISC-V
  build.  The three main binaries and also the `kernel` can be built in this
  mode.
