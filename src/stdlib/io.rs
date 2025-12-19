//! MC6850 ACIA Serial I/O routines
//!
//! Standard RetroShield configuration:
//! - Status port: 0x80
//! - Data port: 0x81
//! - Bit 0 of status: RX ready
//! - Bit 1 of status: TX ready

use crate::CodeGen;

/// MC6850 port configuration
pub struct MC6850Config {
    pub status_port: u8,
    pub data_port: u8,
    pub rx_ready_bit: u8,
    pub tx_ready_bit: u8,
}

impl Default for MC6850Config {
    fn default() -> Self {
        Self {
            status_port: 0x80,
            data_port: 0x81,
            rx_ready_bit: 0x01,
            tx_ready_bit: 0x02,
        }
    }
}

impl CodeGen {
    /// Emit getchar routine (blocking read, char returned in A)
    ///
    /// Labels created: `getchar`
    pub fn emit_getchar(&mut self) {
        self.emit_getchar_config(&MC6850Config::default());
    }

    /// Emit getchar with custom port configuration
    pub fn emit_getchar_config(&mut self, config: &MC6850Config) {
        self.label("getchar");
        self.in_a(config.status_port);
        self.and_a(config.rx_ready_bit);
        self.emit(&[0x28, 0xFA]); // JR Z, -6 (back to getchar)
        self.in_a(config.data_port);
        self.ret();
    }

    /// Emit putchar routine (blocking write, char in A)
    ///
    /// Labels created: `putchar`, `putchar_wait`
    pub fn emit_putchar(&mut self) {
        self.emit_putchar_config(&MC6850Config::default());
    }

    /// Emit putchar with custom port configuration
    pub fn emit_putchar_config(&mut self, config: &MC6850Config) {
        self.label("putchar");
        self.push_af();
        self.label("putchar_wait");
        self.in_a(config.status_port);
        self.and_a(config.tx_ready_bit);
        self.emit(&[0x28, 0xFA]); // JR Z, -6 (back to putchar_wait)
        self.pop_af();
        self.out_a(config.data_port);
        self.ret();
    }

    /// Emit newline routine (prints CR LF)
    ///
    /// Labels created: `newline`
    /// Requires: `putchar`
    pub fn emit_newline(&mut self) {
        self.label("newline");
        self.ld_a(0x0D); // CR
        self.call("putchar");
        self.ld_a(0x0A); // LF
        self.call("putchar");
        self.ret();
    }

    /// Emit print_string routine (prints null-terminated string at HL)
    ///
    /// Labels created: `print_string`, `print_string_loop`
    /// Requires: `putchar`
    pub fn emit_print_string(&mut self) {
        self.label("print_string");
        self.label("print_string_loop");
        self.ld_a_hl_ind();      // LD A, (HL)
        self.or_a_a();           // OR A (test for null)
        self.ret_z();            // RET Z (if null, done)
        self.call("putchar");
        self.inc_hl();
        self.jp("print_string_loop");
    }

    /// Emit all standard I/O routines
    ///
    /// Includes: getchar, putchar, newline, print_string
    pub fn emit_io_routines(&mut self) {
        self.emit_getchar();
        self.emit_putchar();
        self.emit_newline();
        self.emit_print_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getchar_emits() {
        let mut cg = CodeGen::new();
        cg.emit_getchar();
        assert!(cg.has_label("getchar"));
        assert!(cg.size() > 0);
    }

    #[test]
    fn test_putchar_emits() {
        let mut cg = CodeGen::new();
        cg.emit_putchar();
        assert!(cg.has_label("putchar"));
        assert!(cg.has_label("putchar_wait"));
    }
}
