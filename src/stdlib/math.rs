//! Math and number conversion routines
//!
//! Provides routines for number printing, division, multiplication, etc.

use crate::CodeGen;

impl CodeGen {
    /// Emit print_byte_dec routine - prints A as decimal
    /// Always prints exactly what's needed, no leading zeros except for zero itself
    ///
    /// Labels created: `print_byte_dec`
    /// Requires: `putchar`
    pub fn emit_print_byte_dec(&mut self) {
        self.label("print_byte_dec");
        // Use stack to reverse digits
        self.ld_c(0);            // Digit count

        let extract_loop = self.unique_label("pbd_ext");
        self.label(&extract_loop);
        self.ld_b(0);            // Quotient
        let div_loop = self.unique_label("pbd_div");
        self.label(&div_loop);
        self.cp(10);
        let div_done = self.unique_label("pbd_ddone");
        self.jp_c(&div_done);
        self.sub_a(10);
        self.inc_b();
        self.jp(&div_loop);

        self.label(&div_done);
        // A = remainder (digit), B = quotient
        self.add_a(b'0');
        self.push_af();
        self.inc_c();
        self.ld_a_b();
        self.or_a_a();
        self.jp_nz(&extract_loop);

        // Pop and print digits
        let print_loop = self.unique_label("pbd_print");
        self.label(&print_loop);
        self.pop_af();
        self.call("putchar");
        self.dec_c();
        self.jp_nz(&print_loop);
        self.ret();
    }

    /// Emit div16 routine - 16-bit division HL / DE -> HL quotient, DE remainder
    ///
    /// Labels created: `div16`, `div16_loop`, `div16_done`
    pub fn emit_div16(&mut self) {
        self.label("div16");
        self.ld_bc(0);           // BC = quotient

        self.label("div16_loop");
        self.or_a_a();           // Clear carry
        self.sbc_hl_de();        // HL = HL - DE
        self.jp_c("div16_done");
        self.inc_bc();
        self.jp("div16_loop");

        self.label("div16_done");
        self.add_hl_de();        // Restore remainder to HL
        self.ex_de_hl();         // DE = remainder
        // Move BC to HL
        self.emit(&[0x60]);      // LD H, B
        self.emit(&[0x69]);      // LD L, C
        self.ret();
    }

    /// Emit mul8 routine - 8-bit multiply A * B -> HL
    ///
    /// Labels created: `mul8`, `mul8_loop`
    pub fn emit_mul8(&mut self) {
        self.label("mul8");
        self.ld_hl(0);
        self.or_a_a();
        self.ret_z();            // A * 0 = 0

        self.ld_c_a();
        self.ld_a_b();
        self.or_a_a();
        self.ret_z();            // 0 * B = 0

        // HL = 0, C = multiplicand, B = multiplier
        self.label("mul8_loop");
        self.emit(&[0x79]);      // LD A, C
        self.emit(&[0x85]);      // ADD A, L
        self.emit(&[0x6F]);      // LD L, A
        self.emit(&[0x30, 0x01]); // JR NC, +1
        self.emit(&[0x24]);      // INC H
        self.djnz("mul8_loop");
        self.ret();
    }

    /// Emit negate_hl routine - negate HL (two's complement)
    ///
    /// Labels created: `negate_hl`
    pub fn emit_negate_hl(&mut self) {
        self.label("negate_hl");
        self.emit(&[0x7C]);      // LD A, H
        self.cpl();
        self.emit(&[0x67]);      // LD H, A
        self.emit(&[0x7D]);      // LD A, L
        self.cpl();
        self.emit(&[0x6F]);      // LD L, A
        self.inc_hl();
        self.ret();
    }

    /// Emit all math routines
    pub fn emit_math_routines(&mut self) {
        self.emit_print_byte_dec();
        self.emit_div16();
        self.emit_negate_hl();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div16_emits() {
        let mut cg = CodeGen::new();
        cg.emit_div16();
        assert!(cg.has_label("div16"));
        assert!(cg.has_label("div16_loop"));
    }
}
