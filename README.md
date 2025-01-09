# Advent Of Code 2024

I'm working through the [Advent Of Code 2024](https://adventofcode.com/2024) challenges to compare the use of go-lang and rust for:
 * full-stack apps using:
    * [HTMX](https://htmx.org/)
    * [Alpine.js](https://alpinejs.dev/)
 * command-line apps
  * The command-line versions of the go and rust apps reports the elapsed time to compare the performance of each.

GitHub: https://github.com/rlyders/adventofcode2024

# rust version of app

For rust, I've completed Day 1 both Part One and Part Two both as a command-line app and as a full-stack web-app using [HTMX](https://htmx.org/) and [Alpine.js](https://alpinejs.dev/).

prerequisites: rust 1.83.0+

## HOW-TO Run rust full-stack web app

The following builds and runs the rust full-stack [HTMX](https://htmx.org/)/[Alpine.js](https://alpinejs.dev/) web app which allows the user to select one of the challenges from the Advent Of Code 2024 that I've already completed. As of this writing, the user can select from Day 1 Part One or Part two. More to come...

```sh
cd rust && ./build.sh && ./target/release/aoc24-web && cd -
```

Once running, open up http://localhost:3000 in your browser and you should see the following "Advent Of Code 2024" home page where you can select the challenge you want to run.

![Advent Of Code 2024 Home in rust](images/rust/AdventOfCode2024-rust-home.png "Advent Of Code 2024 Home in rust")

### Rust full-stack Day 1 Part One

If you click on Day 1 Part One challenge, you will be shown the following screen.
![Advent Of Code 2024 Home - Day 1 Part One in rust](images/rust/day1/part1/AdventOfCode2024-rust-Day1-Part1-home.png "Advent Of Code 2024 - Day 1 Part One in rust")

You can either enter in your list of numbers, click [Choose file...] to load the list of numbers from a file, or use the default list of numbers provided.

When ready, click [Calculate] to process the list of numbers shown.
![Advent Of Code 2024 Home - Day 1 Part One in rust](images/rust/day1/part1/AdventOfCode2024-rust-Day1-Part1-results.png "Advent Of Code 2024 - Day 1 Part One in rust")

### Rust full-stack Day 1 Part Two

If you click on Day 1 Part Two challenge, you will be shown the following screen.
![Advent Of Code 2024 Home - Day 1 Part Two in rust](images/rust/day1/part2/AdventOfCode2024-rust-Day1-Part2-home.png "Advent Of Code 2024 - Day 1 Part Two in rust")

You can either enter in your list of numbers, click [Choose file...] to load the list of numbers from a file, or use the default list of numbers provided.

When ready, click [Calculate] to process the list of numbers shown.
![Advent Of Code 2024 Home - Day 1 Part Two in rust](images/rust/day1/part2/AdventOfCode2024-rust-Day1-Part2-results.png "Advent Of Code 2024 - Day 1 Part Two in rust")

## HOW-TO Run rust command-line app

The rust command-line app executes the Advent Of Code challenges using the data file path passed as the 1st command-line argument. If no command-line argument is passed, then is uses the the first valid path from the following list: `./data`, `../data`

The following script runs the rust command-line app with the default data file and 25,000 iterations.

```sh
./run.sh rust "" 25000
```

Sample output:
```log
$ ./run.sh rust "" 25000
Valid runtime: rust
Directory exists: rust
prog=./target/release/aoc24

------------- Run with timing -------------
day1 run: iterations: 25000
Iterations: 25000 ... all timings shown below are averages
------------------------------------------------------------------
      Total Distance:  2,970,687 [raw:    2970687]
-- Timings: --
           split and sort:  0 ms [    226 μs;    226,906 ns]
       calculate distance:  0 ms [      1 μs;      1,893 ns]
                    total:  0 ms [    229 μs;    229,062 ns]
------------------------------------------------------------------
Iterations: 25000 ... all timings shown below are averages
------------------------------------------------------------------
    Total Similarity: 23,963,899 [raw:   23963899]
-- Timings: --
           split and sort:  0 ms [     68 μs;     68,848 ns]
     calculate similarity:  0 ms [      4 μs;      4,464 ns]
                    total:  0 ms [     73 μs;     73,481 ns]
------------------------------------------------------------------

real    0m7.928s
user    0m4.816s
sys     0m0.000s
run: OK
-------------------------------------------
```

# go-lang version of app

For go-lang, I've completed Day 1 both Parts One and Two both as a command-line app and as a full-stack web-app using [HTMX](https://htmx.org/) and [Alpine.js](https://alpinejs.dev/).

prerequisites: go 1.23.4+
s
NOTE: to run the go-lang app, be sure to run the `./go/create-links.sh` Linux shell script that creates symbolic links for the `elapsed` and `iterations` HTML templates in for day1 part1 and part2 directories. This avoids the need to duplicate those templates in both directories.

## HOW-TO Run go-lang command-line app

The go-lang command-line app executes the Advent Of Code challenges using the data file path passed as the 1st command-line argument. If no command-line argument is passed, then is uses the the first valid path from the following list: `./data`, `../data`

The following script runs the go-lang command-line app with the default data file and 25,000 iterations.

```sh
./run.sh go "" 25000
```

Sample output:

```log
$ ./run.sh go "" 25000
Valid runtime: go
Directory exists: go
prog=./app/cmd/bin/aoc24

------------- Run with timing -------------
location lists file: ../data/day1/lists.txt
Iterations: 25000 ... all timings shown below are averages
------------------------------------------------------------------
           Total Distance:  2,970,687 [raw:    2970687]
-- Timings: --
           split and sort:  0 ms [    293 μs;     293,589 ns]
       calculate distance:  0 ms [      6 μs;       6,528 ns]
                    total:  0 ms [    293 μs;     293,589 ns]
------------------------------------------------------------------
Iterations: 25000 ... all timings shown below are averages
------------------------------------------------------------------
         Total Similarity: 23,963,899 [raw:   23963899]
-- Timings: --
           split and sort:  0 ms [    358 μs;     358,196 ns]
     calculate similarity:  0 ms [     44 μs;      44,282 ns]
                    total:  0 ms [    402 μs;     402,844 ns]
------------------------------------------------------------------

real    0m16.906s
user    0m18.875s
sys     0m1.191s
run: OK
-------------------------------------------
```

## HOW-TO Run go full-stack web app

The following builds and runs the go full-stack [HTMX](https://htmx.org/)/[Alpine.js](https://alpinejs.dev/) web app which allows the user to select one of the challenges from the Advent Of Code 2024 that I've already completed. As of this writing, the user can select from Day 1 Part One or Part two. More to come...

```sh
cd go && ./build.sh && go run app/web/main.go && cd -
```

Once running, open up http://localhost:3000 in your browser and you should see the following "Advent Of Code 2024" home page where you can select the challenge you want to run.

![Advent Of Code 2024 Home in go](images/go/AdventOfCode2024-go-home.png "Advent Of Code 2024 Home in go")

### Go full-stack Day 1 Part One

If you click on Day 1 Part One challenge, you will be shown the following screen.
![Advent Of Code 2024 Home - Day 1 Part One in go](images/go/day1/part1/AdventOfCode2024-go-Day1-Part1-home.png "Advent Of Code 2024 - Day 1 Part One in go")

You can either enter in your list of numbers, click [Choose file...] to load the list of numbers from a file, or use the default list of numbers provided.

When ready, click [Calculate] to process the list of numbers shown.
![Advent Of Code 2024 Home - Day 1 Part One in go](images/go/day1/part1/AdventOfCode2024-go-Day1-Part1-results.png "Advent Of Code 2024 - Day 1 Part One in go")

### Go full-stack Day 1 Part Two

If you click on Day 1 Part Two challenge, you will be shown the following screen.
![Advent Of Code 2024 Home - Day 1 Part Two in go](images/go/day1/part2/AdventOfCode2024-go-Day1-Part2-home.png "Advent Of Code 2024 - Day 1 Part Two in go")

You can either enter in your list of numbers, click [Choose file...] to load the list of numbers from a file, or use the default list of numbers provided.

When ready, click [Calculate] to process the list of numbers shown.
![Advent Of Code 2024 Home - Day 1 Part Two in go](images/go/day1/part2/AdventOfCode2024-go-Day1-Part2-results.png "Advent Of Code 2024 - Day 1 Part Two in go")

# TODOs

## TODO: add unit tests

