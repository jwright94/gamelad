
use crate::cpu::mbc::MemoryBankController;
use std::fmt;

pub mod registers;
pub mod opcodes;
pub mod mbc;

// L/H registers reversed, c9 instruction lands on wrong addr 49922 instead of 1113 (0x459)

pub struct CPU {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub flags: u8,
    pub pc: u16,
    pub cycle_delay: u8,
    pub ime: u8,
    pub sp: u16,

    stopped: bool
}

fn make_u16(lo: u8, hi:u8) -> u16 {
    let l : u16 = lo.into();
    let h : u16 = hi.into();
    
    //println!("LO {:#08x}", lo);
    //println!("HI {:#08x}", hi);

    l | (h << 8)
}

fn unmake_u16(value: u16) -> (u8, u8) {
    let lo: u8 = (value & 0x00FF) as u8;
    let hi: u8 = ((value & 0xFF00) >> 8) as u8;
    
    //println!("{:#08x}", value);
    //println!("LO {:#08x}", lo);
    //println!("HI {:#08x}", hi);

    (lo, hi)
}

impl CPU {
    pub fn new() -> CPU {
        CPU { 
            a: 0, f: 0,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0,
            flags: 0,
            pc: 0,
            cycle_delay: 0,

            ime: 0,
            sp: 0xfffe,
            stopped: false
        }
    }

    pub fn is_stopped(&mut self) -> bool {
        self.stopped
    }

    pub fn fetch(&mut self, data: &mut dyn MemoryBankController) -> u8 {
        let result = data.read(self.pc);
        self.pc += 1;
        result
    }

    pub fn fetch_u16(&mut self, data: &mut dyn MemoryBankController) -> u16 {
        let lo = self.fetch(data);
        let hi = self.fetch(data);

        make_u16(lo, hi)
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        /*
        write!(f, "PC: {:#04x} SP: {} A: {} B: {} C: {} D: {} E: {} H: {} L: {} F: {}", 
            self.pc, self.sp, self.a, self.b, self.c, self.d, self.e, self.h, self.l, self.f)*/
        write!(f, 
            "PC: {:#06x} SP: {:#06x} AF: {:#06x} BC: {:#06x} DE: {:#06x} HL: {:#06x}", 
            self.pc, self.sp, self.get_af(), self.get_bc(), self.get_de(), self.get_hl())
    }
}

impl CPU {
    pub fn get_af(&self) -> u16 {
        make_u16(self.a, self.f)
    }

    pub fn get_hl(&self) -> u16 {
        make_u16(self.h, self.l)
    }

    pub fn get_bc(&self) -> u16 {
        make_u16(self.b, self.c)
    }

    pub fn get_de(&self) -> u16 {
        make_u16(self.d, self.e)
    }

    pub fn set_af(&mut self, value: u16) {
        let (lo, hi) = unmake_u16(value);
        self.a = lo;
        self.f = hi;
    }

    pub fn set_hl(&mut self, value: u16) {
        let (lo, hi) = unmake_u16(value);
        self.h = lo;
        self.l = hi;
    }

    pub fn set_bc(&mut self, value: u16) {
        let (lo, hi) = unmake_u16(value);
        self.b = lo;
        self.c = hi;
    }

    pub fn set_de(&mut self, value: u16) {
        let (lo, hi) = unmake_u16(value);
        self.d = lo;
        self.e = hi;
    }

    pub const FLAG_ZERO: u8 = 1 << 0x07;
    pub const FLAG_SUBTRACT: u8 = 1 << 0x06;
    pub const FLAG_HALF_CARRY: u8 = 1 << 0x05;
    pub const FLAG_CARRY: u8 = 1 << 0x04;
    pub const FLAG_ALL: u8 = CPU::FLAG_ZERO | CPU::FLAG_SUBTRACT | CPU::FLAG_HALF_CARRY | CPU::FLAG_CARRY;

    #[inline(always)]
    pub fn set_flag(&mut self, flag: u8) {
        self.f |= flag;
    }

    #[inline(always)]
    pub fn unset_flag(&mut self, flag: u8) {
        self.f &= !flag;
    }

    pub fn read_flag(&self, flag: u8) -> bool {
        (self.f & flag) > 0
    }

    pub fn alu_add(&mut self, a: u8, b: u8) -> u8 {
        
        self.unset_flag(CPU::FLAG_HALF_CARRY | CPU::FLAG_CARRY | CPU::FLAG_SUBTRACT | CPU::FLAG_ZERO);

        let result: u16 = (a as u16) + (b as u16);

        if (((a & 0x0f) + (b & 0x0f)) & 0xf0) > 0 {
            self.set_flag(CPU::FLAG_HALF_CARRY);
        }

        if result & 0x0100 == 0x0100 {
            self.set_flag(CPU::FLAG_CARRY);
        }

        if self.a == 0 {
            self.set_flag(CPU::FLAG_ZERO);
        }
        
        (result & 0xff) as u8
    }

    pub fn alu_sub(&mut self, a: u8, b: u8) -> u8 {
        let twos_compliment = !b + 1;
        
        let result = self.alu_add(a, twos_compliment);
        self.set_flag(CPU::FLAG_SUBTRACT);
        result
    }

    pub fn alu_inc(&mut self, a: u8) -> u8 {
        let result = self.alu_add(a, 1);

        if result == 0 {
            self.set_flag(CPU::FLAG_ZERO);
        }

        result
    }

    pub fn alu_dec(&mut self, a: u8) -> u8 {
        let result = self.alu_sub(a, 1);
        
        if result == 0 {
            self.set_flag(CPU::FLAG_ZERO);
        }

        result
    }

    pub fn alu_inc_u16(&self, a: u16) -> u16 {
        a.wrapping_add(1)
    }

    pub fn alu_dec_u16(&self, a: u16) -> u16 {
        a.wrapping_sub(1)
    }
}