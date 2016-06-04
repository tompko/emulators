const RAM_START: u16 = 0x0000;
const RAM_LENGTH: u16 = 0x0800;
const RAM_END: u16 = RAM_START + RAM_LENGTH - 1;

const PRG_ROM_1_START: u16 = 0x8000;
const PRG_ROM_1_LENGTH: u16 = 0x4000;
const PRG_ROM_1_END: u16 = PRG_ROM_1_START + PRG_ROM_1_LENGTH - 1;

const PRG_ROM_2_START: u16 = 0xc000;
const PRG_ROM_2_END: u16 = 0xffff;

#[derive(Debug)]
pub enum Addr {
    Ram(u16),
    PrgRom1(u16),
    PrgRom2(u16),
}

pub fn map_addr(addr: u16) -> Addr {
    match addr {
        RAM_START ... RAM_END => Addr::Ram(addr - RAM_START),
        PRG_ROM_1_START ... PRG_ROM_1_END => Addr::PrgRom1(addr - PRG_ROM_1_START),
        PRG_ROM_2_START ... PRG_ROM_2_END => Addr::PrgRom2(addr - PRG_ROM_2_START),
        _ => panic!("Unrecognised cpu address: {:#x}", addr)
    }
}
