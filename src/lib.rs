mod renderer;

use winit::{
  event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode},
  event_loop::{ControlFlow, EventLoop},
};

pub async fn run() {
  env_logger::init();

  let event_loop = EventLoop::new();
  let renderer = pollster::block_on(renderer::Renderer::new(&event_loop));
  event_loop.run(move |event, _, control_flow| match event {
    Event::WindowEvent {
      ref event,
      window_id,
    } if window_id == renderer.window.id() => match event {
      WindowEvent::CloseRequested
      | WindowEvent::KeyboardInput {
        input:
        KeyboardInput {
          state: ElementState::Pressed,
          virtual_keycode: Some(VirtualKeyCode::Escape),
          ..
        },
        ..
      } => *control_flow = ControlFlow::Exit,
      _ => {}
    },
    _ => {}
  });
}

const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 720.0;