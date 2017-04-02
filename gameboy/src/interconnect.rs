use std::collections::HashSet;
use mem_map::*;
use cartridge::Cartridge;
use memory::Memory;
use gpu::Gpu;
use device::Device;
use apu::Apu;
use timer::Timer;
use gamepad::Gamepad;

pub struct Interconnect {
    cartridge: Cartridge,
    gpu: Gpu,
    apu: Apu,
    timer: Timer,
    gamepad: Gamepad,

    internal_ram: Memory,
    high_ram: Memory,
    if_register: u8,
    ie_register: u8,

    serial_transfer_data: u8,
    serial_control: u8,

    trigger_watchpoint: bool,
    pub watchpoints: HashSet<u16>,
}

impl Interconnect {
    pub fn new(cartridge: Cartridge) -> Interconnect {
        Interconnect {
            cartridge: cartridge,
            gpu: Gpu::new(),
            apu: Apu::new(),
            timer: Timer::default(),
            gamepad: Gamepad::new(),

            internal_ram: Memory::new(INTERNAL_RAM_LENGTH),
            high_ram: Memory::new(HIGH_RAM_END),

            if_register: 0,
            ie_register: 0,

            serial_transfer_data: 0,
            serial_control: 0,

            watchpoints: HashSet::new(),
            trigger_watchpoint: false,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            ROM_START...ROM_END => self.cartridge.read_byte(addr - ROM_START),
            VRAM_START...VRAM_END => self.gpu.read_vram(addr - VRAM_START),
            CRAM_START...CRAM_END => unimplemented!(),
            INTERNAL_RAM_START...INTERNAL_RAM_END => {
                self.internal_ram.read_byte(addr - INTERNAL_RAM_START)
            }
            IRAM_ECHO_START...IRAM_ECHO_END => self.internal_ram.read_byte(addr - IRAM_ECHO_START),
            OAM_START...OAM_END => self.gpu.read_oam(addr - OAM_START),
            0xff00 => self.gamepad.read_reg(),

            // TODO - implement serial port (Link cable)
            0xff01 | 0xff02 => 0,

            0xff04...0xff07 => self.timer.read_reg(addr),
            0xff0f => self.if_register,
            0xff10...0xff3f => self.apu.read_reg(addr),
            0xff40...0xff4f => self.gpu.read_reg(addr),
            HIGH_RAM_START...HIGH_RAM_END => self.high_ram.read_byte(addr - HIGH_RAM_START),
            0xffff => self.ie_register,

            // Unused addresses return 0xff for reads
            _ => 0xff,
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        if self.watchpoints.contains(&addr) {
            self.trigger_watchpoint = true;
        }

        match addr {
            ROM_START...ROM_END => self.cartridge.write(addr - ROM_START, val),
            VRAM_START...VRAM_END => self.gpu.write_vram(addr - VRAM_START, val),
            CRAM_START...CRAM_END => unimplemented!(),
            INTERNAL_RAM_START...INTERNAL_RAM_END => {
                self.internal_ram.write_byte(addr - INTERNAL_RAM_START, val)
            }
            IRAM_ECHO_START...IRAM_ECHO_END => {
                self.internal_ram.write_byte(addr - IRAM_ECHO_START, val)
            }
            HIGH_RAM_START...HIGH_RAM_END => self.high_ram.write_byte(addr - HIGH_RAM_START, val),
            OAM_START...OAM_END => self.gpu.write_oam(addr - OAM_START, val),
            0xff00 => self.gamepad.write_reg(val),

            // TODO - implement serial port (Link cable)
            0xff01 => self.serial_transfer_data = val,
            0xff02 => self.serial_control = val,

            0xff04...0xff07 => self.timer.write_reg(addr, val),
            0xff0f => self.if_register = val,
            0xff10...0xff3f => self.apu.write_reg(addr, val),
            0xff40...0xff4b => self.gpu.write_reg(addr, val),
            0xff50 => self.cartridge.disable_boot_rom(),
            0xffff => self.ie_register = val,
            _ => {} // Writes to unused addresses have no effect
        }
    }

    pub fn read_halfword(&self, addr: u16) -> u16 {
        let lsb = self.read_byte(addr);
        let msb = self.read_byte(addr + 1);

        ((msb as u16) << 8) | (lsb as u16)
    }

    pub fn write_halfword(&mut self, addr: u16, val: u16) {
        let lsb = (val & 0xff) as u8;
        let msb = (val >> 8) as u8;

        self.write_byte(addr, lsb);
        self.write_byte(addr + 1, msb);
    }

    pub fn step(&mut self, cycles: u16, device: &mut Device) -> bool {
        let gpu_int = self.gpu.step(cycles, device);
        let timer_int = self.timer.step(cycles, device);
        let gamepad_int = self.gamepad.step(cycles, device);

        let gpu_int_flag = gpu_int & self.ie_register;
        self.if_register |= gpu_int_flag;
        let timer_flag = timer_int & self.ie_register;
        self.if_register |= timer_flag;
        let gamepad_flag = gamepad_int & self.ie_register;
        self.if_register |= gamepad_flag;

        let trigger_watchpoint = self.trigger_watchpoint;
        self.trigger_watchpoint = false;
        trigger_watchpoint
    }

    pub fn get_width(&self) -> usize {
        self.gpu.get_width()
    }

    pub fn get_height(&self) -> usize {
        self.gpu.get_height()
    }
}
