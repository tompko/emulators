#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::Read;
use clap::{Arg, App};

mod cpu;

fn main() {
    let matches = App::new("CHIP 8 Emulator")
                        .version(crate_version!())
                        .author("tompko  <tompko@gmail.com>")
                        .about("Emulates the CHIP8 language")
                        .arg(Arg::with_name("INPUT")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1))
                        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let rom = read_rom(input_file);

    let cpu = cpu::Cpu::new(rom);

    cpu.Run();
}

fn read_rom(filename: &str) -> Vec<u8> {
    let mut buffer = Vec::new();

    match File::open(filename) {
        Ok(ref mut file) => {
            file.read_to_end(&mut buffer).unwrap();
        },
        Err(err) => {
            println!("chip8: cannot open '{}': {}", filename, err);
            std::process::exit(-1);
        }
    }


    return buffer
}
