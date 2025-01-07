package main

import (
	"bytes"
	"fmt"
	"html/template"
	"log"
	"net/http"

	"github.com/gorilla/mux"
	day1_part1 "github.com/rlyders/adventofcode/day1/part1/web"
	"github.com/rlyders/adventofcode/utils"
)

var tmpl *template.Template

func init() {
	utils.OutputCwd()

	var err error
	tmpl, err = template.ParseGlob("templates/*.html")
	if err != nil {
		log.Fatal("Error loading templates:" + err.Error())
	}

}

func main() {
	gRouter := mux.NewRouter()
	gRouter.HandleFunc("/", home).Methods("GET")

	day1_part1.SetRoutes(gRouter)

	http.ListenAndServe(":3000", gRouter)
}

func home(w http.ResponseWriter, r *http.Request) {
	var buf bytes.Buffer
	err := tmpl.ExecuteTemplate(&buf, "index.html", utils.GetConfig())
	if err != nil {
		// Handle execution error
		http.Error(w, fmt.Sprintf("Internal Server Error: %s", err), http.StatusInternalServerError)
		return
	}
	w.Write(buf.Bytes())
}
