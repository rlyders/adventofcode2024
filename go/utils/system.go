package utils

// #include <unistd.h>
import "C"

import (
	"fmt"
	"os"
	"os/exec"
	"runtime"
	"strconv"
	"strings"

	"github.com/mackerelio/go-osstat/memory"
)

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
	fmt.Printf("memory cached: %d MB\n", memory.Cached/(1024*1024))
	fmt.Printf("memory free: %d MB\n", memory.Free/(1024*1024))
}

func PrintPages() {
	bytes := C.sysconf(C._SC_PHYS_PAGES) * C.sysconf(C._SC_PAGE_SIZE)
	KB := bytes / 1024
	MB := KB / 1024
	GB := MB / 1024
	println("------------------ from:  C.sysconf")
	println(bytes, " bytes")
	println(KB, " KG")
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
