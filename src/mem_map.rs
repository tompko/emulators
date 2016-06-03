const PRG_ROM_1_START: u16 = 0x8000;
const PRG_ROM_1_LENGTH: u16 = 0x4000;
const PRG_ROM_1_END: u16 = PRG_ROM_1_START + PRG_ROM_1_LENGTH - 1;

const PRG_ROM_2_START: u16 = 0xc000;
const PRG_ROM_2_LENGTH: u16 = 0x4000;
const PRG_ROM_2_END: u16 = 0xffff;

#[derive(Debug)]
pub enum Addr {
    PrgRom1(u16),
    PrgRom2(u16),
}

pub fn map_addr(addr: u16) -> Addr {
    match addr {
        PRG_ROM_1_START ... PRG_ROM_1_END => Addr::PrgRom1(addr - PRG_ROM_1_START),
        PRG_ROM_2_START ... PRG_ROM_2_END => Addr::PrgRom2(addr - PRG_ROM_2_START),
        _ => panic!("Unrecognised cpu address: {:#x}", addr)
    }
}
