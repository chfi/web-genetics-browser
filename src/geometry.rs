use bytemuck::{Pod, Zeroable};
use std::{borrow::Cow, mem};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub pos: [f32; 2],
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self { pos: [x, y] }
    }
}

pub fn example_vertices() -> Vec<Vertex> {
    let mut data: Vec<Vertex> = Vec::new();

    for i in 0..200 {
        let x = ((i as f32) / 100.0) - 1.0;
        // let y = ((x * 2.0).sin() / 2.0) + 0.5;
        let y = (x * 2.0).sin();

        let top = Vertex::new(x, y - 0.01);
        let left = Vertex::new(x - 0.01, y + 0.01);
        let right = Vertex::new(x + 0.01, y + 0.01);

        data.push(top);
        data.push(left);
        data.push(right);
    }

    data
}

// pub fn example_vertices() -> Vec<Vertex> {
//     vec![
//         Vertex { pos: [
//     ]
// }
