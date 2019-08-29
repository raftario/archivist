# This script takes care of testing your crate

set -ex

main() {
    cross build --target $TARGET
    cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --all --target $TARGET

    if [ $TRAVIS_PULL_REQUEST = "false" ]; then
        return
    fi

    cross test --all --target $TARGET --release

    cargo clippy --all -- -D warnings

    cross fmt --all -- --check
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
