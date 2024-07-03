#!/bin/bash
cargo build --release
cp -r std target/release/std
clear


export PATH="$PATH:$(pwd)/protocolyt/target/release"

echo "O diretório atual foi adicionado ao PATH:"
echo $PATH