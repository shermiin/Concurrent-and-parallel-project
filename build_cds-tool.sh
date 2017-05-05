#!/bin/bash

set -e

# build the cds-tool installing rust if necessary

type cargo >/dev/null 2>&1 || {
    echo >&2 "I require rust but it's not installed. Invoking Installer."
    curl https://sh.rustup.rs -sSf | sh
    source ~/.cargo/env
}

pushd cds-tool
cargo build --release
cp target/release/cds-tool bin
cargo clean
popd
