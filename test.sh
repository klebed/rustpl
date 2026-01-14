#!/bin/bash

RUST_BACKTRACE=full RUST_BACKTRACE=1 target/x86_64-unknown-linux-musl/release/rustpl -v --template ./samples/test.tpl --render test --set proto=udp --set server=server111 --set port=1111 --set tls.certificate_file=cert.pem --set tls_private_key_file=key.pem --set tls_private_key_password=
