package day1

import (
	"fmt"
	"path/filepath"

	day1_part1 "github.com/rlyders/adventofcode/day1/part1/cmd"
	day1_part2 "github.com/rlyders/adventofcode/day1/part2/cmd"
)

func Run(data_path string, iterations int64) {
	// fmt.Printf("data path: %s\n", data_path)
	location_lists_path := filepath.Join(data_path, "day1", "lists.txt")
	fmt.Printf("location lists file: %s\n", location_lists_path)

	day1_part1.Run(location_lists_path, iterations)
	day1_part2.Run(location_lists_path, iterations)
}
