//! Core Z80 code generation engine
//!
//! Provides the fundamental emit/label/fixup machinery for building Z80 ROMs.

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

/// Configuration for ROM generation
#[derive(Clone)]
pub struct RomConfig {
    /// Origin address (where ROM starts in memory)
    pub org: u16,
    /// Stack top address
    pub stack_top: u16,
    /// RAM start address
    pub ram_start: u16,
}

impl Default for RomConfig {
    fn default() -> Self {
        Self {
            org: 0x0000,
            stack_top: 0x3FFF,
            ram_start: 0x2000,
        }
    }
}

/// Core code generator
pub struct CodeGen {
    rom: Vec<u8>,
    labels: HashMap<String, u16>,
    fixups: Vec<(usize, String)>,
    config: RomConfig,
    unique_counter: u32,
}

impl CodeGen {
    /// Create a new code generator with default config
    pub fn new() -> Self {
        Self::with_config(RomConfig::default())
    }

    /// Create a new code generator with custom config
    pub fn with_config(config: RomConfig) -> Self {
        Self {
            rom: Vec::new(),
            labels: HashMap::new(),
            fixups: Vec::new(),
            config,
            unique_counter: 0,
        }
    }

    /// Get the ROM configuration
    pub fn config(&self) -> &RomConfig {
        &self.config
    }

    /// Get current emit position (address)
    pub fn pos(&self) -> u16 {
        self.config.org + self.rom.len() as u16
    }

    /// Get current ROM size in bytes
    pub fn size(&self) -> usize {
        self.rom.len()
    }

    /// Generate a unique label name
    pub fn unique_label(&mut self, prefix: &str) -> String {
        self.unique_counter += 1;
        format!("_{}_{}", prefix, self.unique_counter)
    }

    // ========== Core Emit Functions ==========

    /// Emit raw bytes
    pub fn emit(&mut self, bytes: &[u8]) {
        self.rom.extend_from_slice(bytes);
    }

    /// Emit a single byte
    pub fn emit_byte(&mut self, b: u8) {
        self.rom.push(b);
    }

    /// Emit a 16-bit word (little-endian)
    pub fn emit_word(&mut self, word: u16) {
        self.rom.push(word as u8);
        self.rom.push((word >> 8) as u8);
    }

    /// Emit a null-terminated string
    pub fn emit_string(&mut self, s: &str) {
        for b in s.bytes() {
            self.rom.push(b);
        }
        self.rom.push(0);
    }

    /// Emit a string without null terminator
    pub fn emit_string_raw(&mut self, s: &str) {
        for b in s.bytes() {
            self.rom.push(b);
        }
    }

    // ========== Label Management ==========

    /// Define a label at current position
    pub fn label(&mut self, name: &str) {
        self.labels.insert(name.to_string(), self.pos());
    }

    /// Check if a label exists
    pub fn has_label(&self, name: &str) -> bool {
        self.labels.contains_key(name)
    }

    /// Get label address (if defined)
    pub fn get_label(&self, name: &str) -> Option<u16> {
        self.labels.get(name).copied()
    }

    /// Record a fixup for later resolution (emits placeholder word)
    pub fn fixup(&mut self, name: &str) {
        self.fixups.push((self.rom.len(), name.to_string()));
        self.emit_word(0); // Placeholder
    }

    /// Resolve all fixups - call after all code is emitted
    pub fn resolve_fixups(&mut self) {
        for (offset, name) in &self.fixups {
            let addr = *self.labels.get(name).unwrap_or_else(|| {
                panic!("Undefined label: {}", name)
            });
            self.rom[*offset] = addr as u8;
            self.rom[*offset + 1] = (addr >> 8) as u8;
        }
    }

    /// Emit a relative jump offset (for JR, DJNZ)
    /// target_label must already be defined
    pub fn emit_relative(&mut self, target_label: &str) {
        let target = *self.labels.get(target_label).unwrap_or_else(|| {
            panic!("Undefined label for relative jump: {}", target_label)
        });
        let current = self.pos() + 1; // +1 because offset is from after the offset byte
        let offset = (target as i32 - current as i32) as i8;
        self.emit_byte(offset as u8);
    }

    // ========== Output ==========

    /// Get the raw ROM bytes
    pub fn rom(&self) -> &[u8] {
        &self.rom
    }

    /// Get mutable access to ROM bytes (for patching relative jumps, etc.)
    pub fn rom_mut(&mut self) -> &mut Vec<u8> {
        &mut self.rom
    }

    /// Write ROM to binary file
    pub fn write_bin(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(&self.rom)?;
        Ok(())
    }

    /// Write ROM as Intel HEX format
    pub fn write_hex(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;

        for (i, chunk) in self.rom.chunks(16).enumerate() {
            let addr = self.config.org + (i * 16) as u16;
            let len = chunk.len() as u8;

            // Calculate checksum
            let mut checksum: u8 = len;
            checksum = checksum.wrapping_add((addr >> 8) as u8);
            checksum = checksum.wrapping_add(addr as u8);
            // Record type 00 = data
            for &b in chunk {
                checksum = checksum.wrapping_add(b);
            }
            checksum = (!checksum).wrapping_add(1);

            write!(file, ":{:02X}{:04X}00", len, addr)?;
            for &b in chunk {
                write!(file, "{:02X}", b)?;
            }
            writeln!(file, "{:02X}", checksum)?;
        }

        // End of file record
        writeln!(file, ":00000001FF")?;
        Ok(())
    }
}

impl Default for CodeGen {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_basic() {
        let mut cg = CodeGen::new();
        cg.emit(&[0x00, 0x01, 0x02]);
        assert_eq!(cg.size(), 3);
        assert_eq!(cg.rom(), &[0x00, 0x01, 0x02]);
    }

    #[test]
    fn test_emit_word() {
        let mut cg = CodeGen::new();
        cg.emit_word(0x1234);
        assert_eq!(cg.rom(), &[0x34, 0x12]); // Little-endian
    }

    #[test]
    fn test_labels_and_fixups() {
        let mut cg = CodeGen::new();
        cg.emit(&[0xC3]); // JP
        cg.fixup("target");
        cg.emit(&[0x00]); // NOP
        cg.label("target");
        cg.emit(&[0xC9]); // RET
        cg.resolve_fixups();

        // JP should point to address 4 (org=0, JP=1, addr=2, NOP=1, target=4)
        assert_eq!(cg.rom()[1], 0x04);
        assert_eq!(cg.rom()[2], 0x00);
    }

    #[test]
    fn test_unique_label() {
        let mut cg = CodeGen::new();
        let l1 = cg.unique_label("loop");
        let l2 = cg.unique_label("loop");
        assert_ne!(l1, l2);
    }
}
