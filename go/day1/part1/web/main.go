package web

import (
	"bytes"
	"fmt"
	"html/template"
	"log"
	"net/http"
	"os"
	"time"

	"github.com/gorilla/mux"
	"github.com/rlyders/adventofcode/day1/part1/engine"
	"github.com/rlyders/adventofcode/utils"
)

var tmpl *template.Template

func init() {
	cwd, err := os.Getwd()
	if err != nil {
		log.Println("Failed to get current working directory:", err)
	} else {
		log.Println("Current working directory:", cwd)
	}

	pattern := "day1/part1/web/templates/*.html"
	tmpl, err = template.New("").Funcs(utils.GetFormatIntFuncMap()).ParseGlob(pattern)
	if err != nil {
		log.Fatalf("Error loading templates from '%s': %s", pattern, err.Error())
	}
}

const LOCATION_COLUMNS = `3   4
4   3
2   5
1   3
3   9
3   3`

func home(w http.ResponseWriter, r *http.Request) {
	var buf bytes.Buffer
	var location_pairs = engine.LocationPairs{
		Location_columns: LOCATION_COLUMNS,
		Sum_of_distances: 0,
	}
	err := tmpl.ExecuteTemplate(&buf, "index.html", location_pairs)
	if err != nil {
		// Handle execution error
		http.Error(w, fmt.Sprintf("Internal Server Error: %s", err), http.StatusInternalServerError)
		return
	}
	w.Write(buf.Bytes())
}

func getSumOfDistances(w http.ResponseWriter, r *http.Request) {
	start := time.Now()

	location_columns := r.FormValue("location_columns")

	list1, list2, err := engine.SplitAndSortLists(location_columns)
	utils.Check(err)

	results, err := engine.GetSumOfDistances(list1, list2)
	utils.Check(err)

	elapsed := time.Since(start)
	results.Elapsed_ms = uint32(elapsed.Milliseconds())
	results.Elapsed_μs = uint32(elapsed.Microseconds())

	var buf bytes.Buffer
	err = tmpl.ExecuteTemplate(&buf, "results.html", results)
	if err != nil {
		// Handle execution error
		http.Error(w, fmt.Sprintf("Internal Server Error: %s", err), http.StatusInternalServerError)
		return
	}
	w.Write(buf.Bytes())
}

func SetRoutes(router *mux.Router) {
	router.HandleFunc("/day1/part1", home).Methods("GET")
	router.HandleFunc("/day1/part1/sum_of_distances", getSumOfDistances).Methods("POST")
}
