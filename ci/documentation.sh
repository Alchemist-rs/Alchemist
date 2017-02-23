#!/usr/bin/env bash
# Doumentation script for alchemist

if [[ ${DOCUMENTATION} == client ]]; then
    cd alchemist
    cargo doc
elif [[ ${DOCUMENTATION} == server ]]; then
    cd alchemist_server
    cargo doc
fi
