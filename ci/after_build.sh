#!/usr/bin/env bash
# upload docs if they were made

set -e

if [[ ${TEST} == client ]]; then
    cd alchemist
    echo "Record & upload coverage of tests with kcov"
    travis-cargo coveralls --no-sudo --verify
elif [[ ${TEST} == server ]]; then
    cd alchemist_server
    echo "Record & upload coverage of tests with kcov"
    travis-cargo coveralls --no-sudo --verify
fi

if [[ ${TRAVIS_TEST_RESULT} == 0 ]]; then
    if [[ ${DOC} == client ]]; then
        cd alchemist
        echo "Upload Alchemist docs to GH-PAGES (Only works when building the MASTER branch)"
        travis-cargo doc-upload --branch master
    elif [[ ${DOC} == server ]]; then
        cd alchemist_server
        echo "Upload Alchemist server docs to GH-PAGES (Only works when building the MASTER branch)"
        travis-cargo doc-upload --branch master
    fi
fi
