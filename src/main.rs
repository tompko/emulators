extern crate byteorder;
#[macro_use]
extern crate clap;
extern crate rand;
extern crate sdl2;
extern crate time;

use std::fs::File;
use std::io::Read;
use clap::{Arg, App};
use vm::VM;

mod cpu;
mod graphics;
mod input;
mod interconnect;
mod memory;
mod vm;

fn main() {
    let matches = App::new("NES Emulator")
                        .version(crate_version!())
                        .author("tompko  <tompko@gmail.com>")
                        .about("Emulates the NES")
                        .arg(Arg::with_name("INPUT")
                            .help("Sets the NES ROM to use")
                            .required(true)
                            .index(1))
                        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let rom = read_rom(input_file);

    let mut vm = VM::new();

    vm.load_rom(rom);
    vm.run();
}

fn read_rom(filename: &str) -> Vec<u8> {
    let mut buffer = Vec::new();

    match File::open(filename) {
        Ok(ref mut file) => {
            file.read_to_end(&mut buffer).unwrap();
        },
        Err(err) => {
            println!("nesRS: cannot open '{}': {}", filename, err);
            std::process::exit(-1);
        }
    }


    return buffer
}
