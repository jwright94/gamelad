

use crate::cpu::CPU;
use std::fs;

pub struct Gamelad {
    cpu: CPU,
    rom: Vec<u8>
}

impl Gamelad {
    pub fn new(filename: &str) -> Gamelad {
        println!("loading {}..", filename);

        let memory = fs::read(filename)
            .expect("Could not load binary");
    
        println!("Binary Size:\n{}", memory.len());

        let mut cpu = CPU::new();

        cpu.pc = 0x0100;

        Gamelad {
            cpu: cpu,
            rom: memory,
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

        self.rom[0xff05] = 0x00;
        self.rom[0xff06] = 0x00;
        self.rom[0xff07] = 0x00;
        self.rom[0xff10] = 0x80;
        self.rom[0xff11] = 0xBF;
        self.rom[0xff12] = 0xF3;
        self.rom[0xff14] = 0xBF;
        self.rom[0xff16] = 0x3F;
        self.rom[0xff17] = 0x00;
        self.rom[0xff19] = 0x00;

        self.rom[0xFF05] = 0x00;
        self.rom[0xFF06] = 0x00;
        self.rom[0xFF07] = 0x00;
        self.rom[0xFF10] = 0x80;
        self.rom[0xFF11] = 0xBF;
        self.rom[0xFF12] = 0xF3;
        self.rom[0xFF14] = 0xBF;
        self.rom[0xFF16] = 0x3F;
        self.rom[0xFF17] = 0x00;
        self.rom[0xFF19] = 0xBF;
        self.rom[0xFF1A] = 0x7F;
        self.rom[0xFF1B] = 0xFF;
        self.rom[0xFF1C] = 0x9F;
        self.rom[0xFF1E] = 0xBF;
        self.rom[0xFF20] = 0xFF;
        self.rom[0xFF21] = 0x00;
        self.rom[0xFF22] = 0x00;
        self.rom[0xFF23] = 0xBF;
        self.rom[0xFF24] = 0x77;
        self.rom[0xFF25] = 0xF3;
        self.rom[0xFF26] = 0xF1;//-GB, 0xF0-SGB ; NR52
        self.rom[0xFF40] = 0x91;
        self.rom[0xFF42] = 0x00;
        self.rom[0xFF43] = 0x00;
        self.rom[0xFF45] = 0x00;
        self.rom[0xFF47] = 0xFC;
        self.rom[0xFF48] = 0xFF;
        self.rom[0xFF49] = 0xFF;
        self.rom[0xFF4A] = 0x00;
        self.rom[0xFF4B] = 0x00;
        self.rom[0xFFFF] = 0x00;

    }
}