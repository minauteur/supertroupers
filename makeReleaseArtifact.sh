#!/bin/bash
cargo build --release
if [ "$?" != "0"  ]; then
  echo "cargo build failed"
  exit 1
fi
os=""
version="$(git describe --tags)"
rm -rf ./supertroupers
echo "deleted old supertroupers folder"
mkdir ./supertroupers
echo "created new supertroupers folder"
if [ -f "./target/release/supertroupers" ]; then
    echo "found linux binary"
    cp -p target/release/supertroupers ./supertroupers/supertroupers
    os="MACOS"
elif [ -f "./target/release/supertroupers.exe" ]; then
    echo "found windows binary"
    cp -p target/release/supertroupers.exe ./supertroupers/supertroupers.exe
    os="WINDOWS"
else
    echo "no binary found"
    exit 1
fi
echo "copying assets..."
cp -p names.json ./supertroupers/names.json
cp -p title.json ./supertroupers/title.json
cp -p poems.txt ./supertroupers/poems.txt
echo "zipping... supertroupers_v${version}_${os}.zip"
zip supertroupers_v"$version"_"$os".zip ./supertroupers/*