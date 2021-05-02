use crate::cpu::{ unmake_u16, make_u16 };
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[derive(FromPrimitive, Debug)]
pub enum MemoryBankControllerType {
    RomOnly = 0x00,
    Mbc1 = 0x01,
    Mbc1Ram = 0x02,
    Mbc1RamBattery = 0x03,
    Mbc2 = 0x05,
    Mbc2Battery = 0x06,
    RomRam = 0x08,
    RomRamBattery = 0x09,
    Mmm01 = 0x0B,
    Mmm01Ram = 0x0C,
    Mmm01RamBattery = 0x0D,
    Mbc3TimerBattery = 0x0F,
    Mbc3TimerRamBattery = 0x10,
    Mbc3 = 0x11,
    Mbc3Ram = 0x12,
    Mbc3RamBattery = 0x13,
    Mbc5 = 0x19,
    Mbc5Ram = 0x1A,
    Mbc5RamBattery = 0x1B,
    Mbc5Rumble = 0x1C,
    Mbc5RumbleRam = 0x1D,
    Mbc5RumbleRamBattery = 0x1E,
    Mbc6 = 0x20,
    Mbc7SensorRumbleRamBattery = 0x22,
    PocketCamera = 0xFC,
    BandaiTama5 = 0xFD,
    Huc3 = 0xFE,
    Huc1RamBattery = 0xFF
}

pub trait MemoryBankController {
    fn write(&mut self, addr: u16, value: u8);
    fn read(&mut self, addr: u16) -> u8;

    fn write_u16(&mut self, addr: u16, value: u16){
        let (lo, hi) = unmake_u16(value);

        self.write(addr, lo);
        self.write(addr + 1, hi);
    }

    fn read_u16(&mut self, addr: u16) -> u16 {
        let lo = self.read(addr);
        let hi = self.read(addr+1);

        let ret = make_u16(lo, hi);
        println!("read {:#08x} from {:#08x}", ret, addr);
        ret
    }
}
