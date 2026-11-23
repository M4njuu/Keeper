use emulator::gba::gba::Gba;
use emulator::common::frontend::window::App;
use winit::event_loop::EventLoop;

fn main() {
    let emulator = Gba::new();

    // windowを生成する
    let event_loop = EventLoop::new().unwrap();
    let mut app = App { window: None };
    event_loop.run_app(&mut app).unwrap();
}
