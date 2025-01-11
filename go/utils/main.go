package utils

import (
	"bufio"
	"errors"
	"fmt"
	"html/template"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"slices"
	"strconv"
	"strings"
	"syscall"
	"time"

	"github.com/mackerelio/go-osstat/memory"
	"golang.org/x/text/language"
	"golang.org/x/text/message"
	"gopkg.in/yaml.v3"
)

var englishPrinter = message.NewPrinter(language.English)

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
		// fmt.Printf("e.Elapsed=%v", e.Elapsed)
		fmt.Printf("     %20s: %2s ms [ %6s μs;  %10s ns]\n",
			e.Name,
			FormatElapsedMilliseconds(e.Elapsed),
			FormatElapsedMicroseconds(e.Elapsed),
			FormatElapsedNanoseconds(e.Elapsed))
	}
	fmt.Println("------------------------------------------------------------------")
}

func SplitLists(lists string) ([]uint32, []uint32, error) {
	// performance optimization: pre-alloc memory for slices of uint32 to avoid repeated resizing of the underlying array for each append
	list1 := make([]uint32, 0, 1000)
	list2 := make([]uint32, 0, 1000)

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
	slices.Sort(list1)
	elapsedSort1 := time.Since(start)

	start = time.Now()
	slices.Sort(list2)
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

func PrintCpuMemory() {
	// Get CPU usage
	cmd := exec.Command("top", "-bn1", "-n1")
	output, err := cmd.Output()
	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	lines := strings.Split(string(output), "\n")
	cpuLine := lines[2] // Line containing CPU usage
	cpuFields := strings.Fields(cpuLine)
	cpuUsage, _ := strconv.ParseFloat(cpuFields[2], 64)

	// Get memory usage
	cmd = exec.Command("free", "-m")
	output, err = cmd.Output()
	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	lines = strings.Split(string(output), "\n")
	memLine := lines[1] // Line containing memory usage
	memFields := strings.Fields(memLine)
	totalMem, _ := strconv.ParseInt(memFields[1], 10, 64)
	usedMem, _ := strconv.ParseInt(memFields[2], 10, 64)

	fmt.Printf("CPU Usage: %.2f%%\n", cpuUsage)
	fmt.Printf("Memory Usage: %dMB / %dMB\n", usedMem, totalMem)
}

func PrintMem() {
	memory, err := memory.Get()
	if err != nil {
		fmt.Fprintf(os.Stderr, "%s\n", err)
		return
	}
	fmt.Printf("memory total: %d MB\n", memory.Total/(1024*1024))
	fmt.Printf("memory used: %d MB\n", memory.Used/(1024*1024))
	fmt.Printf("memory free: %d MB\n", memory.Free/(1024*1024))
}

func PrintPages() {
	pageSize := syscall.Getpagesize()
	bytes := uint64(pageSize)
	KB := bytes / 1024
	MB := KB / 1024
	GB := MB / 1024
	println("------------------ from: syscall")
	println(bytes, " bytes")
	println(KB, " KB")
	println(MB, " MB")
	println(GB, " GB")
	println("------------------")
}

const (
	B  = "B"
	KB = "KB"
	MB = "MB"
	GB = "GB"
	TB = "TB"
)

func valid_mem_units(memory_units string) bool {
	switch memory_units {
	case
		KB, MB:
		return true
	}
	return false
}

func RunGC() {
	runtime.GC()
}

func PrintMemStats_GC_Repeat(msg string) error {
	err := PrintMemStats(msg)
	Check(err, "failed to print memory stats")

	RunGC()
	err = PrintMemStats("POST : GarbageCol")
	Check(err, "failed to print memory stats")
	return nil
}

func PrintMemStats(msg string) error {

	var m runtime.MemStats
	runtime.ReadMemStats(&m)
	fmt.Printf("# [PID %-6d] %-18s: RAM: %8s of avail: %8s [GC cummulative: %8s, cycles: %4v]\n", os.Getpid(), msg, smartRamUnits(m.Alloc), smartRamUnits(m.Sys), smartRamUnits(m.TotalAlloc), m.NumGC)

	return nil
}

var ram_units = [...]string{B, KB, MB, GB, TB}

func smartRamUnits(b uint64) string {
	units_idx := 0 // index to ram_units array
	units := float64(b)
	for units > 1024 && units_idx < 4 {
		units /= 1024
		units_idx += 1
	}
	s := fmt.Sprintf("%.1f %s", units, ram_units[units_idx])
	return s
}

func bToGb(b uint64) uint64 {
	return b / 1024 / 1024 / 1024
}

func bToMb(b uint64) uint64 {
	return b / 1024 / 1024
}

func bToKB(b uint64) uint64 {
	return b / 1024
}
