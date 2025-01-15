mod app;
mod state;

use winit::event_loop::{ControlFlow, EventLoop};

use app::App;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    // let mut state = State::new(window).await;
    event_loop.run_app(&mut App::default()).unwrap();
}
