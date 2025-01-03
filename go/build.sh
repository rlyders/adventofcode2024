#!/bin/bash

# build release version
go build -ldflags "-s -w" -o $prog
