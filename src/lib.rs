//! RetroShield Z80 Workbench
//!
//! A framework for building Z80 ROMs for RetroShield projects.
//!
//! # Example
//!
//! ```rust
//! use retroshield_z80_workbench::prelude::*;
//!
//! let mut rom = CodeGen::new();
//!
//! // Setup
//! rom.label("start");
//! rom.ld_sp(0x3FFF);
//! rom.di();
//!
//! // Include standard I/O
//! rom.emit_io_routines();
//! rom.emit_terminal_routines();
//! rom.emit_math_routines();
//!
//! // Your code here
//! rom.label("main");
//! rom.call("clear_screen");
//! rom.ld_hl_label("hello_msg");
//! rom.call("print_string");
//! rom.jp("main");
//!
//! // Data
//! rom.label("hello_msg");
//! rom.emit_string("Hello, RetroShield!\r\n");
//!
//! // Finalize and write
//! rom.resolve_fixups();
//! rom.write_bin("output.bin").unwrap();
//! ```
//!
//! # Module Structure
//!
//! - `codegen` - Core emit/label/fixup machinery
//! - `instructions` - Z80 instruction helpers
//! - `stdlib::io` - MC6850 serial I/O routines
//! - `stdlib::terminal` - VT100/ANSI terminal sequences
//! - `stdlib::math` - Number conversion and math routines

mod codegen;
mod instructions;
pub mod stdlib;

pub use codegen::{CodeGen, RomConfig};

/// Prelude - import this for convenient access to common types
pub mod prelude {
    pub use crate::codegen::{CodeGen, RomConfig};
}

/// Convenience extension methods for CodeGen
impl CodeGen {
    /// Standard RetroShield startup sequence
    /// Sets up stack pointer and disables interrupts
    pub fn emit_startup(&mut self, stack_top: u16) {
        self.label("_start");
        self.di();
        self.ld_sp(stack_top);
    }

    /// Load HL with address of a label (for string pointers, etc.)
    pub fn ld_hl_label(&mut self, label: &str) {
        self.emit(&[0x21]); // LD HL, nn
        self.fixup(label);
    }

    /// Load DE with address of a label
    pub fn ld_de_label(&mut self, label: &str) {
        self.emit(&[0x11]); // LD DE, nn
        self.fixup(label);
    }

    /// Load BC with address of a label
    pub fn ld_bc_label(&mut self, label: &str) {
        self.emit(&[0x01]); // LD BC, nn
        self.fixup(label);
    }

    /// Emit a labeled string constant
    pub fn string_const(&mut self, label: &str, s: &str) {
        self.label(label);
        self.emit_string(s);
    }

    /// Include all standard library routines
    /// This is a convenience method that includes:
    /// - I/O routines (getchar, putchar, newline, print_string)
    /// - Terminal routines (clear_screen, cursor_pos, etc.)
    /// - Math routines (print_byte_dec, div16, etc.)
    pub fn include_stdlib(&mut self) {
        self.emit_io_routines();
        self.emit_terminal_routines();
        self.emit_math_routines();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_program() {
        let mut rom = CodeGen::new();

        rom.emit_startup(0x3FFF);
        rom.label("main");
        rom.halt();

        rom.resolve_fixups();
        assert!(rom.size() > 0);
    }

    #[test]
    fn test_with_stdlib() {
        let mut rom = CodeGen::new();

        rom.emit_startup(0x3FFF);

        // Main code
        rom.label("main");
        rom.call("clear_screen");
        rom.ld_hl_label("msg");
        rom.call("print_string");
        rom.jp("main");

        // Data
        rom.string_const("msg", "Hello!\r\n");

        // Include stdlib (must come after main code to not disrupt flow)
        rom.include_stdlib();

        rom.resolve_fixups();
        println!("ROM size: {} bytes", rom.size());
    }
}
