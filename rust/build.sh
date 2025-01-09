#!/bin/bash

# build release version
cargo build --bin aoc24 --release
cargo build --bin aoc24-web --release
