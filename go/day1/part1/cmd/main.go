package cmd

import (
	"fmt"
	"os"

	engine "github.com/rlyders/adventofcode/day1/part1/engine"
	"github.com/rlyders/adventofcode/utils"
)

func Run(location_lists_path string, iterations int64) {
	_, err := os.Stat(location_lists_path)
	utils.Check(err, fmt.Sprintf("Stat(%s)", location_lists_path))

	lists, err := utils.LoadTextFile(location_lists_path)
	utils.Check(err, fmt.Sprintf("LoadTextFile(%s)", location_lists_path))

	results, err := engine.GetSimilarityScoreOfListsTextRepeated(lists, iterations)
	utils.Check(err, fmt.Sprintf("GetSumOfDistancesOfListsText(%s)", location_lists_path))

	utils.PrintTotal("Total Distance", results.Sum_of_distances, results.Elapseds)
}
