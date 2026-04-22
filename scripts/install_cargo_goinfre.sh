#!/bin/bash

export RUSTUP_HOME=/home/$USER/goinfre/.rustup
export CARGO_HOME=/home/$USER/goinfre/.cargo

if [ -f "$CARGO_HOME/bin/cargo" ]; then
    echo "Cargo already installed in $CARGO_HOME. Skipping installation."
    return 0
fi

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path

