package cmd

import (
	"fmt"

	engine "github.com/rlyders/adventofcode/day1/part1/engine"
	"github.com/rlyders/adventofcode/utils"
)

const DATA_FILE_PATH = "../data/day1/lists.txt"

func Run() {
	lists := engine.LoadLists(DATA_FILE_PATH)
	results, err := engine.GetSumOfDistancesOfListsText(lists)
	utils.Check(err)
	fmt.Printf("distance: %d\n", results.Sum_of_distances)
}
