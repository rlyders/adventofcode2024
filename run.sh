#!/bin/bash
RUNTIME=$1

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

# run all tests
echo "------------- Run tests -------------------"

if ./test.sh ; then
    echo "tests: OK"
else
    echo "test: FAILED"
    exit 1
fi

echo "-------------------------------------------"

echo ""
echo "------------- Run buid --------------------"
if ./build.sh ; then
    echo "build: OK"
else
    echo "build: FAILED"
    exit 1
fi
echo "-------------------------------------------"

if [ -f "$prog" ]; then
    echo "Build exists: $prog"
else
    echo "Build does not exist: $prog"
    exit 1
fi

# display size of build
echo ""
echo "------------- Build Results ---------------"
if ls -latrh $prog ; then
    echo "Display build results: OK"
else
    echo "Display build results: FAILED"
    exit 1
fi
echo "-------------------------------------------"

echo ""
echo "------------- Run with timing -------------"
# run and show time of run
if time $prog ; then
    echo "run: OK"
else
    echo "run: FAILED"
    exit 1
fi
echo "-------------------------------------------"
