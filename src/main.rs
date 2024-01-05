mod state;

use wgpu::SurfaceError;
use state::State;

fn main() {
    pollster::block_on(run());
}

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    keyboard::{KeyCode, PhysicalKey}
};
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoopWindowTarget;

pub async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().with_title("Learn WGPU").build(&event_loop).unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut state = State::new(window).await;

    event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window.id() => if !state.input(event) {
            handle_window_event(event, elwt, &mut state)
        },
        Event::AboutToWait => {
            // RedrawRequested will only trigger once unless we manually
            // request it.
            state.window.request_redraw()
        },
        _ => {}
    }).expect("Event loop failed");
}

fn handle_window_event(event: &WindowEvent, elwt: &EventLoopWindowTarget<()>, state: &mut State) {
    match event {
        WindowEvent::RedrawRequested => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                Err(e) => eprintln!("{:?}", e),
            }
        },
        WindowEvent::Resized(physical_size) => {
            state.resize(*physical_size);
        },
        // WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
        //     // new_inner_size is &&mut so we have to dereference it twice
        //     state.resize(PhysicalSize::from((state.size.width*scale_factor, state.size.height*scale_factor)));
        // },
        WindowEvent::CloseRequested
        | WindowEvent::KeyboardInput {
            event:
            KeyEvent {
                state: ElementState::Pressed,
                physical_key: PhysicalKey::Code(KeyCode::Escape),
                ..
            },
            ..
        } => elwt.exit(),
        _ => {}
    }
}
