//! VT100/VT220/ANSI Terminal escape sequences
//!
//! Provides routines for cursor control, screen clearing, etc.

use crate::CodeGen;

/// ESC character
const ESC: u8 = 0x1B;

impl CodeGen {
    // ========== Inline Escape Sequence Helpers ==========

    /// Emit ESC [ prefix (common to most sequences)
    fn emit_csi(&mut self) {
        self.ld_a(ESC);
        self.call("putchar");
        self.ld_a(b'[');
        self.call("putchar");
    }

    // ========== Screen Control Routines ==========

    /// Emit clear_screen routine (ESC[2J ESC[H)
    ///
    /// Labels created: `clear_screen`
    /// Requires: `putchar`
    pub fn emit_clear_screen(&mut self) {
        self.label("clear_screen");
        self.emit_csi();
        self.ld_a(b'2');
        self.call("putchar");
        self.ld_a(b'J');
        self.call("putchar");
        // Fall through to cursor_home
    }

    /// Emit cursor_home routine (ESC[H)
    ///
    /// Labels created: `cursor_home`
    /// Requires: `putchar`
    pub fn emit_cursor_home(&mut self) {
        self.label("cursor_home");
        self.emit_csi();
        self.ld_a(b'H');
        self.call("putchar");
        self.ret();
    }

    /// Emit combined clear_screen + cursor_home
    ///
    /// Labels created: `clear_screen`, `cursor_home`
    /// Requires: `putchar`
    pub fn emit_clear_screen_and_home(&mut self) {
        self.emit_clear_screen();
        self.emit_cursor_home();
    }

    /// Emit cursor_pos routine (ESC[row;colH)
    /// Input: B = row (1-based), C = col (1-based)
    ///
    /// Labels created: `cursor_pos`
    /// Requires: `putchar`, `print_byte_dec`
    pub fn emit_cursor_pos(&mut self) {
        self.label("cursor_pos");
        self.emit_csi();
        self.ld_a_b();              // Row
        self.call("print_byte_dec");
        self.ld_a(b';');
        self.call("putchar");
        self.ld_a_c();              // Col
        self.call("print_byte_dec");
        self.ld_a(b'H');
        self.call("putchar");
        self.ret();
    }

    /// Emit clear_to_eol routine (ESC[K)
    ///
    /// Labels created: `clear_to_eol`
    /// Requires: `putchar`
    pub fn emit_clear_to_eol(&mut self) {
        self.label("clear_to_eol");
        self.emit_csi();
        self.ld_a(b'K');
        self.call("putchar");
        self.ret();
    }

    /// Emit clear_to_eos routine (ESC[J) - clear from cursor to end of screen
    ///
    /// Labels created: `clear_to_eos`
    /// Requires: `putchar`
    pub fn emit_clear_to_eos(&mut self) {
        self.label("clear_to_eos");
        self.emit_csi();
        self.ld_a(b'J');
        self.call("putchar");
        self.ret();
    }

    // ========== Cursor Visibility ==========

    /// Emit cursor_hide routine (ESC[?25l)
    ///
    /// Labels created: `cursor_hide`
    /// Requires: `putchar`
    pub fn emit_cursor_hide(&mut self) {
        self.label("cursor_hide");
        self.emit_csi();
        self.ld_a(b'?');
        self.call("putchar");
        self.ld_a(b'2');
        self.call("putchar");
        self.ld_a(b'5');
        self.call("putchar");
        self.ld_a(b'l');
        self.call("putchar");
        self.ret();
    }

    /// Emit cursor_show routine (ESC[?25h)
    ///
    /// Labels created: `cursor_show`
    /// Requires: `putchar`
    pub fn emit_cursor_show(&mut self) {
        self.label("cursor_show");
        self.emit_csi();
        self.ld_a(b'?');
        self.call("putchar");
        self.ld_a(b'2');
        self.call("putchar");
        self.ld_a(b'5');
        self.call("putchar");
        self.ld_a(b'h');
        self.call("putchar");
        self.ret();
    }

    // ========== Cursor Movement ==========

    /// Emit cursor_up routine (ESC[A) - move cursor up 1 line
    ///
    /// Labels created: `cursor_up`
    /// Requires: `putchar`
    pub fn emit_cursor_up(&mut self) {
        self.label("cursor_up");
        self.emit_csi();
        self.ld_a(b'A');
        self.call("putchar");
        self.ret();
    }

    /// Emit cursor_down routine (ESC[B) - move cursor down 1 line
    ///
    /// Labels created: `cursor_down`
    /// Requires: `putchar`
    pub fn emit_cursor_down(&mut self) {
        self.label("cursor_down");
        self.emit_csi();
        self.ld_a(b'B');
        self.call("putchar");
        self.ret();
    }

    /// Emit cursor_right routine (ESC[C) - move cursor right 1 column
    ///
    /// Labels created: `cursor_right`
    /// Requires: `putchar`
    pub fn emit_cursor_right(&mut self) {
        self.label("cursor_right");
        self.emit_csi();
        self.ld_a(b'C');
        self.call("putchar");
        self.ret();
    }

    /// Emit cursor_left routine (ESC[D) - move cursor left 1 column
    ///
    /// Labels created: `cursor_left`
    /// Requires: `putchar`
    pub fn emit_cursor_left(&mut self) {
        self.label("cursor_left");
        self.emit_csi();
        self.ld_a(b'D');
        self.call("putchar");
        self.ret();
    }

    // ========== Text Attributes ==========

    /// Emit reset_attrs routine (ESC[0m) - reset all text attributes
    ///
    /// Labels created: `reset_attrs`
    /// Requires: `putchar`
    pub fn emit_reset_attrs(&mut self) {
        self.label("reset_attrs");
        self.emit_csi();
        self.ld_a(b'0');
        self.call("putchar");
        self.ld_a(b'm');
        self.call("putchar");
        self.ret();
    }

    /// Emit reverse_video routine (ESC[7m)
    ///
    /// Labels created: `reverse_video`
    /// Requires: `putchar`
    pub fn emit_reverse_video(&mut self) {
        self.label("reverse_video");
        self.emit_csi();
        self.ld_a(b'7');
        self.call("putchar");
        self.ld_a(b'm');
        self.call("putchar");
        self.ret();
    }

    // ========== Bundle Emitters ==========

    /// Emit all terminal routines
    ///
    /// Includes: clear_screen, cursor_home, cursor_pos, clear_to_eol,
    /// cursor_hide, cursor_show
    /// Requires: `putchar`, `print_byte_dec`
    pub fn emit_terminal_routines(&mut self) {
        self.emit_clear_screen_and_home();
        self.emit_cursor_pos();
        self.emit_clear_to_eol();
        self.emit_cursor_hide();
        self.emit_cursor_show();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_screen_emits() {
        let mut cg = CodeGen::new();
        cg.emit_clear_screen_and_home();
        assert!(cg.has_label("clear_screen"));
        assert!(cg.has_label("cursor_home"));
    }
}
