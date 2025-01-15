use std::sync::Arc;

use winit::{application::ApplicationHandler, event::{ElementState, KeyEvent, WindowEvent}, event_loop::ActiveEventLoop, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowId}};

use crate::state::State;

#[derive(Default)]
pub enum App {
    #[default]
    Paused,
    Resumed(Arc<Window>, State)
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(
            Window::default_attributes().with_title("Learn WGPU")
        ).unwrap());
        let state = State::new(window.clone());
        *self = Self::Resumed(window, state);
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        *self = Self::Paused;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match self {
            App::Paused => false,
            App::Resumed(window, state) => state.input(&event),
        };
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    ..
                },
                ..
            } => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                match self {
                    App::Paused => todo!(),
                    App::Resumed(window, state) => {
                        state.render();
                        window.request_redraw();
                    }
                }
            },
            WindowEvent::Resized(physical_size) => {
                match self {
                    App::Paused => todo!(),
                    App::Resumed(window, state) => state.resize(physical_size),
                }
            },

            _ => (),
        }
    }
}
