const PRG_PAGE_SIZE: usize = 16*1024;
const CHR_PAGE_SIZE: usize = 8*1024;

pub struct Cart {
    cart_rom: Box<[u8]>,
    prg_rom_offsets: Vec<(usize, usize)>,
    chr_rom_offsets: Vec<(usize, usize)>,
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
        };

        let mut start = 17;
        for i in 0..prg {
            c.prg_rom_offsets.push((start,start+PRG_PAGE_SIZE));
            start = start + PRG_PAGE_SIZE;
        }

        for i in 0..chr {
            c.chr_rom_offsets.push((start, start+CHR_PAGE_SIZE));
        }

        if mapper_lo != 0 || mapper_hi != 0 {
            return Err("Unrecognised mapper");
        }

        return Ok(c);
    }
}
