#!/bin/sh
set -e

DATADIR="src/test-data"

for FILE in "$DATADIR"/????????????; do
    ID=$(basename "$FILE")
    echo "$ID ..."
    curl --silent "https://army-forge.onepagerules.com/api/tts?id=$ID" |
        json_pp > "$DATADIR/$ID"
done
