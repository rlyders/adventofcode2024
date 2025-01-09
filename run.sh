#!/bin/bash
RUNTIME=$1
DATA_FILE_PATH="${2-../data}"

if [[ $RUNTIME == "go" || $RUNTIME == "rust" ]]; then
    echo "Valid runtime: $RUNTIME"
else
    echo "missing required runtime parameter: 'go' or 'rust'"
    exit 1
fi

if [ -d "$RUNTIME" ]; then
    echo "Directory exists: $RUNTIME"
else
    echo "Directory doesn't exist: $RUNTIME"
    exit 1
fi

cd "$RUNTIME"

set -o allexport
source .env
set +o allexport

echo prog=$prog

echo ""
echo "------------- Run with timing -------------"
# run and show time of run

if time ./aoc24.sh "${@:2}"; then
    echo "run: OK"
else
    echo "run: FAILED"
    exit 1
fi
echo "-------------------------------------------"
