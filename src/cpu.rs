
use std::fmt;

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

    pub fn fetch(&mut self, data: &Vec<u8>) -> u8 {
        let result = data[self.pc as usize];
        self.pc += 1;
        result
    }

    pub fn fetch_u16(&mut self, data: &Vec<u8>) -> u16 {
        let lo = self.fetch(data);
        let hi = self.fetch(data);

        return make_u16(lo, hi);
    }

    pub fn read(&self, addr: u16, data: &Vec<u8>) -> u8 {
        let ret = data[addr as usize];
        println!("read {:#08x} from {:#08x}", ret, addr);
        ret
    }

    pub fn read_u16(&self, addr: u16, data: &Vec<u8>) -> u16 {
        let lo = data[addr as usize];
        let hi = data[(addr+1) as usize];

        let ret = make_u16(lo, hi);
        println!("read {:#08x} from {:#08x}", ret, addr);
        ret
    }

    pub fn step(&mut self, data: &mut Vec<u8>) {
        if self.stopped {
            println!("stopped");
            return;
        }

        let instruction = self.fetch(data);
        println!("current instruction {:#02x}", instruction);

        match instruction {
            0 => {
                // NOP
                println!("NOP");
                self.cycle_delay = 4;
            },

            // JR PC+dd, JR Z, r8 (0x28), 0x38
            0x18 | 0x28 | 0x38 | 0x20 | 0x30 => {
                let bytes = self.fetch(data).to_le_bytes();
                let offset:i32 = i8::from_le_bytes(bytes) as i32;

                if instruction == 0x28 {
                    println!("excuse me?");
                }

                let should_jump = match instruction {
                    0x18 => true,
                    0x28 => self.read_flag(CPU::FLAG_ZERO),
                    0x38 => self.read_flag(CPU::FLAG_CARRY),
                    0x20 => !self.read_flag(CPU::FLAG_ZERO),
                    0x30 => !self.read_flag(CPU::FLAG_CARRY),
                    _ => unreachable!()
                };

                if should_jump {
                    self.cycle_delay = 12;
                    self.pc = (offset + (self.pc as i32)) as u16;
                } else {
                    self.cycle_delay = 8;
                }
            },

            // CALL a16
            0xc4 | 0xd4 | 0xcc | 0xdc | 0xcd => {
                let next_addr = self.fetch_u16(data);

                let should_jump = match instruction {
                    0xcd => true,
                    0xcc => self.read_flag(CPU::FLAG_ZERO),
                    0xdc => self.read_flag(CPU::FLAG_CARRY),
                    0xc4 => !self.read_flag(CPU::FLAG_ZERO),
                    0xd4 => !self.read_flag(CPU::FLAG_CARRY),
                    _ => unreachable!()
                };

                if should_jump {
                    self.sp -= 2;
                    self.store_u16(self.sp, self.pc, data);
                    self.pc = next_addr;
                    self.cycle_delay = 24;
                } else {
                    self.cycle_delay = 12;
                }
            },

            // Register to Register Loads
            0x40..=0x45 | 0x47..=0x4d | 0x4f |
            0x50..=0x55 | 0x57..=0x5d | 0x5f |
            0x60..=0x65 | 0x67..=0x6d | 0x6f |
            0x78..=0x7d | 0x7f => {

                // B, C, D, E, H, L, F, A
                const REGISTER_ARRAY: [char; 8] = ['B', 'C', 'D', 'E', 'H', 'L', 'F', 'A'];
                
                let dst = instruction >> 3 & 7;
                let src = instruction & 7;

                println!("LD {}, {}", 
                    REGISTER_ARRAY[dst as usize], 
                    REGISTER_ARRAY[src as usize]);

                let value = match src {
                    0 => self.b,
                    1 => self.c,
                    2 => self.d,
                    3 => self.e,
                    4 => self.h,
                    5 => self.l,
                    6 => self.f,
                    7 => self.a,
                    _ => unreachable!("oof")
                };

                match dst {
                    0 => self.b = value,
                    1 => self.c = value,
                    2 => self.d = value,
                    3 => self.e = value,
                    4 => self.h = value,
                    5 => self.l = value,
                    6 => self.f = value,
                    7 => self.a = value,
                    _ => unreachable!("oof")
                };
                

                self.cycle_delay = 4;
            },

            // Load Immediate
            // B D H
            // C E L A
            0x06 | 0x16 | 0x26 | 
            0x0e | 0x1e | 0x2e | 0x3e => { 
                println!("LD X, d8");
                let imm = self.fetch(data);

                match instruction { 
                    0x06 => self.b = imm,
                    0x16 => self.d = imm,
                    0x26 => self.h = imm,
                    0x0e => self.c = imm,
                    0x1e => self.e = imm,
                    0x2e => self.l = imm,
                    0x3e => self.a = imm,
                    _ => {}
                }

                self.cycle_delay = 8;
            },

            // LD (HL), A
            0x77 => {
                self.store(self.get_hl(), self.a, data);
                self.cycle_delay = 8;
            },

            // Jump 16bit address
            0xc3 => {
                let addr = self.fetch_u16(data);
                println!("JMP {:#08x}", addr);

                self.pc = addr;
                self.cycle_delay = 16;
            },

            // RET
            0xc9 => {
                self.pc = self.read_u16(self.sp, data);
                self.sp += 2;
                self.cycle_delay = 16;
            },
            
            // Disable Interrupts
            0xf3 => {
                self.ime = 0;
                self.cycle_delay = 4;
            },
            
            // Enable Interrupts
            0xfb => {
                self.ime = 1;
                self.cycle_delay = 4;
            },
            
            // Load Immediate 16bit
            0x01 | 0x11 | 0x21 | 0x31 => {
                let imm = self.fetch_u16(data);

                match instruction {
                    0x01 => self.set_bc(imm),
                    0x11 => self.set_de(imm),
                    0x21 => self.set_hl(imm),
                    0x31 => self.sp = imm,

                    _ => unreachable!("oof")
                }
                self.cycle_delay = 12;
            },

            // Write to IO-Port n FF00+n
            0xe0 => {
                let addr: u16 = 0xFF00_u16 + self.fetch(data) as u16;
                self.store(addr, self.a, data);
                self.cycle_delay = 12;
            },

            // Read from IO-Port n FF00+n
            0xf0 => {
                let addr: u16 = 0xFF00_u16 + self.fetch(data) as u16;
                self.a = self.read(addr, data);
                self.cycle_delay = 12;
            },

            // Store A at Address
            0xea => {
                let addr = self.fetch_u16(data);
                self.store(addr, self.a, data);
                self.cycle_delay = 12;
            },
            
            
            // STOP
            0x10 => {
                self.stopped = true;
                self.cycle_delay = 12;
                self.fetch(data);
            },

            // PUSH rr
            0xc5 | 0xd5 | 0xe5 | 0xf5 => {
                println!("PUSH rr");
                self.sp -= 2;
                let value = match instruction {
                    0xc5 => self.get_bc(),
                    0xd5 => self.get_de(),
                    0xe5 => self.get_hl(),
                    0xf5 => self.get_af(),
                    _ => unreachable!()
                };

                self.store_u16(self.sp, value, data);
                self.cycle_delay = 16;
            },

            // POP rr
            0xc1 | 0xd1 | 0xe1 | 0xf1 => {
                println!("POP rr");
                let value = self.read_u16(self.sp, data);

                match instruction {
                    0xc1 => self.set_bc(value),
                    0xd1 => self.set_de(value),
                    0xe1 => self.set_hl(value),
                    0xf1 => self.set_af(value),
                    _ => unreachable!()
                };

                self.sp += 2;
                self.cycle_delay = 12;
            },

            // INC rr
            0x03 | 0x13 | 0x23 | 0x33 => {
                println!("INC rr");
                match instruction {
                    0x03 => self.set_bc(self.alu_inc_u16(self.get_bc())),
                    0x13 => self.set_de(self.alu_inc_u16(self.get_de())),
                    0x23 => self.set_hl(self.alu_inc_u16(self.get_hl())),
                    0x33 => self.sp += 1,
                    _ => unreachable!()
                };
                self.cycle_delay = 8;
            },

            // DEC rr
            0x0b | 0x1b | 0x2b | 0x3b => {
                println!("DEC rr");
                match instruction {
                    0x0b => self.set_bc(self.alu_dec_u16(self.get_bc())),
                    0x1b => self.set_de(self.alu_dec_u16(self.get_de())),
                    0x2b => self.set_hl(self.alu_dec_u16(self.get_hl())),
                    0x3b => self.sp -= 1,
                    _ => unreachable!()
                };
                self.cycle_delay = 8;
            },

            // LDI A, (HL+)
            0x2a => {
                println!("LDI A, (HL+)");
                let hl = self.get_hl();
                self.a = self.read(hl, data);
                self.set_hl(hl+1);
                self.cycle_delay = 8;
            },

            // LDI A, (HL-)
            0x3a => {
                println!("LDI A, (HL-)");
                let hl = self.get_hl();
                self.a = self.read(hl, data);
                self.set_hl(hl-1);
                self.cycle_delay = 8;
            },

            // INC r8
            0x04 | 0x14 | 0x24 | 0x34 |
            0x0c | 0x1c | 0x2c | 0x3c => {
                self.unset_flag(CPU::FLAG_ZERO | CPU::FLAG_SUBTRACT | CPU::FLAG_HALF_CARRY);

                self.cycle_delay = 4;

                match instruction {
                    0x04 => self.b = self.alu_inc(self.b),
                    0x14 => self.d = self.alu_inc(self.d),
                    0x24 => self.h = self.alu_inc(self.h),
                    0x34 => { 
                        let addr = self.get_hl();
                        let value = self.alu_inc(self.read(addr, data)); 
                        self.store(addr, value, data);
                        self.cycle_delay = 12; 
                    },
                    0x0c => self.c = self.alu_inc(self.c),
                    0x1c => self.e = self.alu_inc(self.e),
                    0x2c => self.l = self.alu_inc(self.l),
                    0x3c => self.a = self.alu_inc(self.a),
                    _ => unreachable!()
                };
            },

            // DEC r8
            0x05 | 0x15 | 0x25 | 0x35 |
            0x0d | 0x1d | 0x2d | 0x3d => {
                self.unset_flag(CPU::FLAG_ZERO | CPU::FLAG_SUBTRACT | CPU::FLAG_HALF_CARRY | CPU::FLAG_CARRY);

                self.cycle_delay = 4;

                match instruction {
                    0x05 => self.b = self.alu_dec(self.b),
                    0x15 => self.d = self.alu_dec(self.d),
                    0x25 => self.h = self.alu_dec(self.h),
                    0x35 => { 
                        let addr = self.get_hl();
                        let value = self.alu_dec(self.read(addr, data)); 
                        self.store(addr, value, data);
                        self.cycle_delay = 12; 
                    },
                    0x0d => self.c = self.alu_dec(self.c),
                    0x1d => self.e = self.alu_dec(self.e),
                    0x2d => self.l = self.alu_dec(self.l),
                    0x3d => self.a = self.alu_dec(self.a),
                    _ => unreachable!()
                };
            },

            // OR
            0xb0..=0xb7 => {
                self.unset_flag(CPU::FLAG_ZERO | CPU::FLAG_SUBTRACT | CPU::FLAG_HALF_CARRY | CPU::FLAG_CARRY);

                self.cycle_delay = 4;

                match instruction {
                    0xb0 => self.a |= self.b,
                    0xb1 => self.a |= self.c,
                    0xb2 => self.a |= self.d,
                    0xb3 => self.a |= self.e,
                    0xb4 => self.a |= self.h,
                    0xb5 => self.a |= self.l,
                    0xb7 => self.a |= self.a,
                    0xb6 => { self.a |= self.read(self.get_hl(), data); self.cycle_delay = 8 },
                    _ => unreachable!()
                };

                if self.a == 0 {
                    self.set_flag(CPU::FLAG_ZERO);
                }
            },

            // AND
            0xa0..=0xa7 | 0xe6 => {
                self.unset_flag(CPU::FLAG_ZERO | CPU::FLAG_SUBTRACT | CPU::FLAG_HALF_CARRY | CPU::FLAG_CARRY);

                self.cycle_delay = 4;

                match instruction {
                    0xa0 => self.a &= self.b,
                    0xa1 => self.a &= self.c,
                    0xa2 => self.a &= self.d,
                    0xa3 => self.a &= self.e,
                    0xa4 => self.a &= self.h,
                    0xa5 => self.a &= self.l,
                    0xa7 => self.a &= self.a,
                    0xa6 => { 
                        self.a &= self.read(self.get_hl(), data); 
                        self.cycle_delay = 8; 
                    },
                    0xe6 => {
                        self.a &= self.fetch(data); 
                        self.cycle_delay = 8; 
                    },
                    _ => unreachable!()
                };

                self.set_flag(CPU::FLAG_HALF_CARRY);

                if self.a == 0 {
                    self.set_flag(CPU::FLAG_ZERO);
                }
            },

            // CP A, r8
            0xfe => {
                self.f = 0;
                
                let imm = self.fetch(data);
                self.a = self.alu_sub(self.a, imm);
                self.set_flag(CPU::FLAG_SUBTRACT);

                if self.a == 0 {
                    self.set_flag(CPU::FLAG_ZERO);
                }
                
                self.cycle_delay = 8;
            },

            // LD A, (a16) 0xfa
            0xfa => {
                let imm = self.fetch_u16(data);
                self.a = self.read(imm, data);
                self.cycle_delay = 16;
            },

            _ => {
                panic!("instruction {:#02x} not yet implemented", instruction);
            }
        }

    }
    
    pub fn store(&self, addr: u16, value: u8, data: &mut Vec<u8>){
        data[addr as usize] = value;
    }

    pub fn store_u16(&self, addr: u16, value: u16, data: &mut Vec<u8>){
        let (lo, hi) = unmake_u16(value);

        data[addr as usize] = lo;
        data[addr as usize + 1] = hi;
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