//! Z80 instruction helpers
//!
//! Provides ergonomic methods for emitting Z80 instructions.
//! Instead of `emit(&[0x3E, 0x0A])` you can write `ld_a(0x0A)`.

use crate::CodeGen;

impl CodeGen {
    // ========== 8-bit Load Instructions ==========

    /// LD A, n
    pub fn ld_a(&mut self, n: u8) {
        self.emit(&[0x3E, n]);
    }

    /// LD B, n
    pub fn ld_b(&mut self, n: u8) {
        self.emit(&[0x06, n]);
    }

    /// LD C, n
    pub fn ld_c(&mut self, n: u8) {
        self.emit(&[0x0E, n]);
    }

    /// LD D, n
    pub fn ld_d(&mut self, n: u8) {
        self.emit(&[0x16, n]);
    }

    /// LD E, n
    pub fn ld_e(&mut self, n: u8) {
        self.emit(&[0x1E, n]);
    }

    /// LD H, n
    pub fn ld_h(&mut self, n: u8) {
        self.emit(&[0x26, n]);
    }

    /// LD L, n
    pub fn ld_l(&mut self, n: u8) {
        self.emit(&[0x2E, n]);
    }

    /// LD A, (HL)
    pub fn ld_a_hl_ind(&mut self) {
        self.emit(&[0x7E]);
    }

    /// LD (HL), A
    pub fn ld_hl_ind_a(&mut self) {
        self.emit(&[0x77]);
    }

    /// LD A, B
    pub fn ld_a_b(&mut self) {
        self.emit(&[0x78]);
    }

    /// LD A, C
    pub fn ld_a_c(&mut self) {
        self.emit(&[0x79]);
    }

    /// LD A, D
    pub fn ld_a_d(&mut self) {
        self.emit(&[0x7A]);
    }

    /// LD A, E
    pub fn ld_a_e(&mut self) {
        self.emit(&[0x7B]);
    }

    /// LD B, A
    pub fn ld_b_a(&mut self) {
        self.emit(&[0x47]);
    }

    /// LD C, A
    pub fn ld_c_a(&mut self) {
        self.emit(&[0x4F]);
    }

    /// LD D, A
    pub fn ld_d_a(&mut self) {
        self.emit(&[0x57]);
    }

    /// LD E, A
    pub fn ld_e_a(&mut self) {
        self.emit(&[0x5F]);
    }

    /// LD A, (nn)
    pub fn ld_a_addr(&mut self, addr: u16) {
        self.emit(&[0x3A]);
        self.emit_word(addr);
    }

    /// LD (nn), A
    pub fn ld_addr_a(&mut self, addr: u16) {
        self.emit(&[0x32]);
        self.emit_word(addr);
    }

    // ========== 16-bit Load Instructions ==========

    /// LD BC, nn
    pub fn ld_bc(&mut self, nn: u16) {
        self.emit(&[0x01]);
        self.emit_word(nn);
    }

    /// LD DE, nn
    pub fn ld_de(&mut self, nn: u16) {
        self.emit(&[0x11]);
        self.emit_word(nn);
    }

    /// LD HL, nn
    pub fn ld_hl(&mut self, nn: u16) {
        self.emit(&[0x21]);
        self.emit_word(nn);
    }

    /// LD SP, nn
    pub fn ld_sp(&mut self, nn: u16) {
        self.emit(&[0x31]);
        self.emit_word(nn);
    }

    /// LD HL, (nn)
    pub fn ld_hl_addr(&mut self, addr: u16) {
        self.emit(&[0x2A]);
        self.emit_word(addr);
    }

    /// LD (nn), HL
    pub fn ld_addr_hl(&mut self, addr: u16) {
        self.emit(&[0x22]);
        self.emit_word(addr);
    }

    /// LD DE, (nn) - ED instruction
    pub fn ld_de_addr(&mut self, addr: u16) {
        self.emit(&[0xED, 0x5B]);
        self.emit_word(addr);
    }

    /// LD (nn), DE - ED instruction
    pub fn ld_addr_de(&mut self, addr: u16) {
        self.emit(&[0xED, 0x53]);
        self.emit_word(addr);
    }

    /// LD SP, HL
    pub fn ld_sp_hl(&mut self) {
        self.emit(&[0xF9]);
    }

    // ========== Stack Operations ==========

    /// PUSH AF
    pub fn push_af(&mut self) {
        self.emit(&[0xF5]);
    }

    /// PUSH BC
    pub fn push_bc(&mut self) {
        self.emit(&[0xC5]);
    }

    /// PUSH DE
    pub fn push_de(&mut self) {
        self.emit(&[0xD5]);
    }

    /// PUSH HL
    pub fn push_hl(&mut self) {
        self.emit(&[0xE5]);
    }

    /// POP AF
    pub fn pop_af(&mut self) {
        self.emit(&[0xF1]);
    }

    /// POP BC
    pub fn pop_bc(&mut self) {
        self.emit(&[0xC1]);
    }

    /// POP DE
    pub fn pop_de(&mut self) {
        self.emit(&[0xD1]);
    }

    /// POP HL
    pub fn pop_hl(&mut self) {
        self.emit(&[0xE1]);
    }

    // ========== Exchange Instructions ==========

    /// EX DE, HL
    pub fn ex_de_hl(&mut self) {
        self.emit(&[0xEB]);
    }

    /// EX AF, AF'
    pub fn ex_af(&mut self) {
        self.emit(&[0x08]);
    }

    /// EXX
    pub fn exx(&mut self) {
        self.emit(&[0xD9]);
    }

    // ========== Arithmetic - 8 bit ==========

    /// ADD A, n
    pub fn add_a(&mut self, n: u8) {
        self.emit(&[0xC6, n]);
    }

    /// ADD A, B
    pub fn add_a_b(&mut self) {
        self.emit(&[0x80]);
    }

    /// ADD A, (HL)
    pub fn add_a_hl_ind(&mut self) {
        self.emit(&[0x86]);
    }

    /// SUB n
    pub fn sub_a(&mut self, n: u8) {
        self.emit(&[0xD6, n]);
    }

    /// SUB B
    pub fn sub_b(&mut self) {
        self.emit(&[0x90]);
    }

    /// INC A
    pub fn inc_a(&mut self) {
        self.emit(&[0x3C]);
    }

    /// INC B
    pub fn inc_b(&mut self) {
        self.emit(&[0x04]);
    }

    /// INC C
    pub fn inc_c(&mut self) {
        self.emit(&[0x0C]);
    }

    /// DEC A
    pub fn dec_a(&mut self) {
        self.emit(&[0x3D]);
    }

    /// DEC B
    pub fn dec_b(&mut self) {
        self.emit(&[0x05]);
    }

    /// DEC C
    pub fn dec_c(&mut self) {
        self.emit(&[0x0D]);
    }

    // ========== Arithmetic - 16 bit ==========

    /// INC HL
    pub fn inc_hl(&mut self) {
        self.emit(&[0x23]);
    }

    /// INC DE
    pub fn inc_de(&mut self) {
        self.emit(&[0x13]);
    }

    /// INC BC
    pub fn inc_bc(&mut self) {
        self.emit(&[0x03]);
    }

    /// DEC HL
    pub fn dec_hl(&mut self) {
        self.emit(&[0x2B]);
    }

    /// DEC DE
    pub fn dec_de(&mut self) {
        self.emit(&[0x1B]);
    }

    /// DEC BC
    pub fn dec_bc(&mut self) {
        self.emit(&[0x0B]);
    }

    /// ADD HL, BC
    pub fn add_hl_bc(&mut self) {
        self.emit(&[0x09]);
    }

    /// ADD HL, DE
    pub fn add_hl_de(&mut self) {
        self.emit(&[0x19]);
    }

    /// ADD HL, HL
    pub fn add_hl_hl(&mut self) {
        self.emit(&[0x29]);
    }

    /// SBC HL, DE
    pub fn sbc_hl_de(&mut self) {
        self.emit(&[0xED, 0x52]);
    }

    /// SBC HL, BC
    pub fn sbc_hl_bc(&mut self) {
        self.emit(&[0xED, 0x42]);
    }

    // ========== Logic ==========

    /// AND n
    pub fn and_a(&mut self, n: u8) {
        self.emit(&[0xE6, n]);
    }

    /// OR n
    pub fn or_a(&mut self, n: u8) {
        self.emit(&[0xF6, n]);
    }

    /// OR A (common for flag check)
    pub fn or_a_a(&mut self) {
        self.emit(&[0xB7]);
    }

    /// OR B
    pub fn or_b(&mut self) {
        self.emit(&[0xB0]);
    }

    /// OR L
    pub fn or_l(&mut self) {
        self.emit(&[0xB5]);
    }

    /// XOR A
    pub fn xor_a(&mut self) {
        self.emit(&[0xAF]);
    }

    /// XOR n
    pub fn xor_n(&mut self, n: u8) {
        self.emit(&[0xEE, n]);
    }

    /// CP n
    pub fn cp(&mut self, n: u8) {
        self.emit(&[0xFE, n]);
    }

    /// CP B
    pub fn cp_b(&mut self) {
        self.emit(&[0xB8]);
    }

    /// CP (HL)
    pub fn cp_hl_ind(&mut self) {
        self.emit(&[0xBE]);
    }

    /// CPL (complement A)
    pub fn cpl(&mut self) {
        self.emit(&[0x2F]);
    }

    // ========== Jumps ==========

    /// JP nn (with fixup)
    pub fn jp(&mut self, label: &str) {
        self.emit(&[0xC3]);
        self.fixup(label);
    }

    /// JP nn (absolute address)
    pub fn jp_addr(&mut self, addr: u16) {
        self.emit(&[0xC3]);
        self.emit_word(addr);
    }

    /// JP Z, nn
    pub fn jp_z(&mut self, label: &str) {
        self.emit(&[0xCA]);
        self.fixup(label);
    }

    /// JP NZ, nn
    pub fn jp_nz(&mut self, label: &str) {
        self.emit(&[0xC2]);
        self.fixup(label);
    }

    /// JP C, nn
    pub fn jp_c(&mut self, label: &str) {
        self.emit(&[0xDA]);
        self.fixup(label);
    }

    /// JP NC, nn
    pub fn jp_nc(&mut self, label: &str) {
        self.emit(&[0xD2]);
        self.fixup(label);
    }

    /// JP P, nn (positive/sign flag clear)
    pub fn jp_p(&mut self, label: &str) {
        self.emit(&[0xF2]);
        self.fixup(label);
    }

    /// JP M, nn (minus/sign flag set)
    pub fn jp_m(&mut self, label: &str) {
        self.emit(&[0xFA]);
        self.fixup(label);
    }

    /// JP (HL)
    pub fn jp_hl(&mut self) {
        self.emit(&[0xE9]);
    }

    /// JR e (relative jump, label must be defined)
    pub fn jr(&mut self, label: &str) {
        self.emit(&[0x18]);
        self.emit_relative(label);
    }

    /// JR Z, e
    pub fn jr_z(&mut self, label: &str) {
        self.emit(&[0x28]);
        self.emit_relative(label);
    }

    /// JR NZ, e
    pub fn jr_nz(&mut self, label: &str) {
        self.emit(&[0x20]);
        self.emit_relative(label);
    }

    /// JR C, e
    pub fn jr_c(&mut self, label: &str) {
        self.emit(&[0x38]);
        self.emit_relative(label);
    }

    /// JR NC, e
    pub fn jr_nc(&mut self, label: &str) {
        self.emit(&[0x30]);
        self.emit_relative(label);
    }

    /// DJNZ e (decrement B, jump if not zero)
    pub fn djnz(&mut self, label: &str) {
        self.emit(&[0x10]);
        self.emit_relative(label);
    }

    // ========== Calls and Returns ==========

    /// CALL nn (with fixup)
    pub fn call(&mut self, label: &str) {
        self.emit(&[0xCD]);
        self.fixup(label);
    }

    /// CALL nn (absolute address)
    pub fn call_addr(&mut self, addr: u16) {
        self.emit(&[0xCD]);
        self.emit_word(addr);
    }

    /// CALL Z, nn
    pub fn call_z(&mut self, label: &str) {
        self.emit(&[0xCC]);
        self.fixup(label);
    }

    /// CALL NZ, nn
    pub fn call_nz(&mut self, label: &str) {
        self.emit(&[0xC4]);
        self.fixup(label);
    }

    /// RET
    pub fn ret(&mut self) {
        self.emit(&[0xC9]);
    }

    /// RET Z
    pub fn ret_z(&mut self) {
        self.emit(&[0xC8]);
    }

    /// RET NZ
    pub fn ret_nz(&mut self) {
        self.emit(&[0xC0]);
    }

    /// RET C
    pub fn ret_c(&mut self) {
        self.emit(&[0xD8]);
    }

    /// RET NC
    pub fn ret_nc(&mut self) {
        self.emit(&[0xD0]);
    }

    // ========== I/O ==========

    /// IN A, (n)
    pub fn in_a(&mut self, port: u8) {
        self.emit(&[0xDB, port]);
    }

    /// OUT (n), A
    pub fn out_a(&mut self, port: u8) {
        self.emit(&[0xD3, port]);
    }

    // ========== Misc ==========

    /// NOP
    pub fn nop(&mut self) {
        self.emit(&[0x00]);
    }

    /// HALT
    pub fn halt(&mut self) {
        self.emit(&[0x76]);
    }

    /// DI (disable interrupts)
    pub fn di(&mut self) {
        self.emit(&[0xF3]);
    }

    /// EI (enable interrupts)
    pub fn ei(&mut self) {
        self.emit(&[0xFB]);
    }

    /// SCF (set carry flag)
    pub fn scf(&mut self) {
        self.emit(&[0x37]);
    }

    /// CCF (complement carry flag)
    pub fn ccf(&mut self) {
        self.emit(&[0x3F]);
    }

    // ========== Bit Operations ==========

    /// BIT b, A
    pub fn bit_a(&mut self, bit: u8) {
        self.emit(&[0xCB, 0x47 | (bit << 3)]);
    }

    /// SET b, A
    pub fn set_a(&mut self, bit: u8) {
        self.emit(&[0xCB, 0xC7 | (bit << 3)]);
    }

    /// RES b, A
    pub fn res_a(&mut self, bit: u8) {
        self.emit(&[0xCB, 0x87 | (bit << 3)]);
    }

    /// RLA (rotate left through carry)
    pub fn rla(&mut self) {
        self.emit(&[0x17]);
    }

    /// RRA (rotate right through carry)
    pub fn rra(&mut self) {
        self.emit(&[0x1F]);
    }

    /// RLCA (rotate left circular)
    pub fn rlca(&mut self) {
        self.emit(&[0x07]);
    }

    /// RRCA (rotate right circular)
    pub fn rrca(&mut self) {
        self.emit(&[0x0F]);
    }

    /// SLA A (shift left arithmetic)
    pub fn sla_a(&mut self) {
        self.emit(&[0xCB, 0x27]);
    }

    /// SRA A (shift right arithmetic)
    pub fn sra_a(&mut self) {
        self.emit(&[0xCB, 0x2F]);
    }

    /// SRL A (shift right logical)
    pub fn srl_a(&mut self) {
        self.emit(&[0xCB, 0x3F]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ld_a() {
        let mut cg = CodeGen::new();
        cg.ld_a(0x42);
        assert_eq!(cg.rom(), &[0x3E, 0x42]);
    }

    #[test]
    fn test_ld_hl() {
        let mut cg = CodeGen::new();
        cg.ld_hl(0x1234);
        assert_eq!(cg.rom(), &[0x21, 0x34, 0x12]);
    }

    #[test]
    fn test_call_and_ret() {
        let mut cg = CodeGen::new();
        cg.label("start");
        cg.call("func");
        cg.halt();
        cg.label("func");
        cg.ret();
        cg.resolve_fixups();

        // CALL should point to func at offset 4
        assert_eq!(cg.rom()[0], 0xCD);
        assert_eq!(cg.rom()[1], 0x04);
        assert_eq!(cg.rom()[2], 0x00);
    }
}
