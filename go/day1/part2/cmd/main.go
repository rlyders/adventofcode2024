package cmd

import (
	"fmt"
	"os"

	engine "github.com/rlyders/adventofcode/day1/part2/engine"
	"github.com/rlyders/adventofcode/utils"
)

func Run(location_lists_path string, iterations int64) {
	utils.PrintMemStats("START: Day1 Part2")
	_, err := os.Stat(location_lists_path)
	utils.Check(err, fmt.Sprintf("Stat(%s)", location_lists_path))

	lists, err := utils.LoadTextFile(location_lists_path)
	utils.Check(err, fmt.Sprintf("LoadTextFile(%s)", location_lists_path))

	results, err := engine.GetSimilarityScoreOfListsTextRepeated(lists, iterations)
	utils.Check(err, "GetSimilarityScoreOfListsText")

	utils.PrintTotal("Total Similarity", results.Total_similarity_score, results.Elapseds)
	utils.PrintMemStats("END  : Day1 Part2")
}
