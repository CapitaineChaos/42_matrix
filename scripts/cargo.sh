#!/bin/bash


# Cargo déjà disponible dans le PATH courant
if command -v cargo >/dev/null 2>&1; then
    echo "Using existing cargo: $(command -v cargo)"
    return 0 2>/dev/null || exit 0
fi


# Cargo dans goinfre
GOINFRE_BASE="/home/$USER/goinfre"
export RUSTUP_HOME="$GOINFRE_BASE/.rustup"
export CARGO_HOME="$GOINFRE_BASE/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"


if command -v cargo >/dev/null 2>&1; then
    echo "Using goinfre cargo: $(command -v cargo)"
    return 0 2>/dev/null || exit 0
fi

# Si goinfre n'existe pas, on ne peut pas installer là-bas
if [ ! -d "$GOINFRE_BASE" ]; then
    echo "Error: cargo not found in PATH, and $GOINFRE_BASE does not exist."
    return 1 2>/dev/null || exit 1
fi

# Installation dans goinfre
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
