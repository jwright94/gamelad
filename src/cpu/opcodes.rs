
use crate::cpu::mbc::MemoryBankController;
use crate::cpu::CPU;
use crate::cpu::registers::{ Reg8, Reg16, Condition };


impl CPU {
    pub fn step(&mut self, data: &mut dyn MemoryBankController) {
        if self.stopped {
            println!("stopped");
            return;
        }

        let instruction = self.fetch(data);
        println!("current instruction {:#04x}", instruction);

        match instruction {
            0 => {
                // NOP
                println!("NOP");
                self.cycle_delay = 4;
            },

            // JR PC+dd, JR Z, r8 (0x28), 0x38
            0x20 => self.jr(Condition::NZ, data),
            0x30 => self.jr(Condition::NC, data),
            0x28 => self.jr(Condition::Z, data),
            0x38 => self.jr(Condition::C, data),
            0x18 => self.jr(Condition::Always, data),

            // CALL a16
            0xc4 => self.call(Condition::NZ, data),
            0xd4 => self.call(Condition::NC, data),
            0xcc => self.call(Condition::Z, data),
            0xdc => self.call(Condition::C, data),
            0xcd => self.call(Condition::Always, data),

            // Register to Register Loads
            0x40 => self.ld_u8(Reg8::B, Reg8::B),
            0x41 => self.ld_u8(Reg8::B, Reg8::C),
            0x42 => self.ld_u8(Reg8::B, Reg8::D),
            0x43 => self.ld_u8(Reg8::B, Reg8::E),
            0x44 => self.ld_u8(Reg8::B, Reg8::H),
            0x45 => self.ld_u8(Reg8::B, Reg8::L),
            0x47 => self.ld_u8(Reg8::B, Reg8::A),

            0x50 => self.ld_u8(Reg8::D, Reg8::B),
            0x51 => self.ld_u8(Reg8::D, Reg8::C),
            0x52 => self.ld_u8(Reg8::D, Reg8::D),
            0x53 => self.ld_u8(Reg8::D, Reg8::E),
            0x54 => self.ld_u8(Reg8::D, Reg8::H),
            0x55 => self.ld_u8(Reg8::D, Reg8::L),
            0x57 => self.ld_u8(Reg8::D, Reg8::A),

            0x60 => self.ld_u8(Reg8::H, Reg8::B),
            0x61 => self.ld_u8(Reg8::H, Reg8::C),
            0x62 => self.ld_u8(Reg8::H, Reg8::D),
            0x63 => self.ld_u8(Reg8::H, Reg8::E),
            0x64 => self.ld_u8(Reg8::H, Reg8::H),
            0x65 => self.ld_u8(Reg8::H, Reg8::L),
            0x67 => self.ld_u8(Reg8::H, Reg8::A),

            0x48 => self.ld_u8(Reg8::C, Reg8::B),
            0x49 => self.ld_u8(Reg8::C, Reg8::C),
            0x4a => self.ld_u8(Reg8::C, Reg8::D),
            0x4b => self.ld_u8(Reg8::C, Reg8::E),
            0x4c => self.ld_u8(Reg8::C, Reg8::H),
            0x4d => self.ld_u8(Reg8::C, Reg8::L),
            0x4f => self.ld_u8(Reg8::C, Reg8::A),

            0x58 => self.ld_u8(Reg8::E, Reg8::B),
            0x59 => self.ld_u8(Reg8::E, Reg8::C),
            0x5a => self.ld_u8(Reg8::E, Reg8::D),
            0x5b => self.ld_u8(Reg8::E, Reg8::E),
            0x5c => self.ld_u8(Reg8::E, Reg8::H),
            0x5d => self.ld_u8(Reg8::E, Reg8::L),
            0x5f => self.ld_u8(Reg8::E, Reg8::A),

            0x68 => self.ld_u8(Reg8::L, Reg8::B),
            0x69 => self.ld_u8(Reg8::L, Reg8::C),
            0x6a => self.ld_u8(Reg8::L, Reg8::D),
            0x6b => self.ld_u8(Reg8::L, Reg8::E),
            0x6c => self.ld_u8(Reg8::L, Reg8::H),
            0x6d => self.ld_u8(Reg8::L, Reg8::L),
            0x6f => self.ld_u8(Reg8::L, Reg8::A),

            0x78 => self.ld_u8(Reg8::A, Reg8::B),
            0x79 => self.ld_u8(Reg8::A, Reg8::C),
            0x7a => self.ld_u8(Reg8::A, Reg8::D),
            0x7b => self.ld_u8(Reg8::A, Reg8::E),
            0x7c => self.ld_u8(Reg8::A, Reg8::H),
            0x7d => self.ld_u8(Reg8::A, Reg8::L),
            0x7f => self.ld_u8(Reg8::A, Reg8::A),

            // Load Immediate 8bit
            0x06 => self.ld_imm_u8(Reg8::B, data),
            0x16 => self.ld_imm_u8(Reg8::D, data),
            0x26 => self.ld_imm_u8(Reg8::H, data),
            0x0e => self.ld_imm_u8(Reg8::C, data),
            0x1e => self.ld_imm_u8(Reg8::E, data),
            0x2e => self.ld_imm_u8(Reg8::L, data),
            0x3e => self.ld_imm_u8(Reg8::A, data),

            // Load Immediate 16bit
            0x01 => self.ld_imm_u16(Reg16::BC, data),
            0x11 => self.ld_imm_u16(Reg16::DE, data),
            0x21 => self.ld_imm_u16(Reg16::HL, data),
            0x31 => self.ld_imm_u16(Reg16::SP, data),

            // LD R, (HL)
            0x46 => self.ldhl(Reg8::B, data),
            0x56 => self.ldhl(Reg8::D, data),
            0x66 => self.ldhl(Reg8::H, data),
            0x4e => self.ldhl(Reg8::C, data),
            0x5e => self.ldhl(Reg8::E, data),
            0x6e => self.ldhl(Reg8::L, data),
            0x7e => self.ldhl(Reg8::A, data),

            // LD (HL), A
            0x77 => {
                data.write(self.get_hl(), self.a);
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
                self.pc = data.read_u16(self.sp);
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
            
            

            // Write to IO-Port n FF00+n
            0xe0 => {
                let addr: u16 = 0xFF00_u16 + self.fetch(data) as u16;
                data.write(addr, self.a);
                self.cycle_delay = 12;
            },

            // Read from IO-Port n FF00+n
            0xf0 => {
                let addr: u16 = 0xFF00_u16 + self.fetch(data) as u16;
                self.a = data.read(addr);
                self.cycle_delay = 12;
            },

            // Store A at Address
            0xea => {
                let addr = self.fetch_u16(data);
                data.write(addr, self.a);
                self.cycle_delay = 12;
            },
            
            
            // STOP
            0x10 => {
                self.stopped = true;
                self.cycle_delay = 12;
                self.fetch(data);
            },

            // PUSH rr
            0xc5 => self.push(Reg16::BC, data),
            0xd5 => self.push(Reg16::DE, data),
            0xe5 => self.push(Reg16::HL, data),
            0xf5 => self.push(Reg16::AF, data),

            // POP rr
            0xc1 => self.pop(Reg16::BC, data),
            0xd1 => self.pop(Reg16::DE, data),
            0xe1 => self.pop(Reg16::HL, data),
            0xf1 => self.pop(Reg16::AF, data),

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
            
            // LD [DE], A
            0x12 => {
                println!("LD [DE], A");
                let addr = self.get_de();
                data.write(addr, self.a);
                self.cycle_delay = 8;
            },

            // LD [BC], A
            0x02 => {
                println!("LD [DE], A");
                let addr = self.get_bc();
                data.write(addr, self.a);
                self.cycle_delay = 8;
            },

            // LDI [HL], A
            0x22 => {
                println!("LDI [HL], A");
                let hl = self.get_hl();
                data.write(hl, self.a);
                self.set_hl(hl.wrapping_add(1));
                self.cycle_delay = 8;
            },

            // LDI A, [HL]
            0x2a => {
                println!("LDI A, [HL]");
                let hl = self.get_hl();
                self.a = data.read(hl);
                self.set_hl(hl.wrapping_add(1));
                self.cycle_delay = 8;
            },

            // LDD A, [HL]
            0x3a => {
                println!("LDD A, [HL]");
                let hl = self.get_hl();
                self.a = data.read(hl);
                self.set_hl(hl.wrapping_sub(1));
                self.cycle_delay = 8;
            },
            
            // LDD [HL], A
            0x32 => {
                println!("LDD [HL], A");
                let hl = self.get_hl();
                self.a = data.read(hl);
                self.set_hl(hl.wrapping_sub(1));
                self.cycle_delay = 8;
            },

            // LDI A, (DE)
            0x1a => {
                println!("LD A, (DE)");
                let addr = self.get_de();
                self.a = data.read(addr);
                self.cycle_delay = 8;
            },

            // LDI A, (BC)
            0x0a => {
                println!("LD A, (BC)");
                let addr = self.get_bc();
                self.a = data.read(addr);
                self.cycle_delay = 8;
            },

            // INC r8
            0x04 => self.inc_r8(Reg8::B),
            0x14 => self.inc_r8(Reg8::D),
            0x24 => self.inc_r8(Reg8::H),
            0x0c => self.inc_r8(Reg8::C),
            0x1c => self.inc_r8(Reg8::E),
            0x2c => self.inc_r8(Reg8::L),
            0x3c => self.inc_r8(Reg8::A),

            0x34 => {
                self.unset_flag(CPU::FLAG_ZERO | CPU::FLAG_SUBTRACT | CPU::FLAG_HALF_CARRY);

                self.cycle_delay = 4;
                
                let addr = self.get_hl();
                let value = self.alu_inc(data.read(addr)); 
                data.write(addr, value);
                self.cycle_delay = 12; 
            },

            // DEC r8
            0x05 => self.dec_r8(Reg8::B),
            0x15 => self.dec_r8(Reg8::D),
            0x25 => self.dec_r8(Reg8::H),
            0x0d => self.dec_r8(Reg8::C),
            0x1d => self.dec_r8(Reg8::E),
            0x2d => self.dec_r8(Reg8::L),
            0x3d => self.dec_r8(Reg8::A),

            0x35  => {
                self.unset_flag(CPU::FLAG_ZERO | CPU::FLAG_SUBTRACT | CPU::FLAG_HALF_CARRY | CPU::FLAG_CARRY);

                let addr = self.get_hl();
                let value = self.alu_dec(data.read(addr)); 
                data.write(addr, value);
                self.cycle_delay = 12;
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
                    0xb6 => { self.a |= data.read(self.get_hl()); self.cycle_delay = 8 },
                    _ => unreachable!()
                };

                if self.a == 0 {
                    self.set_flag(CPU::FLAG_ZERO);
                }
            },

            // ADD
            0x80 => self.add(Reg8::B),
            0x81 => self.add(Reg8::C),
            0x82 => self.add(Reg8::D),
            0x83 => self.add(Reg8::E),
            0x84 => self.add(Reg8::H),
            0x85 => self.add(Reg8::L),
            0x87 => self.add(Reg8::A),

            0xc6 => self.add_imm(data),

            // SUB
            0x90 => self.sub(Reg8::B),
            0x91 => self.sub(Reg8::C),
            0x92 => self.sub(Reg8::D),
            0x93 => self.sub(Reg8::E),
            0x94 => self.sub(Reg8::H),
            0x95 => self.sub(Reg8::L),
            0x97 => self.sub(Reg8::A),

            0xd6 => self.sub_imm(data),
            
            // XOR
            0xa8 => self.xor(Reg8::B),
            0xa9 => self.xor(Reg8::C),
            0xaa => self.xor(Reg8::D),
            0xab => self.xor(Reg8::E),
            0xac => self.xor(Reg8::H),
            0xad => self.xor(Reg8::L),
            0xaf => self.xor(Reg8::A),

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
                        self.a &= data.read(self.get_hl()); 
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
                self.a = data.read(imm);
                self.cycle_delay = 16;
            },

            _ => {
                panic!("instruction {:#04x} not yet implemented", instruction);
            }
        }
    }

    fn ld_imm_u8(&mut self, dst: Reg8, data: &mut dyn MemoryBankController) {
        println!("LD {:?}, d8", dst);
        let imm = self.fetch(data);

        match dst {
            Reg8::A => self.a = imm,
            Reg8::B => self.b = imm,
            Reg8::C => self.c = imm,
            Reg8::D => self.d = imm,
            Reg8::E => self.e = imm,
            Reg8::L => self.l = imm,
            Reg8::H => self.h = imm,
            Reg8::F => unreachable!(),
        }
        self.cycle_delay = 8;
    }

    fn ld_imm_u16(&mut self, dst: Reg16, data: &mut dyn MemoryBankController) {
        println!("LD {:?}, d16", dst);
        let imm = self.fetch_u16(data);
        
        match dst {
            Reg16::BC => self.set_bc(imm),
            Reg16::DE => self.set_de(imm),
            Reg16::HL => self.set_hl(imm),
            Reg16::SP => self.sp = imm,
            Reg16::AF => unreachable!(),
        }

        self.cycle_delay = 8;
    }

    fn get_r8(&self, register: Reg8) -> u8 {
        match register {
            Reg8::A => self.a,
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::L => self.l,
            Reg8::H => self.h,
            Reg8::F => self.f,
        }
    }

    fn set_r8(&mut self, register: Reg8, value: u8) {
        match register {
            Reg8::A => self.a = value,
            Reg8::B => self.b = value,
            Reg8::C => self.c = value,
            Reg8::D => self.d = value,
            Reg8::E => self.e = value,
            Reg8::L => self.l = value,
            Reg8::H => self.h = value,
            Reg8::F => self.f = value,
        }
    }

    fn get_r16(&self, register: Reg16) -> u16 {
        match register {
            Reg16::BC => self.get_bc(),
            Reg16::DE => self.get_de(),
            Reg16::HL => self.get_hl(),
            Reg16::AF => self.get_af(),
            Reg16::SP => self.sp
        }
    }

    fn set_r16(&mut self, register: Reg16, value: u16) {
        match register {
            Reg16::BC => self.set_bc(value),
            Reg16::DE => self.set_de(value),
            Reg16::HL => self.set_hl(value),
            Reg16::AF => self.set_af(value),
            Reg16::SP => self.sp = value
        };
    }

    fn ld_u8(&mut self, dst: Reg8, src: Reg8) {
        println!("LD {:?}, {:?}", dst, src);
        
        let value = self.get_r8(src);
        self.set_r8(dst, value);

        self.cycle_delay = 4;
    }

    fn check_condition(&self, condition: Condition) -> bool{
        match condition {
            Condition::Always => true,
            Condition::Z => self.read_flag(CPU::FLAG_ZERO),
            Condition::C => self.read_flag(CPU::FLAG_CARRY),
            Condition::NZ => !self.read_flag(CPU::FLAG_ZERO),
            Condition::NC => !self.read_flag(CPU::FLAG_CARRY)
        }
    }

    fn call(&mut self, condition: Condition, data: &mut dyn MemoryBankController){
        let next_addr = self.fetch_u16(data);
        println!("CALL {:?}, {:#04x}", condition, next_addr);
        
        if self.check_condition(condition) {
            self.sp -= 2;
            data.write_u16(self.sp, self.pc);
            self.pc = next_addr;
            self.cycle_delay = 24;
        } else {
            self.cycle_delay = 12;
        }
    }

    fn jr(&mut self, condition: Condition, data: &mut dyn MemoryBankController){
        let bytes = self.fetch(data).to_le_bytes();
        let offset:i32 = i8::from_le_bytes(bytes) as i32;
        println!("JR {:?}", condition);

        if self.check_condition(condition) {
            self.cycle_delay = 12;
            self.pc = (offset + (self.pc as i32)) as u16;
        } else {
            self.cycle_delay = 8;
        }
    }

    fn push(&mut self, register: Reg16, data: &mut dyn MemoryBankController){
        println!("PUSH {:?}", register);
        self.sp -= 2;
        let value = self.get_r16(register);

        data.write_u16(self.sp, value);
        self.cycle_delay = 16;
    }

    fn pop(&mut self, register: Reg16, data: &mut dyn MemoryBankController){
        println!("POP {:?}", register);
        let value = data.read_u16(self.sp);

        self.set_r16(register, value);

        self.sp += 2;
        self.cycle_delay = 12;
    }

    fn inc_r8(&mut self, register: Reg8){
        println!("INC {:?}", register);
        self.unset_flag(CPU::FLAG_ZERO | CPU::FLAG_SUBTRACT | CPU::FLAG_HALF_CARRY);

        self.cycle_delay = 4;
        let current_value = self.get_r8(register);
        let new_value = self.alu_inc(current_value);

        self.set_r8(register, new_value);
    }

    fn dec_r8(&mut self, register: Reg8) {
        println!("DEC {:?}", register);
        self.unset_flag(CPU::FLAG_ZERO | CPU::FLAG_SUBTRACT | CPU::FLAG_HALF_CARRY | CPU::FLAG_CARRY);
        self.cycle_delay = 4;

        let current_value = self.get_r8(register);
        let new_value = self.alu_dec(current_value);

        self.set_r8(register, new_value);
    }

    fn xor(&mut self, register: Reg8){
        let value = self.get_r8(register);
        self.a ^= value;

        self.unset_flag(CPU::FLAG_ALL);

        if self.a == 0 {
            self.set_flag(CPU::FLAG_ZERO);
        }

        self.cycle_delay = 4;
    }

    fn add(&mut self, register: Reg8){
        let value = self.get_r8(register);
        
        self.unset_flag(CPU::FLAG_ALL);

        self.a = self.alu_add(self.a, value);

        self.cycle_delay = 4;
    }

    fn add_imm(&mut self, data: &mut dyn MemoryBankController) {
        let value = self.fetch(data);
        self.a = self.alu_add(self.a, value);
        self.cycle_delay = 8;
    }

    fn sub(&mut self, register: Reg8){
        let value = self.get_r8(register);
        self.a = self.alu_sub(self.a, value);
        self.cycle_delay = 4;
    }

    fn sub_imm(&mut self, data: &mut dyn MemoryBankController) {
        let value = self.fetch(data);
        self.a = self.alu_sub(self.a, value);
        self.cycle_delay = 8;
    }

    fn ldhl(&mut self, register: Reg8, data: &mut dyn MemoryBankController){
        let value = data.read(self.get_hl());
        self.set_r8(register, value);
        self.cycle_delay = 8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FlatMbc {
        memory: Vec<u8>
    }

    impl MemoryBankController for FlatMbc {
        
        fn write(&mut self, a: u16, v: u8) { self.memory[a as usize] = v; }
        fn read(&mut self, a: u16) -> u8 { self.memory[a as usize] }
    }

    fn run_bytecode(bytecode: Vec<u8>) -> CPU {
        
        let length = bytecode.len();

        let mut cpu = CPU::new();
        let mut mbc = FlatMbc{
            memory: bytecode
        };

        while (cpu.pc as usize) < length {
            cpu.step(&mut mbc);
        }

        cpu
    }

    #[test]
    fn nop_does_nothing() {
        let cpu = run_bytecode(vec![0x00, 0x00]);
        assert!(cpu.pc == 2);
        assert!(cpu.a == 0);
    }

    #[test]
    fn ld_imm_16() {
        let cpu = run_bytecode(vec![0x21, 0x34, 0x12]);
        println!("hl = {:#04x}", cpu.get_hl());
        assert_eq!(cpu.get_hl(), 0x1234_u16);
    }

    #[test]
    fn get_and_set_r16_return_same_values() {
        let mut cpu = CPU::new();

        cpu.set_hl(0x1234);
        assert_eq!(0x1234, cpu.get_hl());

        cpu.set_bc(0xbeef);
        assert_eq!(0xbeef, cpu.get_bc());
        
        cpu.set_de(0xcafe);
        assert_eq!(0xcafe, cpu.get_de());
        
        cpu.set_af(0xfade);
        assert_eq!(0xfade, cpu.get_af());
    }

    #[test]
    fn r16_registers_in_correct_order() {
        let mut cpu = CPU::new();

        cpu.set_hl(0x1234);
        println!("h = {:#04x} l = {:#04x}", cpu.h, cpu.l);
        assert_eq!(0x12, cpu.h);
        assert_eq!(0x34, cpu.l);

        cpu.set_bc(0xbeef);
        println!("b = {:#04x} c = {:#04x}", cpu.b, cpu.c);
        assert_eq!(0xbe, cpu.b);
        assert_eq!(0xef, cpu.c);
        
        cpu.set_de(0xcafe);
        println!("d = {:#04x} e = {:#04x}", cpu.d, cpu.e);
        assert_eq!(0xca, cpu.d);
        assert_eq!(0xfe, cpu.e);
        
        cpu.set_af(0xfade);
        println!("a = {:#04x} f = {:#04x}", cpu.a, cpu.f);
        assert_eq!(0xfa, cpu.a);
        assert_eq!(0xde, cpu.f);
    }

    #[test]
    fn inc16_properly_increments_hl() {
        let cpu = run_bytecode(vec![0x21, 0x34, 0x12, 0x23]);

        let hl = cpu.get_hl();

        assert_eq!(hl, 0x1235);
    }

    #[test]
    fn make_u16_and_unmake_u16_are_symmetric() {
        let lo = 0x12;
        let hi = 0x34;

        let a = crate::cpu::make_u16(lo, hi);
        let (l, h) = crate::cpu::unmake_u16(a);

        println!("a = {:#04x}", a);
        println!("l = {:#04x} h = {:#04x}", l, h);
        println!("lo = {:#04x} hi = {:#04x}", lo, hi);

        assert_eq!(lo, l);
        assert_eq!(hi, h);
    }

    #[test]
    fn alu_dec_subtracts_one() {
        let mut cpu = CPU::new();

        let a = cpu.alu_dec(5);

        assert_eq!(a, 4);
    }

    #[test]
    fn alu_dec_ets_correct_flags() {
        let mut cpu = CPU::new();

        let a = cpu.alu_dec(1);

        assert_eq!(a, 0);
        assert!(cpu.read_flag(CPU::FLAG_ZERO));
        assert!(cpu.read_flag(CPU::FLAG_SUBTRACT));
    }

    #[test]
    fn alu_inc_sets_correct_flags() {
        let mut cpu = CPU::new();

        let a = cpu.alu_inc(0x09);

        assert_eq!(a, 0x0a);
        assert!(!cpu.read_flag(CPU::FLAG_ZERO));
        assert!(!cpu.read_flag(CPU::FLAG_SUBTRACT));
    }

}
/*
0x01 => self.ld_imm_u16(Reg16::BC, data),
0x11 => self.ld_imm_u16(Reg16::DE, data),
0x21 => self.ld_imm_u16(Reg16::HL, data),
0x31 => self.ld_imm_u16(Reg16::SP, data),
*/