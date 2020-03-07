#!/bin/bash

sudo RUST_BACKTRACE=1 $HOME/.cargo/bin/cargo run -- \
  --socket tcp://127.0.0.1:5000;