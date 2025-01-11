package utils

import (
	"fmt"
	"log"
	"math"
	"os"
	"runtime"
	"strconv"
	"time"

	"github.com/shirou/gopsutil/cpu"
	"github.com/shirou/gopsutil/host"
	"github.com/shirou/gopsutil/mem"
)

// SysInfo saves the basic system information
type SysInfo struct {
	// Hostname string `bson:hostname`
	Platform string `bson:platform`
	CPU      string `bson:cpu`
	RAM      uint64 `bson:ram`
	// Disk     uint64 `bson:disk`
}

func getCpuUsage() int {
	percent, err := cpu.Percent(time.Second, false)
	if err != nil {
		log.Fatal(err)
	}
	return int(math.Ceil(percent[0]))
}

func getMemoryUsedPct() float64 {
	memory, err := mem.VirtualMemory()
	if err != nil {
		log.Fatal(err)
	}
	return memory.UsedPercent
}

func getMemoryUsed() uint64 {
	memory, err := mem.VirtualMemory()
	if err != nil {
		log.Fatal(err)
	}
	return memory.Used
}

func getMemoryFree() uint64 {
	memory, err := mem.VirtualMemory()
	if err != nil {
		log.Fatal(err)
	}
	return memory.Free
}

func PrintSysInfo() {
	info := new(SysInfo)

	hostStat, err := host.Info()
	if err == nil {
		info.Platform = hostStat.Platform
	}
	cpuStat, err := cpu.Info()
	if err == nil {
		info.CPU = cpuStat[0].ModelName
	}
	vmStat, _ := mem.VirtualMemory()
	if err == nil {
		info.RAM = vmStat.Total / 1024 / 1024
	}
	// diskStat, _ := disk.Usage("\\") // If you're in Unix change this "\\" for "/"

	// info.Hostname = hostStat.Hostname
	// info.Disk = diskStat.Total / 1024 / 1024

	fmt.Printf("%s: %s %s %s CPU %d %%, %s of %s RAM (free: %s)\n", runtime.GOOS, info.Platform, runtime.GOARCH, info.CPU, getCpuUsage(), smartRamUnits(uint64(getMemoryUsed())), smartRamUnits(vmStat.Total), smartRamUnits(getMemoryFree()))
}

func errHandler(err error) {
	if err != nil {
		log.Println(err.Error())
		os.Exit(1)
	}
}

func print_system_details() {
	runtimeOS := runtime.GOOS
	runtimeARCH := runtime.GOARCH
	fmt.Println("OS: ", runtimeOS)
	fmt.Println("Architecture: ", runtimeARCH)
	vmStat, err := mem.VirtualMemory()
	errHandler(err)
	fmt.Println("Total memory: ", strconv.FormatUint(vmStat.Total/(1024*1024), 10)+" MB")
	fmt.Println("Free memory: ", strconv.FormatUint(vmStat.Free/(1024*1024), 10)+" MB")

	// Cached and swap memory are ignored. Should be considered to get the understanding of the used %
	fmt.Println("Percentage used memory: ", strconv.FormatFloat(vmStat.UsedPercent, 'f', 2, 64)+"%")
}
