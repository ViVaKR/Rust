#! /usr/bin/env zsh

cargo build --verbose &>build.log
cargo run --quiet -p viv $1
