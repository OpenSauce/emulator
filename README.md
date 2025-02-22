# GameBoy Emulator

A work-in-progress GameBoy emulator written in Rust.

## Useful Links
- [Pan Docs](https://gbdev.io/pandocs/) – Official GameBoy hardware documentation.
- [How to Write an Emulator in Rust](https://gbdev.io/hardware/DMG01) – A guide to building your own emulator in Rust.
- [Cheatsheet](https://gbdev.io/pandocs/CPU_Instructions.html) – A quick reference for CPU instructions.
- [CPU Breakdown](https://rgbds.gbdev.io/docs/v0.9.1) – Detailed documentation of CPU operations.

## Getting Started

### Prerequisites
- **Rust:** Install the latest stable version from [rust-lang.org](https://www.rust-lang.org/tools/install).

### Running the Emulator
Launch the emulator with a GameBoy ROM:
```bash
cargo run -- path/to/game.gb
```

### Testing
Run the test suite, which includes a set of GameBoy test ROMs:
```bash
cargo test
```

## Contributing
Contributions are welcome! Please review our [CONTRIBUTING](CONTRIBUTING.md) guidelines for details on our code of conduct and the process for submitting pull requests.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

### Third-Party Licenses
- The test ROMs in `roms/` are from [Christoph Sprenger's Game Boy tests](https://github.com/CTSRD-CHRIS/gb-tests) and are licensed under the MIT License. See [LICENSE-THIRD-PARTY](LICENSE-THIRD-PARTY).

## Acknowledgements
Special thanks to the GameBoy development community for the wealth of documentation and resources that have helped shape this project.
