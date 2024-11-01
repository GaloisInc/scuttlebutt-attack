#!/bin/bash
set -xeuo pipefail

case $1 in
    secrets)
        features=microram cc_link=0 ../rust-support/build_microram.sh secrets secrets
        ;;
    secrets_dummy)
        features=microram cc_link=0 ../rust-support/build_microram.sh secrets_dummy secrets_dummy
        ;;
    victim)
        features=microram,constants,secrets ../rust-support/build_microram.sh victim
        ;;
    kernel_attacker)
        features=microram,secrets keep_symbols=__cc_syscall_handler \
            ../rust-support/build_microram.sh kernel_attacker
        ;;
    kernel_attacker_dummy)
        features=microram,secrets keep_symbols=__cc_syscall_handler \
            cc_secret_objects="build/secrets_dummy.bc" \
            ../rust-support/build_microram.sh kernel_attacker . kernel_attacker_dummy
        ;;
    checker)
        features=microram,secrets ../rust-support/build_microram.sh checker
        ;;
    attacker)
        features=microram cc_secret_objects= keep_symbols=CC_COMMITMENT_RANDOMNESS \
            ../rust-support/build_microram.sh attacker
        ;;
    *)
        echo "bad build target: $1" 1>&2
esac
