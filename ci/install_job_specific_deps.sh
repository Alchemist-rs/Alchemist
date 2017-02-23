#!/usr/bin/env bash
# This checks what job is being run based on travis input.

set -e

if [[ ! -f ~/.cargo/bin/cargo-install-update ]]; then
    echo "cargo-update not found in cache, installing..."
    cross install cargo-update
else
    echo "cargo-update is found in cache, installing updates for it if available."
    cross install-update cargo-update
fi

if [[ -n ${RUSTFMT} ]]; then
    if [[ ! -f ~/.cargo/bin/cargo-fmt ]]; then
        echo "rustfmt not found in the cache, installing..."
        cross install rustfmt
    else
        echo "rustfmt is found in the cache, installing updates for it if available."
        cross install-update rustfmt
    fi
elif [[ -n ${BUILD} ]] || [[ -n ${TEST} ]] || [[ -n ${RUN} ]]; then
    if [[ ! -f ~/.cargo/bin/diesel ]]; then
        echo "diesel not found in the cache, installing..."
        cross install diesel_cli
    else
        echo "diesel is found in the cache, installing updates for it if available."
        cross install-update diesel_cli
    fi
fi
