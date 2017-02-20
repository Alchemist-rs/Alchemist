#!/usr/bin/env bash
# This is a job specific testing script

set -e

database () {
    if [ ! -f ./alchemist.db ]; then
        echo "Installing Database"
        diesel setup
        diesel migration run
    fi
}
build () {
    database
    echo "Building Alchemist ${BUILD}"
}
tests () {
    database
    echo "Running tests" &&
    travis-cargo test
}
bench () {
    database
    echo "Running Benches" &&
    travis-cargo bench
}
doc () {
    echo "Documenting Alchemist" &&
    travis-cargo doc
}

if [[ ${BUILD} == client ]]; then
    cd alchemist
    build
    cargo check -p alchemist
elif [[ ${BUILD} == server ]]; then
    cd alchemist_server
    build
    cargo check -p alchemist_server
elif [[ ${TEST} == client ]]; then
    cd alchemist
    tests
elif [[ ${TEST} == server ]]; then
    cd alchemist_server
    tests
elif [[ ${DOC} == client ]]; then
    cd alchemist
    doc
elif [[ ${DOC} == server ]]; then
    cd alchemist_server
    doc
elif [[ ${RUSTFMT} == client ]]; then
    cd alchemist
    cargo fmt -- --write-mode=diff
elif [[ ${RUSTFMT} == server ]]; then
    cd alchemist_server
    cargo fmt -- --write-mode=diff
fi

