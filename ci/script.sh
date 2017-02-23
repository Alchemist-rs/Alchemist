# This script takes care of testing your crate

set -ex

build() {
    RUST_BACKTRACE=1 cross build --target $TARGET --verbose -p ${BUILD}
    RUST_BACKTRACE=1 cross build --target $TARGET --release --verbose -p ${BUILD}
}

tests() {
    RUST_BACKTRACE=1 cross test --target $TARGET --verbose -p ${TEST}
    RUST_BACKTRACE=1 cross test --target $TARGET --release --verbose -p ${TEST}
}

run() {
    RUST_BACKTRACE=1 cross run --target $TARGET --verbose -p ${RUN}
    RUST_BACKTRACE=1 cross run --target $TARGET --release --verbose -p ${RUN}
}

# TODO This is the "test phase", tweak it as you see fit
main() {
    if [[ -n ${BUILD} ]]; then
        build
        return
    elif [[ -n ${TEST} ]]; then
        tests
        return
    elif [[ -n ${RUN} ]]; then
        run
        return
    fi
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
