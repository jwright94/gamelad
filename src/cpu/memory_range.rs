pub struct MemoryRange {
    low_range: u16,
    high_range: u16,
    size: usize,
}

impl MemoryRange {
    const fn new(low_range: u16, high_range: u16) -> MemoryRange {
        MemoryRange {
            low_range: low_range,
            high_range: high_range,
            size: (high_range-low_range) as usize
        }
    }

    const fn in_range(&self, addr: u16) -> bool {
        addr >= self.low_range && addr <= self.high_range
    }

    const ROM_BANK_0: MemoryRange = MemoryRange::new(0x0000, 0x3fff);
    const ROM_BANK_N: MemoryRange = MemoryRange::new(0x4000, 0x7FFF);
    const VRAM: MemoryRange = MemoryRange::new(0x8000, 0x9FFF);
    const EXTERNAL_RAM: MemoryRange = MemoryRange::new(0xA000, 0xBFFF);
    const WRAM: MemoryRange = MemoryRange::new(0xC000, 0xCFFF);
    const WRAM_SWITCH: MemoryRange = MemoryRange::new(0xD000, 0xDFFF);
    const ECHO_RAM: MemoryRange = MemoryRange::new(0xE000, 0xFDFF);
    const SPRITE_ATTRIBUTE_TABLE: MemoryRange = MemoryRange::new(0xFE00, 0xFE9F);
    const UNUSED: MemoryRange = MemoryRange::new(0xFEA0, 0xFEFF);
    const IO_REGISTERS: MemoryRange = MemoryRange::new(0xFF00, 0xFF7F);
    const HRAM: MemoryRange = MemoryRange::new(0xFF80, 0xFFFE);
}