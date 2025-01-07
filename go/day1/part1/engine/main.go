package engine

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strings"

	"github.com/rlyders/adventofcode/utils"
)

const DATA_FILE_PATH = "../data/day1/lists.txt"

type LocationPairs struct {
	Location_columns string
	Sum_of_distances uint32
}

type SumOfDistancesResults struct {
	Sorted_location_pairs []LocationPair
	Sum_of_distances      uint32
	Elapsed_ms            uint32
	Elapsed_μs            uint32
}

type LocationPair struct {
	Location_a uint32
	Location_b uint32
	Distance   uint32
}

func GetSumOfDistancesOfListsText(lists string) (SumOfDistancesResults, error) {
	list1, list2, err := SplitAndSortLists(lists)
	utils.Check(err)
	return GetSumOfDistances(list1, list2)
}

func SplitAndSortLists(lists string) ([]uint32, []uint32, error) {
	var list1 []uint32
	var list2 []uint32

	scanner := bufio.NewScanner(strings.NewReader(lists))
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			continue
		}
		words := strings.Fields(line)

		list1 = append(list1, utils.GetInt(words[0]))
		list2 = append(list2, utils.GetInt(words[1]))
	}
	if err := scanner.Err(); err != nil {
		fmt.Printf("error occurred: %v\n", err)
		return nil, nil, err
	}

	sort.Slice(list1, func(i, j int) bool {
		return list1[i] < list1[j]
	})

	sort.Slice(list2, func(i, j int) bool {
		return list2[i] < list2[j]
	})

	return list1, list2, nil
}

func GetSumOfDistances(list1 []uint32, list2 []uint32) (SumOfDistancesResults, error) {

	var results SumOfDistancesResults
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
		results.Sorted_location_pairs = append(results.Sorted_location_pairs, locationPair)
		results.Sum_of_distances = results.Sum_of_distances + locationPair.Distance
	}
	return results, nil
}

func LoadLists(file_path string) string {
	bytes, err := os.ReadFile(file_path)
	utils.Check(err)

	lists := string(bytes)
	return lists
}
