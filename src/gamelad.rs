

use crate::cpu::mbc::{ MemoryBankController, MemoryBankControllerType};
use crate::cpu::CPU;

use std::fs;

pub struct Gamelad {
    cpu: CPU,
    rom: Vec<u8>,
    memory: Vec<u8>,
    mbc_type: MemoryBankControllerType
}

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

impl Gamelad {
    pub fn new(filename: &str) -> Gamelad {
        println!("loading {}..", filename);

        let rom = fs::read(filename)
            .expect("Could not load binary");
    
        println!("Binary Size:\n{}", rom.len());

        let mut cpu = CPU::new();

        cpu.pc = 0x0100;

        let memory = vec![0x00_u8; 0xffff];
        let mbc_byte = rom[0x0147];
        let mbc_type = match num::FromPrimitive::from_u8(mbc_byte) {
            Some(result) => result,
            _ => panic!("invalid MBC type {}", mbc_byte)
        };

        println!("MBC Type: {:?}", mbc_type);
        Gamelad {
            cpu: cpu,
            rom: rom,
            memory: memory,
            mbc_type: mbc_type
        }
    }

    pub fn run(&mut self){
        self.reset();

        let mut cycle = 0;

        while !self.cpu.is_stopped() {
            cycle += 1;
            println!("cycle #{}", cycle);
            println!("Initial State {}", self.cpu);
            self.cpu.step(&mut self.rom);
            println!();
        }
    }

    pub fn reset(&mut self) {
        // todo: boot from an actual rom
        self.cpu.set_af(0x01b0);
        self.cpu.set_bc(0x0013);
        self.cpu.set_de(0x01d8);
        self.cpu.set_hl(0x014d);
        self.cpu.sp = 0xfffe;

        self.write(0xff05, 0x00);
        self.write(0xff06, 0x00);
        self.write(0xff07, 0x00);
        self.write(0xff10, 0x80);
        self.write(0xff11, 0xBF);
        self.write(0xff12, 0xF3);
        self.write(0xff14, 0xBF);
        self.write(0xff16, 0x3F);
        self.write(0xff17, 0x00);
        self.write(0xff19, 0x00);

        self.write(0xFF05, 0x00);
        self.write(0xFF06, 0x00);
        self.write(0xFF07, 0x00);
        self.write(0xFF10, 0x80);
        self.write(0xFF11, 0xBF);
        self.write(0xFF12, 0xF3);
        self.write(0xFF14, 0xBF);
        self.write(0xFF16, 0x3F);
        self.write(0xFF17, 0x00);
        self.write(0xFF19, 0xBF);
        self.write(0xFF1A, 0x7F);
        self.write(0xFF1B, 0xFF);
        self.write(0xFF1C, 0x9F);
        self.write(0xFF1E, 0xBF);
        self.write(0xFF20, 0xFF);
        self.write(0xFF21, 0x00);
        self.write(0xFF22, 0x00);
        self.write(0xFF23, 0xBF);
        self.write(0xFF24, 0x77);
        self.write(0xFF25, 0xF3);
        self.write(0xFF26, 0xF1);//-GB, 0xF0-SGB ; NR52
        self.write(0xFF40, 0x91);
        self.write(0xFF42, 0x00);
        self.write(0xFF43, 0x00);
        self.write(0xFF45, 0x00);
        self.write(0xFF47, 0xFC);
        self.write(0xFF48, 0xFF);
        self.write(0xFF49, 0xFF);
        self.write(0xFF4A, 0x00);
        self.write(0xFF4B, 0x00);
        self.write(0xFFFF, 0x00);

    }
}

impl MemoryBankController for Gamelad {
    fn write(&mut self, _: u16, _: u8) { todo!() }
    fn read(&mut self, _: u16, _: u8) { todo!() }
}