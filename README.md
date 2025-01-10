# Advent Of Code 2024

I'm working through the [Advent Of Code 2024](https://adventofcode.com/2024) challenges to compare the use of go-lang and rust for:
 * full-stack apps using:
    * [HTMX](https://htmx.org/)
    * [Alpine.js](https://alpinejs.dev/)
 * command-line apps
  * The command-line versions of the go and rust apps reports the elapsed time to compare the performance of each.

GitHub: https://github.com/rlyders/adventofcode2024

NOTE: This project was developed under Windows 11 WSL2, which is configured to use 50% of system memory. Thus, the following results from `free -h` reflect only half of the system RAM and these are typical results for my work on this project. I often run multiple instances of VS Code for this project to switch back and forth between rust and go-lang. 

```sh
$ free -h
               total        used        free      shared  buff/cache   available
Mem:           7.6Gi       6.0Gi       254Mi       3.1Mi       1.7Gi       1.7Gi
Swap:          2.0Gi       528Mi       1.5Gi
```

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

The following script runs the rust command-line app with the default data file repeatedly for 50,000 iterations.

```sh
./run.sh rust "" 50000
```

Sample output:
```log
$ ./run.sh rust "" 50000
Valid runtime: rust
Directory exists: rust
prog=./target/release/aoc24

------------- Run with timing -------------
# [PID  60321] START: main       : RAM: 3.2 MB   (resident: 1.1 MB   share: 1.0 MB   code: 340.0 KB data: 368.0 KB)
# [PID  60321] START: Day1       : RAM: 3.2 MB   (resident: 1.1 MB   share: 1.0 MB   code: 340.0 KB data: 368.0 KB)
# [PID  60321] START: Day1 Part1 : RAM: 3.2 MB   (resident: 1.1 MB   share: 1.0 MB   code: 340.0 KB data: 368.0 KB)
Iterations: 50,000 ... all timings shown below are averages
------------------------------------------------------------------
      Total Distance:  2,970,687 [raw:    2970687]
-- Timings: --
                    split:  0 ms [     73 μs;         73 ns]
                    sort1:  0 ms [     10 μs;         10 ns]
                    sort2:  0 ms [      9 μs;          9 ns]
           split and sort:  0 ms [     93 μs;         93 ns]
       calculate distance:  0 ms [      1 μs;          1 ns]
                    total:  0 ms [     95 μs;         95 ns]
------------------------------------------------------------------
# [PID  60321] START: Day1 Part1 : RAM: 3.2 MB   (resident: 1.1 MB   share: 1.0 MB   code: 340.0 KB data: 368.0 KB)
# [PID  60321] START: Day1 Part2 : RAM: 3.2 MB   (resident: 1.1 MB   share: 1.0 MB   code: 340.0 KB data: 368.0 KB)
Iterations: 50,000 ... all timings shown below are averages
------------------------------------------------------------------
    Total Similarity: 23,963,899 [raw:   23963899]
-- Timings: --
                    split:  0 ms [     74 μs;         74 ns]
                    sort1:  0 ms [     10 μs;         10 ns]
                    sort2:  0 ms [     10 μs;         10 ns]
           split and sort:  0 ms [     95 μs;         95 ns]
     calculate similarity:  0 ms [      5 μs;          5 ns]
                    total:  0 ms [    103 μs;        103 ns]
------------------------------------------------------------------
# [PID  60321] END  : Day1 Part2 : RAM: 3.2 MB   (resident: 1.1 MB   share: 1.0 MB   code: 340.0 KB data: 368.0 KB)
# [PID  60321] END  : Day1       : RAM: 3.2 MB   (resident: 1.1 MB   share: 1.0 MB   code: 340.0 KB data: 368.0 KB)
# [PID  60321] END  : main       : RAM: 3.2 MB   (resident: 1.1 MB   share: 1.0 MB   code: 340.0 KB data: 368.0 KB)

real    0m9.894s
user    0m10.048s
sys     0m0.010s
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

The following script runs the go-lang command-line app with the default data file repeatedly for 50,000 iterations.

```sh
./run.sh go "" 50000
```

Sample output:

```log
$ ./run.sh go "" 50000
Valid runtime: go
Directory exists: go
prog=./app/cmd/bin/aoc24

------------- Run with timing -------------
# [PID 66540 ] START: main       : RAM: 158.5 KB of avail:   6.3 MB [GC cummulative: 158.5 KB, cycles:    0]
location lists file: ../data/day1/lists.txt
# [PID 66540 ] START: Day1 Part1 : RAM: 159.4 KB of avail:   6.3 MB [GC cummulative: 159.4 KB, cycles:    0]
Iterations: 50,000 ... all timings shown below are averages
------------------------------------------------------------------
           Total Distance:  2,970,687 [raw:    2970687]
-- Timings: --
                    split:  0 ms [    138 μs;     138,908 ns]
                    sort1:  0 ms [     39 μs;      39,268 ns]
                    sort2:  0 ms [     35 μs;      35,656 ns]
           split and sort:  0 ms [    214 μs;     214,031 ns]
       calculate distance:  0 ms [      6 μs;       6,830 ns]
                    total:  0 ms [    221 μs;     221,338 ns]
------------------------------------------------------------------
# [PID 66540 ] END  : Day1 Part1 : RAM:   1.1 MB of avail:  15.2 MB [GC cummulative:   5.5 GB, cycles: 1783]
# [PID 66540 ] POST : GarbageCol : RAM: 159.8 KB of avail:  15.2 MB [GC cummulative:   5.5 GB, cycles: 1784]
# [PID 66540 ] START: Day1 Part2 : RAM: 161.0 KB of avail:  15.2 MB [GC cummulative:   5.5 GB, cycles: 1784]
Iterations: 50,000 ... all timings shown below are averages
------------------------------------------------------------------
         Total Similarity: 23,963,899 [raw:   23963899]
-- Timings: --
                    split:  0 ms [    143 μs;     143,590 ns]
                    sort1:  0 ms [     46 μs;      46,393 ns]
                    sort2:  0 ms [     42 μs;      42,829 ns]
           split and sort:  0 ms [    233 μs;     233,019 ns]
     calculate similarity:  0 ms [     40 μs;      40,394 ns]
                    total:  0 ms [    274 μs;     274,003 ns]
------------------------------------------------------------------
# [PID 66540 ] END  : Day1 Part2 : RAM:   2.7 MB of avail:  19.5 MB [GC cummulative:  11.7 GB, cycles: 3796]
# [PID 66540 ] END  : main       : RAM:   2.7 MB of avail:  19.5 MB [GC cummulative:  11.7 GB, cycles: 3796]
# [PID 66540 ] POST : GarbageCol : RAM: 163.1 KB of avail:  19.5 MB [GC cummulative:  11.7 GB, cycles: 3797]

real    0m27.407s
user    0m27.106s
sys     0m2.333s
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

