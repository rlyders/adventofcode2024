package main

import (
	"github.com/rlyders/adventofcode/day1"
	"github.com/rlyders/adventofcode/utils"
)

var DEFAULT_DATA_PATHS = []string{"./data", "../data"}

func main() {
	utils.PrintSysInfo()

	utils.PrintMemStats("START: main")

	day1.Run(utils.ArgOrDefaultPath(1, DEFAULT_DATA_PATHS), utils.ArgOrDefaultNum(2, 1))

	utils.PrintMemStats_GC_Repeat("END  : main")
}
