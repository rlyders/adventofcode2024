#!/bin/bash

# build release version
cd app/cmd
go build -ldflags "-s -w" -o ./bin/aoc24
cd -
ls -latrh ./app/cmd/bin/aoc24

cd app/web
go build -ldflags "-s -w" -o ./bin/aoc24-web
cd -
ls -latrh ./app/web/bin/aoc24-web

# build debug version
cd app/cmd
go build -o ./bin/aoc24-dbg
cd -
ls -latrh ./app/cmd/bin/aoc24-dbg

cd app/web
go build  -o ./bin/aoc24-web-dbg
cd -
ls -latrh ./app/web/bin/aoc24-web-dbg
