package utils

import (
	"html/template"
	"log"
	"os"
	"strconv"

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

func init() {
	aoc.Load("../data/advent_of_code_2024.yaml")
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

func Check(e error) {
	if e != nil {
		panic(e)
	}
}

func GetInt(str string) uint32 {
	i, err := strconv.Atoi(str)
	Check(err)
	return uint32(i)
}

var englishPrinter = message.NewPrinter(language.English)

func GetFormatIntFuncMap() template.FuncMap {
	return template.FuncMap{
		"formatInt": func(n uint32) string {
			return englishPrinter.Sprintf("%v", n)
		},
	}
}
