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

func Day1() {
	fmt.Println("Advent of code 2024 Day 1")

	_, filename, _, _ := runtime.Caller(0)
	pkgDir := filepath.Dir(filename)
	bytes, err := os.ReadFile(filepath.Join(pkgDir, "data", "lists.txt"))
	check(err)

	lists := string(bytes)
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
		return
	}

	sort.Slice(list1, func(i, j int) bool {
		return list1[i] < list1[j]
	})

	sort.Slice(list2, func(i, j int) bool {
		return list2[i] < list2[j]
	})

	distance := 0
	for i := range list1 {
		d := abs(list1[i] - list2[i])
		distance = distance + d
	}
	fmt.Printf("distance: %d\n", distance)
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func getChar(str string, index int) rune {
	return []rune(str)[index]
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
