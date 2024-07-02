#!/bin/bash
git clone https://github.com/lucasFelixSilveira/protocolyt.git
cd protocolyt
cargo build --release
cd ..

export PATH="$PATH:$(pwd)/protocolyt/target/release"

echo "O diretório atual foi adicionado ao PATH:"
echo $PATH
