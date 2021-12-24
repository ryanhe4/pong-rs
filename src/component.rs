use log::info;
use wgpu::util::DeviceExt;

use crate::buffer;
use crate::buffer::Vertex;

pub struct Point {
  pub x: f32,
  pub y: f32,
}

pub struct Rectangle {
  pub vertex_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer,
  pub num_indices: u32,
  point: Point,
  width: u32,
  height: u32,
}

impl Rectangle {
  pub fn new(device: &wgpu::Device, point: Point, width: u32, height: u32, color: Option<f32>) -> Self {
    // info!("Create Object Component::Renderer::New() width: {}, height: {}", width, height);
    let vertices: &[Vertex] = &[
      Vertex { position: [(point.x - 640.0) / 640.0, (-point.y + 360.0) / 360.0, 0.0], color: [0.5, 0.5, 0.5] }, // A
      Vertex { position: [(point.x - 640.0) / 640.0, (-point.y - height as f32 + 360.0) / 360.0, 0.0], color: [0.5, 0.5, 0.5] }, // B
      Vertex { position: [(point.x + width as f32 - 640.0) / 640.0, (-point.y - height as f32 + 360.0) / 360.0, 0.0], color: [0.5, 0.5, 0.5] }, // C
      Vertex { position: [(point.x + width as f32 - 640.0) / 640.0, (-point.y + 360.0) / 360.0, 0.0], color: [0.5, 0.5, 0.5] }, // D
    ];
    let vertex_buffer = device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsages::VERTEX,
      }
    );
    let index_buffer = device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(buffer::RECT_INDICES),
        usage: wgpu::BufferUsages::INDEX,
      }
    );
    let num_indices = buffer::RECT_INDICES.len() as u32;

    Self {
      vertex_buffer,
      index_buffer,
      num_indices,
      point,
      width,
      height,
    }
  }
}