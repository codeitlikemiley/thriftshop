#!/bin/sh

cargo clean
rm arithmetic-parser.pkg
cargo build --release
cargo bundle --release
pkgbuild --root ./target/release/bundle/osx/Thriftshop.app --install-location "/Applications/Thriftshop.app" --identifier com.codeitlikemiley.thriftshop --version 1.0.0 --scripts ./scripts thriftshop.pkg
