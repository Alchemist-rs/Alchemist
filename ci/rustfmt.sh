#!/usr/bin/env bash
# This is the script for rustfmt

if [[ ${RUSTFMT} == client ]]; then
    cd alchemist
    cargo fmt -- --write-mode=diff
elif [[ ${RUSTFMT} == server ]]; then
    cd alchemist_server
    cargo fmt -- --write-mode=diff
fi
