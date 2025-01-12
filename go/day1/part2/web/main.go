package web

import (
	"bytes"
	"fmt"
	"html/template"
	"log"
	"net/http"
	"path/filepath"
	"runtime"
	"strconv"

	"github.com/gorilla/mux"
	"github.com/rlyders/adventofcode/day1/part2/engine"
	"github.com/rlyders/adventofcode/utils"
)

var tmpl *template.Template

func init() {
	var err error
	_, filename, _, _ := runtime.Caller(0)
	currentDir := filepath.Dir(filename)

	pattern := filepath.Join(currentDir, "templates", "*.html")
	tmpl, err = template.New("").Funcs(utils.GetFormatUInt32FuncMap()).Funcs(utils.GetFormatElapsedMillisecondsFuncMap()).Funcs(utils.GetFormatElapsedMicrosecondsFuncMap()).Funcs(utils.GetFormatElapsedNanosecondsFuncMap()).Funcs(utils.GetFormatSysInfoFuncMap()).ParseGlob(pattern)
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
	err := tmpl.ExecuteTemplate(&buf, "index.html", LOCATION_COLUMNS)
	if err != nil {
		// Handle execution error
		http.Error(w, fmt.Sprintf("Internal Server Error: %s", err), http.StatusInternalServerError)
		return
	}
	w.Write(buf.Bytes())
}

func getSimilarityScoreResults(w http.ResponseWriter, r *http.Request) {
	location_columns := r.FormValue("location_columns")

	iterations_form_value := r.FormValue("iterations")
	iterations, err := strconv.ParseInt(iterations_form_value, 10, 64)
	if err != nil {
		panic(err)
	}

	results, err := engine.GetSimilarityScoreOfListsTextRepeated(location_columns, iterations)
	utils.Check(err, "GetSimilarityScoreOfListsText")

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
	router.HandleFunc("/day1/part2", home).Methods("GET")
	router.HandleFunc("/day1/part2/calculate", getSimilarityScoreResults).Methods("POST")
}
