use super::byteorder::{ByteOrder, LittleEndian};

const PRG_PAGE_SIZE: usize = 16*1024;
const CHR_PAGE_SIZE: usize = 8*1024;

pub struct Cart {
    cart_rom: Box<[u8]>,
    prg_rom_offsets: Vec<(usize, usize)>,
    chr_rom_offsets: Vec<(usize, usize)>,
    // We can have two prg rom pages mapped at any time
    // or the same one twice if we have only 1
    active_prg_pages: [usize; 2],
}

impl Cart {
    pub fn new(rom: Box<[u8]>) -> Result<Cart, &'static str> {
        if rom[0..4] != [0x4e, 0x45, 0x53, 0x1A] {
            return Err("Bad magic bytes in rom")
        }

        let prg = rom[4];
        let chr = rom[5];
        let mapper_lo = rom[6];
        let mapper_hi = rom[7];
        let mut c = Cart{
            cart_rom: rom,
            prg_rom_offsets: Vec::new(),
            chr_rom_offsets: Vec::new(),
            active_prg_pages: [0, 0],
        };

        let mut start = 16;
        for _ in 0..prg {
            c.prg_rom_offsets.push((start,start+PRG_PAGE_SIZE));
            start += PRG_PAGE_SIZE;
        }

        for _ in 0..chr {
            c.chr_rom_offsets.push((start, start+CHR_PAGE_SIZE));
            start += CHR_PAGE_SIZE;
        }

        if mapper_lo != 0 || mapper_hi != 0 {
            return Err("Unrecognised mapper");
        }

        Ok(c)
    }

    pub fn read_prg_byte(&self, pgr_page: usize, offset: u16) -> u8 {
        let page_index = self.active_prg_pages[pgr_page];
        let base_offset = self.prg_rom_offsets[page_index];

        self.cart_rom[base_offset.0 + offset as usize]
    }

    pub fn read_prg_word(&self, pgr_page: usize, offset: u16) -> u16 {
        let page_index = self.active_prg_pages[pgr_page];
        let base_offset = self.prg_rom_offsets[page_index];
        let addr = base_offset.0 + offset as usize;

        LittleEndian::read_u16(&self.cart_rom[addr..])
    }
}
