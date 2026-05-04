#![recursion_limit = "256"]
//! The Disassembler module provides functionality to convert EVM bytecode
//! into human-readable assembly instructions.
//!
//! This module enables the translation of raw bytecode into meaningful operations,
//! which is a critical step for understanding and analyzing smart contracts.

#![recursion_limit = "256"]

/// Error types for the disassembler module
pub mod error;

mod core;
mod interfaces;

// re-export the public interface
pub use core::disassemble;
pub use error::Error;
pub use heimdall_vm::core::hardfork::HardFork;
pub use interfaces::{DisassemblerArgs, DisassemblerArgsBuilder};
