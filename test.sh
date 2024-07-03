cargo build --release

echo
echo "avalie se ha falhas"
read

clear

rm -r ./std
cp -r ./target/release/std/ ./std/
clear

proto bootstrap/main.ply

read