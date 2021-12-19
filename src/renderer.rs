use winit::{
  event_loop::EventLoop,
  window::Window,
  window::WindowBuilder,
};

use cgmath::*;
use wgpu::SamplerDescriptor;
use wgpu::util::DeviceExt;
use crate::{WIDTH, HEIGHT};

pub struct Renderer {
  surface: wgpu::Surface,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  size: winit::dpi::PhysicalSize<u32>,
  pub window: Window,
}

impl Renderer {
  pub async fn new(event_loop: &EventLoop<()>) -> Self {
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let size = window.inner_size();
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
    Self {
      window,
      surface,
      device,
      queue,
      config,
      size,
    }
  }
}