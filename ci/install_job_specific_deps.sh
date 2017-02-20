#!/usr/bin/env bash
# This checks what job is being run based on travis input.

set -e

if [[ ! -f ~/.cargo/bin/cargo-install-update ]]; then
    echo "cargo-update not found in cache, installing..."
    cargo install cargo-update
else
    echo "cargo-update is found in cache, installing updates for it if available."
    cargo install-update cargo-update
fi

if [[ ${RUSTFMT} == client ]] || [[ ${RUSTFMT} == server ]]; then
    if [[ ! -f ~/.cargo/bin/cargo-fmt ]]; then
        echo "rustfmt not found in the cache, installing..."
        cargo install rustfmt
    else
        echo "rustfmt is found in the cache, installing updates for it if available."
        cargo install-update rustfmt
    fi
elif [[ ${BUILD} == client ]] || [[ ${TEST} == client ]]; then
    if [[ ! -f ~/.cargo/bin/diesel ]]; then
        echo "diesel not found in the cache, installing..."
        cargo install diesel_cli --no-default-features --features="postgres sqlite"
    else
        echo "diesel is found in the cache, installing updates for it if available."
        cargo install-update diesel_cli
    fi
elif [[ ${BUILD} == server ]] || [[ ${TEST} == server ]]; then
    if [[ ! -f ~/.cargo/bin/diesel ]]; then
        echo "diesel not found in the cache, installing..."
        cargo install diesel_cli --no-default-features --features="postgres sqlite"
    else
        echo "diesel is found in the cache, installing updates for it if available."
        cargo install-update diesel_cli
    fi
fi
