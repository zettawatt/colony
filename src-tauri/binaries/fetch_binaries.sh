#!/bin/bash

# fetch the latest dweb binaries from github

# check if the binaries already exist, if so exit
if [ -f "dweb-x86_64-unknown-linux-gnu" ] && [ -f "dweb-x86_64-pc-windows-msvc.exe" ] && [ -f "dweb-aarch64-apple-darwin" ]; then
  echo "Binaries already exist, exiting"
  exit 0
fi

# remove all existing dweb binaries
rm -f dweb-*

# get the latest tag from github
latest_tag=$(curl -s https://api.github.com/repos/happybeing/dweb/releases/latest | jq -r .tag_name)

# download the binaries
curl -L -o dweb-x86_64-unknown-linux-gnu https://github.com/happybeing/dweb/releases/download/$latest_tag/dweb-linux-amd64
curl -L -o dweb-x86_64-pc-windows-msvc.exe https://github.com/happybeing/dweb/releases/download/$latest_tag/dweb-amd64.exe
curl -L -o dweb-aarch64-apple-darwin https://github.com/happybeing/dweb/releases/download/$latest_tag/dweb-darwin-arm64
#curl -L -o dweb-x86_64-apple-darwin https://github.com/happybeing/dweb/releases/download/$latest_tag/dweb-darwin-

# make the binaries executable
chmod +x dweb-*
