use emulator::gba::gba::Gba;
use wgpu;

fn main() {
    let emulator = Gba::new();
    emulator.test();
    emulator.framebuffer.test();
    println!("wgpu installed!")
}
