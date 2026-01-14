#!/bin/bash

RUST_BACKTRACE=full RUST_BACKTRACE=1 target/x86_64-unknown-linux-musl/release/rustpl -v --template ./samples/test.tpl --render test --values samples/data.json
