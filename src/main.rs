use emulator::gba::gba::Gba;

fn main() {
    let emulator = Gba::new();
    emulator.test();
    emulator.framebuffer.test();
}
