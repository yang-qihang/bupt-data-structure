// shared helpers for terminal output (clear screen, page titles)
pub mod ui;

// Token-based parsing shared by every interactive entry point (the main menu,
// the three `run` functions, and the parking lot's event records). It reads from
// any `BufRead`, so it can consume a single integer without gobbling a whole line.
pub mod parsing;

// The three problems need the following data structures:
//   1. a sequential stack
//   2. a singly linked stack
//   3. a doubly linked queue
// Those are useful abstractions to be extracted and managed
// in separate modules.
mod data_structures;

// Problem 1: Base Conversion
pub mod base_conversion;

// Problem 2: Bracket Matching
pub mod bracket_matching;

// Problem 3: Parking Lot
pub mod parking_lot;
