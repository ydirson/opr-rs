#!/bin/sh
set -e

DATADIR="opr-test-data/src/data"

if [ $# = 0 ]; then
    echo "Updating all data files"
    set -- "$DATADIR"/armies/*
fi

for FILE in "$@"; do
    case "$FILE" in
        "$DATADIR"/armies/*)
            ID=$(basename "$FILE")
            URL="https://army-forge.onepagerules.com/api/tts?id=$ID"
            ;;
        "$DATADIR"/common-rules)
            URL="https://army-forge.onepagerules.com/api/afs/common-rules"
            ;;
        *)
            echo >&2 "ERROR: unrecognized datafile '$FILE'"
            ;;
    esac
    printf "."
    curl --silent "$URL" | json_pp > "$FILE"
done

echo
