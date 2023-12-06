#!/bin/bash
set -xeuo pipefail

params='2 2 16 512'

if [ "${1-}" = "dummy" ]; then
    python3 convert_trace.py $params >secrets_dummy/lib.rs
else
    cargo +stable run --features std,constants --bin recorder
    python3 convert_trace.py $params recording.cbor >secrets/lib.rs
fi
