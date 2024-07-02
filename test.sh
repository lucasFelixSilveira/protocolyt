cargo build --release

echo
echo "avalie se ha falhas"
read

clear

rm -r ./std
mkdir std
mkdir std/io
cp ./target/release/std/*.ts ./std/
cp ./target/release/std/*/*.ts ./std/*/
clear

proto bootstrap/main.ply

read