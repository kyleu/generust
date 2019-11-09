# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    local binary="$CRATE_NAME"
    case $TARGET in
        x86_64-pc-windows-gnu)
            binary="$CRATE_NAME.exe"
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    cross rustc --bin {{project-name}} --target $TARGET --release -- -C lto

    cp target/$TARGET/release/$binary $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
