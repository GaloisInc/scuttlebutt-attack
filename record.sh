#!/bin/bash
set -xeuo pipefail

cargo run --features std,constants --bin recorder
python3 convert_trace.py 2 2 16 512 recording.cbor >secrets/lib.rs
