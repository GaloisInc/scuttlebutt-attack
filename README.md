# Updating the commitment and trace

This must be done after any changes that affect the attacker program.

```sh
# Build the attacker program
../scripts/build_scuttlebutt_attacker_cbor

# Update the commitment based on the new attacker program
COMMITMENT_TOOL=$PWD/../witness-checker/target/release/commitment_tool \
    python3 update_commitment.py ../out/scuttlebutt/ssb-attacker.cbor
# Generate new trace to match the new commitment and seed
./record.sh
```

# Native builds

```sh
cargo run --features constants,secrets,inline-secrets --bin victim
cargo run --features constants,secrets,inline-secrets --bin attacker
cargo run --features constants,secrets,inline-secrets --bin checker
```
