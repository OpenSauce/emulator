use anyhow::{Context, Result};
use clap::{Arg, Command};

mod emulator;

fn main() -> Result<()> {
    let matches = Command::new("Gameboy Emulator")
        .version("0.1.0")
        .author("OpenSauce")
        .about("A simple Game Boy emulator written in Rust")
        .arg(Arg::new("rom").required(true).help("Path to the ROM file"))
        .get_matches();

    let rom_path = matches
        .get_one::<String>("rom")
        .context("ROM path is required")?;

    let mut gameboy = emulator::Emulator::new();
    gameboy.load_rom(rom_path).context("Failed to load ROM")?;

    gameboy.run();
    Ok(())
}
