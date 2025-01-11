package engine

import (
	"fmt"
	"slices"
	"time"

	"github.com/rlyders/adventofcode/utils"
	"golang.org/x/text/language"
	"golang.org/x/text/message"
)

type LocationPairs struct {
	Location_columns string
	Sum_of_distances uint32
}

type SumOfDistancesResults struct {
	Sorted_location_pairs []LocationPair
	Sum_of_distances      uint32
	Elapseds              []utils.NamedElapsed
}

type LocationPair struct {
	Location_a uint32
	Location_b uint32
	Distance   uint32
}

var englishPrinter = message.NewPrinter(language.English)

func GetSimilarityScoreOfListsTextRepeated(lists string, iterations int64) (SumOfDistancesResults, error) {
	if iterations > 1 {
		fmt.Println(englishPrinter.Sprintf("Iterations: %v ... all timings shown below are averages", iterations))
	}
	var results SumOfDistancesResults
	var err error
	var splitElapsed int64
	var sort1Elapsed int64
	var sort2Elapsed int64
	var splitSortElapsed int64
	var calculateElapsed int64
	var totalElapsed int64
	for range iterations {
		results, err = GetSumOfDistancesOfListsText(lists)
		utils.Check(err, "GetSumOfDistancesOfListsText")
		for _, d := range results.Elapseds {
			switch d.Name {
			case "split":
				splitElapsed += d.Elapsed.Nanoseconds()
			case "sort1":
				sort1Elapsed += d.Elapsed.Nanoseconds()
			case "sort2":
				sort2Elapsed += d.Elapsed.Nanoseconds()
			case "split and sort":
				splitSortElapsed += d.Elapsed.Nanoseconds()
			case "calculate distance":
				calculateElapsed += d.Elapsed.Nanoseconds()
			case "total":
				totalElapsed += d.Elapsed.Nanoseconds()
			}
		}
	}
	for i, d := range results.Elapseds {
		switch d.Name {
		case "split":
			results.Elapseds[i].Elapsed = time.Duration(splitElapsed / iterations)
		case "sort1":
			results.Elapseds[i].Elapsed = time.Duration(sort1Elapsed / iterations)
		case "sort2":
			results.Elapseds[i].Elapsed = time.Duration(sort2Elapsed / iterations)
		case "split and sort":
			results.Elapseds[i].Elapsed = time.Duration(splitSortElapsed / iterations)
		case "calculate distance":
			results.Elapseds[i].Elapsed = time.Duration(calculateElapsed / iterations)
		case "total":
			results.Elapseds[i].Elapsed = time.Duration(totalElapsed / iterations)
		}
	}
	return results, nil // FIXME all
}

func GetSumOfDistancesOfListsText(lists string) (SumOfDistancesResults, error) {
	start := time.Now()

	list1, list2, elapsedSplit, elapsedSort1, elapsedSort2, err := utils.SplitAndSortLists(lists)
	utils.Check(err, "SplitAndSortLists")
	elapsedSplitAndSort := time.Since(start)

	results, err := GetSumOfDistances(list1, list2)
	utils.Check(err, "GetSumOfDistances")

	results.Elapseds = slices.Insert(results.Elapseds, 0, utils.NamedElapsed{Name: "split and sort", Elapsed: elapsedSplitAndSort})
	results.Elapseds = slices.Insert(results.Elapseds, 0, utils.NamedElapsed{Name: "sort2", Elapsed: elapsedSort2})
	results.Elapseds = slices.Insert(results.Elapseds, 0, utils.NamedElapsed{Name: "sort1", Elapsed: elapsedSort1})
	results.Elapseds = slices.Insert(results.Elapseds, 0, utils.NamedElapsed{Name: "split", Elapsed: elapsedSplit})
	results.Elapseds = append(results.Elapseds, utils.NamedElapsed{Name: "total", Elapsed: time.Since(start)})

	return results, nil
}

func GetSumOfDistances(list1 []uint32, list2 []uint32) (SumOfDistancesResults, error) {
	start := time.Now()

	var results SumOfDistancesResults
	// performance optimization: pre-alloc memory for slice of LocationPairs to avoid repeated resizing of the underlying array for each append
	results.Sorted_location_pairs = make([]LocationPair, len(list1))
	for i := range list1 {
		int1 := list1[i]
		int2 := list2[i]
		var distance uint32
		if int1 > int2 {
			distance = int1 - int2
		} else {
			distance = int2 - int1
		}

		locationPair := LocationPair{
			Location_a: list1[i],
			Location_b: list2[i],
			Distance:   distance,
		}
		results.Sorted_location_pairs[i] = locationPair
		results.Sum_of_distances = results.Sum_of_distances + locationPair.Distance
	}
	results.Elapseds = append(results.Elapseds, utils.NamedElapsed{Name: "calculate distance", Elapsed: time.Since(start)})
	return results, nil
}
