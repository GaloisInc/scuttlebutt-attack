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

mkdir -p build

# Build RISC-V ASM for the victim (server) program:
RUSTC_BOOTSTRAP=1 cargo +1.56.0 rustc \
    --release -Z build-std=core,alloc --target target-rv64.json \
    --manifest-path "$package_dir/Cargo.toml" --bin "$name" \
    --features "$features" -- --emit llvm-bc -Z no-link
# The output filename contains a random hash.  Find it as follows:
bc_path="$(find target -name "$name-*.bc")"
cp "$bc_path" "build/$name.bc"

# TODO:
# - default: call fromager-link.sh on build/$name.bc, with secrets.bc
# - for building secrets, don't link
# - for building attacker, don't include any secrets

SCUTTLEBUTT_HOME="$(dirname "$0")"
PICOLIBC_HOME="$SCUTTLEBUTT_HOME/../picolibc/build/image/picolibc/riscv64-unknown-fromager"

case $name in
    secrets)
        # Don't link
        ;;
    attacker)
        # Omit secrets
        # TODO
        ;;
    *)
        # Link normally, including secrets
        # TODO: compile llvm-passes with llvm 13

        cc_objects="build/$name.bc" \
            cc_secret_objects="build/secrets.bc" \
            cc_build_dir="build/$name" \
            cc_microram_output="build/$name.ll" \
            LLVM_SUFFIX=-13 \
            LLVM_OPT_FLAGS=-enable-new-pm=0 \
            COMPILER_RT_HOME=$SCUTTLEBUTT_HOME/../llvm-project/compiler-rt/build-13 \
            bash -x $PICOLIBC_HOME/lib/fromager-link.sh microram
        llc-13 "build/$name.ll" -o "build/$name.s" -relocation-model=static -mattr=+m
        ;;
esac


# Optimize bitcode and produce assembly:
#opt-13 "$name.bc" -O3 -mattr=+m | \
#    llc-13 -o "$name.s" -relocation-model=static -mattr=+m
# Compile the asm to an object file and check for undefined symbols:
#clang -target riscv64-unknown-none-elf -c "$name.s" -o "$name.o"
#riscv64-unknown-elf-nm "$name.o" | grep ' [uU] '
# This should list only __cc_* intrinsics, CC_SSB_* secret inputs, and memcmp,
# memcpy, and memset library functions.

