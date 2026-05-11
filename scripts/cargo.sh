#!/bin/bash


# Cargo déjà disponible dans le PATH courant (ou dans /usr/bin)
if command -v cargo >/dev/null 2>&1 || [ -x /usr/bin/cargo ]; then
    CARGO_BIN=$(command -v cargo 2>/dev/null || echo "/usr/bin/cargo")
    echo "Using existing cargo: $CARGO_BIN"
    # S'assurer que cargo est bien dans le PATH
    export PATH="$(dirname "$CARGO_BIN"):$PATH"
    return 0 2>/dev/null || exit 0
fi


# Cargo dans /tmp
TMP_BASE="/tmp"
export RUSTUP_HOME="$TMP_BASE/.rustup"
export CARGO_HOME="$TMP_BASE/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"


if command -v cargo >/dev/null 2>&1; then
    echo "Using /tmp cargo: $(command -v cargo)"
    return 0 2>/dev/null || exit 0
fi

# Installation dans /tmp
echo "Installing cargo in $CARGO_HOME ..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path

# Vérification
if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: cargo installation failed."
    return 1 2>/dev/null || exit 1
fi

echo "Cargo ready: $(command -v cargo)"
cargo --version
return 0 2>/dev/null || exit 0
