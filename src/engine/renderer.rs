use winit::{
  event_loop::EventLoop,
  window::Window,
  window::WindowBuilder,
};

use log::{info};
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::event::{WindowEvent};
use crate::engine::component::*;
use crate::engine::{buffer, WIDTH, HEIGHT};
use crate::engine::component::{Point, Rectangle};

// Renderer Struct Interface
pub struct Renderer {
  surface: wgpu::Surface,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  pub(crate) size: winit::dpi::PhysicalSize<u32>,
  pub window: Window,
  render_pipeline: wgpu::RenderPipeline,
  render_util: RenderUtil,
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
      source: wgpu::ShaderSource::Wgsl(include_str!("../shader/shader.wgsl").into()),
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
    let render_util = RenderUtil::new();
    Self {
      window,
      surface,
      device,
      queue,
      config,
      size,
      render_pipeline,
      render_util,
    }
  }

  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);
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
    let rect: Rectangle = Rectangle::new(&self.device, Point { x: 0.0, y: 360.0 }, 1280, 1, None);
    let rect2: Rectangle = Rectangle::new(&self.device, Point { x: 640.0, y: 0.0 }, 1, 720, None);
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
      // 1.Layout??? ????????? pipeline??? ??????.
      render_pass.set_pipeline(&self.render_pipeline);
      // 2. render pass??? ?????? ????????? vertex_buffer??? ??????

      // 1. confirm Scene
      // 2. input
      // 3. update
      // 4. draw
      RenderUtil::draw_rect(&mut render_pass, &rect);
      RenderUtil::draw_rect(&mut render_pass, &rect2);
    }

    // submit will accept anything that implements IntoIter
    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }
}

struct RenderUtil {
  is_draw_grid: bool,
}

impl RenderUtil {
  pub fn new() -> Self {
    is_draw_grid = true;
    Self { is_draw_grid }
  }

  pub fn draw_rect<'a>(render_pass: &mut wgpu::RenderPass<'a>, rect: &'a Rectangle) {
    render_pass.set_vertex_buffer(0, rect.vertex_buffer.slice(..));
    render_pass.set_index_buffer(rect.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..rect.num_indices, 0, 0..1);
  }

  pub fn draw_grid<'a>(&mut self, render_pass: &mut wgpu::RenderPass<'a>) {
    if self.is_draw_grid {
      // todo!(draw index instances)
    }
  }

  pub fn switch_draw_grid(&mut self) {
    self.is_draw_grid = !self.is_draw_grid;
  }
}