#!/bin/bash
set -xeuo pipefail

name="$1"

if [[ "$#" -eq 2 ]]; then
    package_dir=$2
else
    package_dir=.
fi

features="secrets,microram"
if [[ -n "${WITH_CONSTANTS-}" ]]; then
    features="$features,constants"
fi

# Build RISC-V ASM for the victim (server) program:
RUSTC_BOOTSTRAP=1 cargo +stable rustc \
    --release -Z build-std=core --target target-rv64.json \
    --manifest-path "$package_dir/Cargo.toml" --bin "$name" \
    --features "$features" -- --emit llvm-bc -Z no-link
# The output filename contains a random hash.  Find it as follows:
bc_path="$(find target -name "$name-*.bc")"
cp "$bc_path" "$name.bc"
# Optimize bitcode and produce assembly:
opt-13 "$name.bc" -O3 -mattr=+m | \
    llc-13 -o "$name.s" -relocation-model=static -mattr=+m
# Compile the asm to an object file and check for undefined symbols:
clang -target riscv64-unknown-none-elf -c "$name.s" -o "$name.o"
riscv64-unknown-elf-nm "$name.o" | grep ' [uU] '
# This should list only __cc_* intrinsics, CC_SSB_* secret inputs, and memcmp,
# memcpy, and memset library functions.

