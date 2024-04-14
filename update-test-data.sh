#!/bin/sh
set -e

DATADIR="opr-test-data/src/data"

if [ $# = 0 ]; then
    echo "Updating all data files"
    set -- "$DATADIR"/armies/*
fi

die() {
    echo >&2 "ERROR: $*"
    exit 1
}

for FILE in "$@"; do
    case "$FILE" in
        "$DATADIR"/armies/*)
            ID=$(basename "$FILE")
            URL="https://army-forge.onepagerules.com/api/tts?id=$ID"
            ;;
        "$DATADIR"/common-rules-*) # maybe followed by a dash amd a query arg
            URL="https://army-forge.onepagerules.com/api/afs/common-rules"
            FILENAME=$(basename "$FILE")
            QUERYARG="${FILENAME#common-rules-}"
            if [ -n "$QUERYARG" ]; then
                URL="${URL}?gameSystem=${QUERYARG}"
            fi
            ;;
        *)
            die "unrecognized datafile '$FILE'"
            ;;
    esac
    printf "."
    curl --silent "$URL" | json_pp > "$FILE"
done

echo
