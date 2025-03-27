# Recursive Maze

A recursive maze generator and solver written in **Rust**, with real-time ASCII animation in the terminal.

This project is designed to help you **understand recursion** — both for generating mazes (via recursive backtracking) and solving them (via depth-first search). It's visual, educational, and mesmerizing to watch.

---

## Demo

```text
S#########################################
#     #     #     #     # #     #       #
# ### # ### ### ### ### # ### ### ##### #
#   #   #   # #     #   #   #     #   # #
# ##### # # # # ##### ##### ### ### # # #
# #     # #   # #       #     # #   # # #
# # ####### ### ### ##### ##### # ### ###
#   #     #     #     #   #     #     # E
#########################################

## Features
	•	Recursive maze generation (backtracking)
	•	Recursive maze solving (depth-first)
	•	Live animated terminal output
	•	ANSI color rendering
	•	Final stats: maze size, steps taken, time to solve
	•	Optional backtrack animation

## Requiremenets
Rust

## Run it
cargo run

## Customize Maze Size
const WIDTH: usize = 41;  // Must be odd
const HEIGHT: usize = 21; // Must be odd

## What you will learn
	•	How recursion works visually
	•	How maze algorithms explore and backtrack
	•	Terminal rendering with ANSI escape codes
	•	Structuring a clean Rust project with enums, state, and timing

## TODO / Ideas
	•	Add CLI args to set maze size or speed
	•	Export solved mazes to a file
	•	Add maze seeding for reproducibility
	•	Add animated backtracking
	•	Add themes or colored trails

## License
MIT 
