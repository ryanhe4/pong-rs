use winit::{
  event_loop::EventLoop,
  window::Window,
  window::WindowBuilder,
};

use log::{info};
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::event::{WindowEvent};
use crate::{WIDTH, HEIGHT, buffer};
use crate::buffer::{PENTAGON_INDICES, TRIANGLE_VERTICES};
use crate::component::*;

// Renderer Struct Interface
pub struct Renderer {
  surface: wgpu::Surface,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  pub(crate) size: winit::dpi::PhysicalSize<u32>,
  pub window: Window,
  render_pipeline: wgpu::RenderPipeline,
  rect: Rectangle,
}

impl Renderer {
  // when create window even_loop needed
  pub async fn new(event_loop: &EventLoop<()>) -> Self {
    info!("Initialize Renderer::New()");

    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let size = PhysicalSize { width: WIDTH, height: HEIGHT };
    window.set_inner_size(size);
    window.set_resizable(false);
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance.request_adapter(
      &wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
      }).await.unwrap();

    let (device, queue) = adapter.request_device(
      &wgpu::DeviceDescriptor {
        label: None,
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::default(),
      }, None /*Some(&std::path::Path::new("trace"))*/).await.unwrap();
    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface.get_preferred_format(&adapter).unwrap(),
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Fifo,
    };
    surface.configure(&device, &config);

    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
      label: Some("Shader"),
      source: wgpu::ShaderSource::Wgsl(include_str!("shader/shader.wgsl").into()),
    });

    let render_pipeline_layout =
      device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
      });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Render Pipeline"),
      layout: Some(&render_pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers: &[wgpu::VertexBufferLayout {
          array_stride: std::mem::size_of::<buffer::Vertex>() as wgpu::BufferAddress, // 1.
          step_mode: wgpu::VertexStepMode::Vertex, // 2.
          attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
        }],
      },
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList, // 1.
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw, // 2.
        cull_mode: Some(wgpu::Face::Back),
        // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
        unclipped_depth: false,
        polygon_mode: wgpu::PolygonMode::Fill,
        // Requires Features::CONSERVATIVE_RASTERIZATION
        conservative: false,
      },
      depth_stencil: None,
      multisample: wgpu::MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: "fs_main",
        targets: &[wgpu::ColorTargetState {
          format: config.format,
          blend: Some(wgpu::BlendState::REPLACE),
          write_mask: wgpu::ColorWrites::ALL,
        }],
      }),
      multiview: None,
    });
    let rect: Rectangle = Rectangle::new(&device, Point { x: 0.0, y: 0.0 }, 100, 100, None);

    Self {
      window,
      surface,
      device,
      queue,
      config,
      size,
      render_pipeline,
      rect,
    }
  }

  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);
    }
    if new_size.width != 1024 || new_size.height != 720 {
      info!("Size is not 1024x720");
    }
  }
  pub fn input(&mut self, event: &WindowEvent) -> bool {
    match event {
      /*      WindowEvent::KeyboardInput {
              input:
              KeyboardInput {
                state,
                virtual_keycode: Some(VirtualKeyCode::Space),
                ..
              },
              ..
            } => {
              self.use_color = *state == ElementState::Released;
              true
            }*/
      _ => false,
    }
  }

  pub fn update(&mut self) {}

  pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    let output = self.surface.get_current_texture()?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: Some("Render encoder"),
    });

    {
      // Render Block
      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[
          wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
              load: wgpu::LoadOp::Clear(
                wgpu::Color {
                  r: 0.1,
                  g: 0.2,
                  b: 0.3,
                  a: 1.0,
                }),
              store: true,
            },
          }],
        depth_stencil_attachment: None,
      });
      // 1.Layout을 설정한 pipeline을 지정.
      render_pass.set_pipeline(&self.render_pipeline);
      // 2. render pass에 미리 정의된 vertex_buffer를 입력
      render_pass.set_vertex_buffer(0, self.rect.vertex_buffer.slice(..));
      render_pass.set_index_buffer(self.rect.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

      render_pass.draw_indexed(0..self.rect.num_indices, 0, 0..1);
      // render_pass.draw(0..self.num_vertices, 0..1);
    }

    // submit will accept anything that implements IntoIter
    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }
}