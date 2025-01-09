package utils

import (
	"bufio"
	"errors"
	"fmt"
	"html/template"
	"log"
	"os"
	"path/filepath"
	"runtime"
	"sort"
	"strconv"
	"strings"
	"time"

	"golang.org/x/text/language"
	"golang.org/x/text/message"
	"gopkg.in/yaml.v3"
)

var aoc AdventOfCode

type AdventOfCode struct {
	Days []Day `yaml:"days"`
}

type Day struct {
	Title string `yaml:"title"`
	Parts []Part `yaml:"parts"`
}

type Part struct {
	Name    string `yaml:"name"`
	Summary string `yaml:"summary"`
}

type NamedElapsed struct {
	Name    string
	Elapsed time.Duration
}

func init() {
	_, filename, _, _ := runtime.Caller(0)
	currentDir := filepath.Dir(filename)

	yaml_path := filepath.Join(currentDir, "..", "..", "data", "advent_of_code_2024.yaml")
	aoc.Load(yaml_path)
}

func (y *AdventOfCode) Load(yaml_file string) *AdventOfCode {
	yamlFile, err := os.ReadFile(yaml_file)
	if err != nil {
		log.Printf("yamlFile.Get err   #%v ", err)
	}
	err = yaml.Unmarshal(yamlFile, y)
	if err != nil {
		log.Fatalf("Unmarshal: %v", err)
	}

	return y
}

func GetConfig() *AdventOfCode {
	return &aoc
}

func OutputCwd() {
	cwd, err := os.Getwd()
	if err != nil {
		log.Println("Failed to get current working directory:", err)
	} else {
		log.Println("Current working directory:", cwd)
	}
}

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func Check(e error, msg string) {
	if e != nil {
		if len(msg) > 0 {
			panic(fmt.Sprintf("%v: %s", e, msg))
		} else {
			panic(e)
		}
	}
}

func GetInt(str string) uint32 {
	i, err := strconv.Atoi(str)
	Check(err, str)
	return uint32(i)
}

var englishPrinter = message.NewPrinter(language.English)

func GetFormatUInt32FuncMap() template.FuncMap {
	return template.FuncMap{
		"FormatUInt32": FormatUInt32,
	}
}

func GetFormatElapsedMillisecondsFuncMap() template.FuncMap {
	return template.FuncMap{
		"FormatElapsedMilliseconds": FormatElapsedMilliseconds,
	}
}

func GetFormatElapsedMicrosecondsFuncMap() template.FuncMap {
	return template.FuncMap{
		"FormatElapsedMicroseconds": FormatElapsedMicroseconds,
	}
}

func GetFormatElapsedNanosecondsFuncMap() template.FuncMap {
	return template.FuncMap{
		"FormatElapsedNanoseconds": FormatElapsedNanoseconds,
	}
}

func GetFormatInt64FuncMap() template.FuncMap {
	return template.FuncMap{
		"FormatInt64": FormatInt64,
	}
}

func FormatUInt32(n uint32) string {
	return englishPrinter.Sprintf("%v", n)
}

func FormatInt64(n int64) string {
	return englishPrinter.Sprintf("%v", n)
}

func FormatElapsedMilliseconds(elapsed time.Duration) string {
	return englishPrinter.Sprintf("%v", elapsed.Milliseconds())
}

func FormatElapsedMicroseconds(elapsed time.Duration) string {
	return englishPrinter.Sprintf("%v", elapsed.Microseconds())
}

func FormatElapsedNanoseconds(elapsed time.Duration) string {
	return englishPrinter.Sprintf("%v", elapsed.Nanoseconds())
}

func PrintTotal(title string, total uint32, elapseds []NamedElapsed) {
	fmt.Println("------------------------------------------------------------------")
	fmt.Printf("%25s: %10s [raw: %10d]\n", title, FormatUInt32(total), total)
	fmt.Println("-- Timings: --")
	for _, e := range elapseds {
		fmt.Printf("     %20s: %2s ms [ %6s μs;  %10s ns]\n",
			e.Name,
			FormatElapsedMilliseconds(e.Elapsed),
			FormatElapsedMicroseconds(e.Elapsed),
			FormatElapsedNanoseconds(e.Elapsed))
	}
	fmt.Println("------------------------------------------------------------------")
}

func SplitLists(lists string) ([]uint32, []uint32, error) {
	var list1 []uint32
	var list2 []uint32

	scanner := bufio.NewScanner(strings.NewReader(lists))
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			continue
		}
		words := strings.Fields(line)

		list1 = append(list1, GetInt(words[0]))
		list2 = append(list2, GetInt(words[1]))
	}
	if err := scanner.Err(); err != nil {
		fmt.Printf("error occurred: %v\n", err)
		return nil, nil, err
	}

	return list1, list2, nil
}

func SplitAndSortLists(lists string) ([]uint32, []uint32, time.Duration, time.Duration, time.Duration, error) {
	start := time.Now()
	list1, list2, err := SplitLists(lists)
	Check(err, "SplitLists")
	elapsedSplit := time.Since(start)

	start = time.Now()
	sort.Slice(list1, func(i, j int) bool {
		return list1[i] < list1[j]
	})
	elapsedSort1 := time.Since(start)

	start = time.Now()
	sort.Slice(list2, func(i, j int) bool {
		return list2[i] < list2[j]
	})
	elapsedSort2 := time.Since(start)

	return list1, list2, elapsedSplit, elapsedSort1, elapsedSort2, nil
}

func ArgOrDefaultPath(argNum int, default_paths []string) string {
	var paths []string

	args := os.Args[1:] // ignore first arg which is usually the program name (i.e. something we don't need here)
	if len(args) < argNum || len(strings.Trim(args[argNum-1], " ")) == 0 {
		// if no command-line argument was given, then use the default paths
		paths = default_paths
	} else {
		// if a data file path was given via a command-line argument, then use that only...and ignore default paths
		paths = append(paths, args[argNum-1])
	}

	for _, p := range paths {
		if len(strings.Trim(p, " ")) == 0 {
			return p
		}
		_, err := os.Stat(p)
		if err != nil {
			continue
		}
		return p
	}
	panic(fmt.Sprintf("failed to find valid data file from list: %s", strings.Join(paths, ",")))
}

func ArgOrDefaultNum(argNum int, default_num int64) int64 {
	args := os.Args[1:] // ignore first arg which is usually the program name (i.e. something we don't need here)
	if len(args) < argNum || len(strings.Trim(args[argNum-1], " ")) == 0 {
		return default_num
	}
	num, err := strconv.ParseInt(args[argNum-1], 10, 64)
	if err != nil {
		panic(err)
	}
	return num
}

func LoadTextFile(file_path string) (string, error) {
	if _, err := os.Stat(file_path); errors.Is(err, os.ErrNotExist) {
		return "", err
	}
	bytes, err := os.ReadFile(file_path)
	Check(err, fmt.Sprintf("ReadFile(%s)", file_path))

	lists := string(bytes)
	return lists, nil
}
