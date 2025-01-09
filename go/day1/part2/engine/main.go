package engine

import (
	"fmt"
	"slices"
	"time"

	"github.com/rlyders/adventofcode/utils"
)

type SimilarityResults struct {
	Similarity_scores      []SimilarityScore
	Total_similarity_score uint32
	Elapseds               []utils.NamedElapsed
}

type SimilarityScore struct {
	Location_a       uint32
	Found_in_list_b  uint32
	Similarity_score uint32
}

type LocationPair struct {
	Location_a uint32
	Location_b uint32
	Distance   uint32
}

func GetSimilarityScoreOfListsTextRepeated(lists string, iterations int64) (SimilarityResults, error) {
	if iterations > 1 {
		fmt.Printf("Iterations: %d ... all timings shown below are averages\n", iterations)
	}
	var results SimilarityResults
	var err error
	var splitSortElapsed int64
	var calculateElapsed int64
	var totalElapsed int64
	for range iterations {
		results, err = GetSimilarityScoreOfListsText(lists)
		utils.Check(err, "GetSimilarityScoreOfListsText")
		for _, d := range results.Elapseds {
			switch d.Name {
			case "split and sort":
				splitSortElapsed += d.Elapsed.Nanoseconds()
			case "calculate similarity":
				calculateElapsed += d.Elapsed.Nanoseconds()
			case "total":
				totalElapsed += d.Elapsed.Nanoseconds()
			}
		}
	}
	for _, d := range results.Elapseds {
		switch d.Name {
		case "split and sort":
			d.Elapsed = time.Duration(splitSortElapsed / iterations)
		case "calculate similarity":
			d.Elapsed = time.Duration(calculateElapsed / iterations)
		case "total":
			d.Elapsed = time.Duration(totalElapsed / iterations)
		}
	}
	return results, nil
}

func GetSimilarityScoreOfListsText(lists string) (SimilarityResults, error) {
	start := time.Now()

	list1, list2, err := utils.SplitAndSortLists(lists)
	utils.Check(err, "SplitLists")
	elapsedSplitAndSort := time.Since(start)

	results, err := GetSimilarityScoreOfSortedLists(list1, list2)
	utils.Check(err, "GetSimilarityScore")

	elapsed := time.Since(start)
	results.Elapseds = slices.Insert(results.Elapseds, 0, utils.NamedElapsed{Name: "split and sort", Elapsed: elapsedSplitAndSort})
	results.Elapseds = append(results.Elapseds, utils.NamedElapsed{Name: "total", Elapsed: elapsed})

	return results, nil
}

func GetSimilarityScoreOfSortedLists(sorted_locations_a []uint32, sorted_locations_b []uint32) (SimilarityResults, error) {
	start := time.Now()

	var results SimilarityResults
	var b_idx uint32
	var last_a uint32
	var similarityScore *SimilarityScore
	for i, a := range sorted_locations_a {
		// if this `a` value is *not* the same as the last
		// *or* this is first iteration of this loop
		if a != last_a || i == 0 {
			last_a = a
			// find all locations in list B that match the location from list A
			similarityScore = countMatchingValuesInList(a, &b_idx, sorted_locations_b)
		}
		results.Similarity_scores = append(results.Similarity_scores, *similarityScore)
	}
	for _, s := range results.Similarity_scores {
		results.Total_similarity_score += s.Similarity_score
	}
	results.Elapseds = append(results.Elapseds, utils.NamedElapsed{Name: "calculate similarity", Elapsed: time.Since(start)})
	return results, nil
}

func countMatchingValuesInList(src_val uint32, sorted_list_start_idx *uint32, sorted_list []uint32) *SimilarityScore {
	similarityScore := SimilarityScore{
		Location_a:       src_val,
		Found_in_list_b:  0,
		Similarity_score: 0,
	}
	for ; *sorted_list_start_idx < uint32(len(sorted_list)); *sorted_list_start_idx++ {
		b := sorted_list[*sorted_list_start_idx]
		if src_val == b {
			similarityScore.Found_in_list_b += 1
		} else if b > src_val {
			break // the lists are both sorted, so if b is greater than a, then exit the loop since
			// there won't be any more matches as the values will only get greater from this index
		}
	}
	similarityScore.Similarity_score = src_val * similarityScore.Found_in_list_b
	return &similarityScore
}
