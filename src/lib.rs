mod renderer;
mod buffer;

use env_logger::Env;
use log::info;
use winit::{
  event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode},
  event_loop::{ControlFlow, EventLoop},
};

pub async fn run() {
  let env = Env::default()
    .filter_or("MY_LOG_LEVEL", "pong")
    .write_style_or("MY_LOG_STYLE", "info");

  env_logger::init_from_env(env);

  let event_loop = EventLoop::new();
  let mut renderer = pollster::block_on(renderer::Renderer::new(&event_loop));
  event_loop.run(move |event, _, control_flow|
    match event {
      Event::WindowEvent {
        ref event,
        window_id,
      } if window_id == renderer.window.id() =>
        if !renderer.input(event) {
          //input Event
          match event {
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
            WindowEvent::Resized(physical_size) => {
              renderer.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
              // new_inner_size is &&mut so we have to dereference it twice
              renderer.resize(**new_inner_size);
            }
            _ => {}
          }
        },
      Event::RedrawRequested(_) => {
        renderer.update();
        match renderer.render() {
          Ok(_) => {}
          // Reconfigure the surface if lost
          Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
          // The system is out of memory, we should probably quit
          Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
          // All other errors (Outdated, Timeout) should be resolved by the next frame
          Err(e) => eprintln!("{:?}", e),
        }
      }
      Event::MainEventsCleared => {
        // RedrawRequested will only trigger once, unless we manually
        // request it.
        renderer.window.request_redraw();
      }
      _ => {}
    });
}

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;