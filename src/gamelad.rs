

use crate::cpu::mbc::Mbc1;
use crate::cpu::mbc::{ MemoryBankController, MemoryBankControllerType};
use crate::cpu::CPU;

use std::fs;
use std::io;

//#[derive(MemoryBankController)]
pub struct Gamelad {
    cpu: CPU,
    memory: Vec<u8>,
    rom: Vec<u8>,
    mbc_type: MemoryBankControllerType
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

        let mut cycle = 0;

        self.reset();

        let mut mbc = Mbc1::new(&mut self.rom);
        
        let mut buf = [0_u8; 1];

        while !self.cpu.is_stopped() {
            cycle += 1;
            println!("cycle #{}", cycle);
            println!("Initial State {}", self.cpu);

            //let mbc: &mut dyn MemoryBankController = &mut mbc;
            
            self.cpu.step(&mut mbc);
            
            //let mut input = String::new();
            //std::io::stdin().read_line(&mut input);

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
        
        /*
        mbc.write(0xff05, 0x00);
        mbc.write(0xff06, 0x00);
        mbc.write(0xff07, 0x00);
        mbc.write(0xff10, 0x80);
        mbc.write(0xff11, 0xBF);
        mbc.write(0xff12, 0xF3);
        mbc.write(0xff14, 0xBF);
        mbc.write(0xff16, 0x3F);
        mbc.write(0xff17, 0x00);
        mbc.write(0xff19, 0x00);

        mbc.write(0xFF05, 0x00);
        mbc.write(0xFF06, 0x00);
        mbc.write(0xFF07, 0x00);
        mbc.write(0xFF10, 0x80);
        mbc.write(0xFF11, 0xBF);
        mbc.write(0xFF12, 0xF3);
        mbc.write(0xFF14, 0xBF);
        mbc.write(0xFF16, 0x3F);
        mbc.write(0xFF17, 0x00);
        mbc.write(0xFF19, 0xBF);
        mbc.write(0xFF1A, 0x7F);
        mbc.write(0xFF1B, 0xFF);
        mbc.write(0xFF1C, 0x9F);
        mbc.write(0xFF1E, 0xBF);
        mbc.write(0xFF20, 0xFF);
        mbc.write(0xFF21, 0x00);
        mbc.write(0xFF22, 0x00);
        mbc.write(0xFF23, 0xBF);
        mbc.write(0xFF24, 0x77);
        mbc.write(0xFF25, 0xF3);
        mbc.write(0xFF26, 0xF1);//-GB, 0xF0-SGB ; NR52
        mbc.write(0xFF40, 0x91);
        mbc.write(0xFF42, 0x00);
        mbc.write(0xFF43, 0x00);
        mbc.write(0xFF45, 0x00);
        mbc.write(0xFF47, 0xFC);
        mbc.write(0xFF48, 0xFF);
        mbc.write(0xFF49, 0xFF);
        mbc.write(0xFF4A, 0x00);
        mbc.write(0xFF4B, 0x00);
        mbc.write(0xFFFF, 0x00);
        */
    }
}