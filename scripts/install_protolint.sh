#!/bin/bash
set -eu pipefail

PROTOLINT_VERSION="0.47.4"

ARCH=$(uname -m)
if [[ $ARCH == "x86_64" ]] || [[ $ARCH == "amd64" ]]
then
    ARCH="amd64"
fi
FILENAME="protolint_"${PROTOLINT_VERSION}"_linux_"${ARCH}".tar.gz"
URL="https://github.com/yoheimuta/protolint/releases/download/v"${PROTOLINT_VERSION}"/$FILENAME"

echo "Using $URL"
wget -O /tmp/$FILENAME $URL

tar -zxvf /tmp/$FILENAME -C /usr/local/bin/

exit 0