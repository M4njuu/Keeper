use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct App {
    pub window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // ウィンドウがまだなければ作る
        if self.window.is_none() {
            // ウィンドウのタイトルを設定する
            let attributes = Window::default_attributes().with_title("Test Window");
            if let Ok(window) = event_loop.create_window(attributes) {
                // 作ったウィンドウをAppに保存する
                self.window = Some(window);
            }
        }
    }

    // ウィンドウのイベントに応じて処理する
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}
