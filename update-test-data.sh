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
        "$DATADIR"/common-rules*) # maybe followed by a dash amd a query arg
            URL="https://army-forge.onepagerules.com/api/afs/common-rules"
            FILENAME=$(basename "$FILE")
            QUERYARG="${FILENAME#common-rules}"
            QUERYARG="${QUERYARG#-}" # none when no query arg
            if [ -n "$QUERYARG" ]; then
                URL="${URL}?description=${QUERYARG}"
            fi
            ;;
        *)
            echo >&2 "ERROR: unrecognized datafile '$FILE'"
            ;;
    esac
    printf "."
    curl --silent "$URL" | json_pp > "$FILE"
done

echo
