package day1

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"sort"
	"strconv"
	"strings"
)

func Day1() (int, error) {
	fmt.Println("Advent of code 2024 Day 1: go")
	lists := loadLists("lists.txt")
	return getSumOfDistancesOfListsText(lists)
}

func getSumOfDistancesOfListsText(lists string) (int, error) {
	list1, list2, err := splitAndSortLists(lists)
	check(err)
	return getSumOfDistances(list1, list2)
}

func splitAndSortLists(lists string) ([]int, []int, error) {
	var list1 []int
	var list2 []int

	scanner := bufio.NewScanner(strings.NewReader(lists))
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			continue
		}
		words := strings.Fields(line)

		list1 = append(list1, getInt(words[0]))
		list2 = append(list2, getInt(words[1]))
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

func getSumOfDistances(list1 []int, list2 []int) (int, error) {

	distance := 0
	for i := range list1 {
		d := abs(list1[i] - list2[i])
		distance = distance + d
	}
	fmt.Printf("distance: %d\n", distance)
	return distance, nil
}

func loadLists(listsFileName string) string {
	_, filename, _, _ := runtime.Caller(0)
	pkgDir := filepath.Dir(filename)
	bytes, err := os.ReadFile(filepath.Join(pkgDir, "data", listsFileName))
	check(err)

	lists := string(bytes)
	return lists
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func getInt(str string) int {
	i, err := strconv.Atoi(str)
	check(err)
	return i
}
