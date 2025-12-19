# retroshield-z80-workbench

A Rust framework for generating Z80 machine code ROMs, designed for [RetroShield](https://www.tindie.com/stores/8bitforce/) projects and other Z80-based systems.

Build Z80 programs in Rust using a fluent API instead of writing raw assembly or hex bytes.

## Features

- **Instruction Helpers**: 80+ Z80 instructions as Rust methods (`ld_a()`, `call()`, `jp_z()`, etc.)
- **Label System**: Define labels and forward-reference them; fixups resolved automatically
- **Standard Library**: Pre-built routines for serial I/O, VT100 terminal control, and math
- **Zero Dependencies**: Pure Rust, no external assembler needed
- **Multiple Output Formats**: Binary (`.bin`) and Intel HEX (`.hex`)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
retroshield-z80-workbench = "0.1"
```

## Quick Start

Here's a complete "Hello World" program that prints to a serial terminal:

```rust
use retroshield_z80_workbench::prelude::*;

fn main() {
    let mut rom = CodeGen::new();

    // Setup
    rom.emit_startup(0x3FFF);  // Set stack pointer

    // Main program
    rom.label("main");
    rom.call("clear_screen");
    rom.ld_hl_label("hello_msg");
    rom.call("print_string");

    rom.label("loop");
    rom.jp("loop");  // Infinite loop

    // Data
    rom.label("hello_msg");
    rom.emit_string("Hello, RetroShield!\r\n");

    // Include standard library (I/O, terminal, math routines)
    rom.include_stdlib();

    // Finalize and write
    rom.resolve_fixups();
    rom.write_bin("hello.bin").unwrap();

    println!("Generated {} bytes", rom.size());
}
```

## API Overview

### Core Methods

```rust
let mut rom = CodeGen::new();

// Emit raw bytes
rom.emit(&[0x00, 0x01, 0x02]);
rom.emit_byte(0xFF);
rom.emit_word(0x1234);        // Little-endian
rom.emit_string("Hello\0");   // Null-terminated

// Labels and fixups
rom.label("my_label");
rom.jp("my_label");           // Forward reference OK
rom.resolve_fixups();         // Call once at the end

// Output
rom.write_bin("output.bin")?;
rom.write_hex("output.hex")?;
```

### Instruction Helpers

Instead of remembering opcodes, use named methods:

```rust
// 8-bit loads
rom.ld_a(0x42);           // LD A, 0x42
rom.ld_b(10);             // LD B, 10
rom.ld_a_b();             // LD A, B
rom.ld_a_hl_ind();        // LD A, (HL)
rom.ld_hl_ind_a();        // LD (HL), A
rom.ld_a_addr(0x3000);    // LD A, (0x3000)
rom.ld_addr_a(0x3000);    // LD (0x3000), A

// 16-bit loads
rom.ld_hl(0x2000);        // LD HL, 0x2000
rom.ld_de(0x1000);        // LD DE, 0x1000
rom.ld_bc(100);           // LD BC, 100
rom.ld_sp(0x3FFF);        // LD SP, 0x3FFF
rom.ld_hl_label("data");  // LD HL, data (with fixup)

// Stack
rom.push_af();
rom.push_hl();
rom.pop_de();
rom.pop_bc();

// Arithmetic
rom.add_a(5);             // ADD A, 5
rom.sub_a(1);             // SUB 1
rom.inc_a();
rom.dec_b();
rom.inc_hl();
rom.dec_de();
rom.add_hl_de();          // ADD HL, DE
rom.add_hl_bc();          // ADD HL, BC

// Logic
rom.and_a(0x0F);          // AND 0x0F
rom.or_a(0x80);           // OR 0x80
rom.xor_a();              // XOR A (clear A)
rom.cp(0x0D);             // CP 0x0D
rom.cpl();                // CPL (complement A)

// Jumps
rom.jp("label");          // JP label
rom.jp_z("label");        // JP Z, label
rom.jp_nz("label");       // JP NZ, label
rom.jp_c("label");        // JP C, label
rom.jp_nc("label");       // JP NC, label
rom.jr("label");          // JR label (relative)
rom.djnz("label");        // DJNZ label

// Calls and returns
rom.call("subroutine");   // CALL subroutine
rom.ret();                // RET
rom.ret_z();              // RET Z
rom.ret_nz();             // RET NZ

// I/O
rom.in_a(0x80);           // IN A, (0x80)
rom.out_a(0x81);          // OUT (0x81), A

// Misc
rom.nop();
rom.halt();
rom.di();                 // Disable interrupts
rom.ei();                 // Enable interrupts
rom.ex_de_hl();           // EX DE, HL
```

### Standard Library

The framework includes pre-built routines for common tasks:

```rust
// Include all standard library routines
rom.include_stdlib();

// Or include selectively:
rom.emit_io_routines();       // getchar, putchar, print_string, newline
rom.emit_terminal_routines(); // clear_screen, cursor_pos, cursor_home, etc.
rom.emit_math_routines();     // print_byte_dec, div16, negate_hl
```

**I/O Routines** (MC6850 ACIA at ports 0x80/0x81):
- `getchar` - Read character into A (blocking)
- `putchar` - Write character from A
- `print_string` - Print null-terminated string at HL
- `newline` - Print CR+LF

**Terminal Routines** (VT100/ANSI):
- `clear_screen` - Clear screen and home cursor
- `cursor_home` - Move cursor to top-left
- `cursor_pos` - Move cursor to row B, column C
- `clear_to_eol` - Clear from cursor to end of line
- `cursor_hide` / `cursor_show` - Toggle cursor visibility

**Math Routines**:
- `print_byte_dec` - Print A as decimal number
- `div16` - 16-bit division: HL / DE → HL quotient, DE remainder
- `negate_hl` - Two's complement negate HL

## Complete Example: Number Counter

A program that counts from 0 to 255 on the terminal:

```rust
use retroshield_z80_workbench::prelude::*;

fn main() {
    let mut rom = CodeGen::new();

    // Initialize
    rom.emit_startup(0x3FFF);
    rom.call("clear_screen");

    // Print header
    rom.ld_hl_label("header_msg");
    rom.call("print_string");

    // Initialize counter
    rom.xor_a();
    rom.ld_addr_a(0x3000);  // Store counter at 0x3000

    // Main loop
    rom.label("count_loop");

    // Print current value
    rom.ld_a_addr(0x3000);
    rom.call("print_byte_dec");
    rom.ld_a(b' ');
    rom.call("putchar");

    // Increment counter
    rom.ld_a_addr(0x3000);
    rom.inc_a();
    rom.ld_addr_a(0x3000);

    // Loop until overflow (A wraps from 255 to 0)
    rom.jp_nz("count_loop");

    // Done
    rom.call("newline");
    rom.ld_hl_label("done_msg");
    rom.call("print_string");
    rom.halt();

    // Data
    rom.label("header_msg");
    rom.emit_string("Counting: ");

    rom.label("done_msg");
    rom.emit_string("\r\nDone!\r\n");

    // Standard library
    rom.include_stdlib();

    // Finalize
    rom.resolve_fixups();
    rom.write_bin("counter.bin").unwrap();

    println!("Generated counter.bin ({} bytes)", rom.size());
}
```

## Memory Map

Default configuration for RetroShield Z80:

| Address | Size | Description |
|---------|------|-------------|
| 0x0000-0x1FFF | 8KB | ROM |
| 0x2000-0x3FFF | 8KB | RAM |
| 0x80 | - | MC6850 Status Register |
| 0x81 | - | MC6850 Data Register |

## Custom I/O Ports

Configure different I/O ports for the serial routines:

```rust
use retroshield_z80_workbench::stdlib::io::MC6850Config;

let config = MC6850Config {
    status_port: 0x00,
    data_port: 0x01,
};
rom.emit_io_routines_with_config(&config);
```

## Tips

1. **Always call `resolve_fixups()`** after emitting all code and before writing output.

2. **Place data after code** to avoid executing data as instructions.

3. **Standard library goes last** - include it after your main code so execution doesn't fall into library routines.

4. **Use `unique_label()`** for generated code to avoid label collisions:
   ```rust
   let loop_label = rom.unique_label("loop");
   rom.label(&loop_label);
   // ... loop body ...
   rom.jp(&loop_label);
   ```

## License

BSD 3-Clause License. See [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! Please open an issue or pull request on GitHub.

## See Also

- [RetroShield](https://www.tindie.com/stores/8bitforce/) - Arduino shields for retro CPUs
- [Z80 Instruction Set](http://z80-heaven.wikidot.com/instructions-set) - Reference documentation
