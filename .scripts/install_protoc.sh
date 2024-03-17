#!/bin/bash

shopt -s extglob
ARCH=$(arch)
ARCH=${ARCH/+(amd64|aarch64)/aarch_64}
shopt -u extglob

URL=$(curl https://api.github.com/repos/protocolbuffers/protobuf/releases/latest | jq -r '.assets[] | select(.name | match("protoc-.+-linux-'${ARCH}'\\.zip")) | .browser_download_url')
wget -O /tmp/protoc.zip ${URL}
unzip /tmp/protoc.zip -x readme.txt -d /usr
rm /tmp/protoc.zip
