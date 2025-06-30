mod emulators {
    pub mod chip8;
}

use emulators::chip8::run_chip8_emulator;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let rom_path = "../roms/PONG";
    run_chip8_emulator(&sdl_context, rom_path);
}
