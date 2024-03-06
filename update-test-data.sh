#!/bin/sh
set -e

DATADIR="opr-test-data/src/data"

if [ $# = 0 ]; then
    echo "Updating all data files"
    set -- "$DATADIR"/armies/* "$DATADIR"/common-rules-* "$DATADIR"/books/*
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
            URL="https://army-forge.onepagerules.com/api/rules/common"
            FILENAME=$(basename "$FILE")
            GSID="${FILENAME#common-rules-}"
            URL="${URL}/${GSID}"
            ;;
        "$DATADIR"/books/*)
            FILENAME=$(basename "$FILE")
            BOOKID="${FILENAME%-*}"
            GSID="${FILENAME#*-}"
            URL="https://army-forge.onepagerules.com/api/army-books/${BOOKID}?gameSystem=${GSID}"
            ;;
        *)
            die "unrecognized datafile '$FILE'"
            ;;
    esac
    printf "."
    curl --silent "$URL" | json_pp > "$FILE"
done

echo
