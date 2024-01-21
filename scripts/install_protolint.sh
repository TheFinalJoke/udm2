#!/bin/bash
set -eu pipefail

PROTOLINT_VERSION="0.47.5"

ARCH=$(uname -m)
if [[ $ARCH == "x86_64" ]] || [[ $ARCH == "amd64" ]]
then
    ARCH="amd64"
fi
COMPUTER_TYPE=$(uname -s | tr '[:upper:]' '[:lower:]')
FILENAME="protolint_${PROTOLINT_VERSION}_${COMPUTER_TYPE}_${ARCH}.tar.gz"
URL="https://github.com/yoheimuta/protolint/releases/download/v${PROTOLINT_VERSION}/$FILENAME"

echo "Using $URL"
wget -O /tmp/"$FILENAME" "$URL"

tar -zxvf /tmp/"$FILENAME" -C /usr/local/bin/

exit 0