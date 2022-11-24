pub mod entrypoint;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod error;

// lib.rs - register modules
// entrypoint.rs - entry point to the program
// instruction.rs - serialize and deserialize instruction data
// processor.rs - program logic to process instructions
// state.rs - serialize and deserialize state
// error.rs - custom program errors