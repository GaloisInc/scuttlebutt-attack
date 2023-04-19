#!/bin/bash
set -xeuo pipefail

features=microram ../rust-support/build_microram.sh secrets secrets
features=microram,constants,secrets ../rust-support/build_microram.sh victim
features=microram cc_secret_objects= keep_symbols=CC_COMMITMENT_RANDOMNESS \
    ../rust-support/build_microram.sh attacker
features=microram,secrets keep_symbols=__cc_syscall_handler \
    ../rust-support/build_microram.sh kernel_attacker
features=microram,secrets ../rust-support/build_microram.sh checker
