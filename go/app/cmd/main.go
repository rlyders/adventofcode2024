package main

import (
	day1 "github.com/rlyders/adventofcode/day1"
	"github.com/rlyders/adventofcode/utils"
)

var DEFAULT_DATA_PATHS = []string{"./data", "../data"}

func main() {
	day1.Run(utils.ArgOrDefaultPath(1, DEFAULT_DATA_PATHS), utils.ArgOrDefaultNum(2, 1))
}
