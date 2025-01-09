package web

import (
	"bytes"
	"fmt"
	"html/template"
	"log"
	"net/http"
	"strconv"

	"github.com/gorilla/mux"
	"github.com/rlyders/adventofcode/day1/part1/engine"
	"github.com/rlyders/adventofcode/utils"
)

var tmpl *template.Template

func init() {
	var err error
	pattern := "day1/part1/web/templates/*.html"
	tmpl, err = template.New("").Funcs(utils.GetFormatUInt32FuncMap()).Funcs(utils.GetFormatElapsedMillisecondsFuncMap()).Funcs(utils.GetFormatElapsedMicrosecondsFuncMap()).Funcs(utils.GetFormatElapsedNanosecondsFuncMap()).ParseGlob(pattern)
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
	location_columns := r.FormValue("location_columns")

	iterations_form_value := r.FormValue("iterations")
	iterations, err := strconv.ParseInt(iterations_form_value, 10, 64)
	if err != nil {
		panic(err)
	}

	results, err := engine.GetSimilarityScoreOfListsTextRepeated(location_columns, iterations)
	utils.Check(err, "GetSumOfDistancesOfListsText")

	var buf bytes.Buffer
	err = tmpl.ExecuteTemplate(&buf, "results.html", results)
	if err != nil {
		// Handle execution error
		msg := fmt.Sprintf("Internal Server Error: %s", err)
		fmt.Println(msg)
		http.Error(w, msg, http.StatusInternalServerError)
		return
	}
	w.Write(buf.Bytes())
}

func SetRoutes(router *mux.Router) {
	router.HandleFunc("/day1/part1", home).Methods("GET")
	router.HandleFunc("/day1/part1/calculate", getSumOfDistances).Methods("POST")
}
