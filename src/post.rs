use super::camera::*;
use cgmath::prelude::*;
use cgmath::Vector2;
use graphics::*;

use super::read_file_contents;
use gl::types::GLvoid;

use std::mem;
use std::ptr;

#[derive(Debug, Clone, Copy)]
pub struct ClipVertex {
    position: Vector2<f32>,
}

impl ClipVertex {
    const fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vector2::new(x, y),
        }
    }
}

use crate::post::ClipVertex as CP;

// Vertices to draw a fullscreen rectangle in clip space
pub const FRAME_RECT: [CP; 6] = [
    CP::new(-1.0, 1.0),
    CP::new(1.0, 1.0),
    CP::new(-1.0, -1.0),
    CP::new(-1.0, -1.0),
    CP::new(1.0, 1.0),
    CP::new(1.0, -1.0),
];

pub struct SkyScatterShader {
    pub shader: graphics::Shader,
    vao: u32,
}

impl SkyScatterShader {
    pub fn load() -> Self {
        let shader = Shader::new();
        shader
            .attach(
                &read_file_contents("assets/shaders/post/post.vert"),
                gl::VERTEX_SHADER,
            )
            .unwrap();
        shader
            .attach(
                &read_file_contents("assets/shaders/post/sky_scatter.fs"),
                gl::FRAGMENT_SHADER,
            )
            .unwrap();
        shader.compile().unwrap();

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::CreateVertexArrays(1, &mut vao);
            gl::CreateBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::NamedBufferData(
                vbo,
                (mem::size_of::<ClipVertex>() * FRAME_RECT.len()) as isize,
                FRAME_RECT.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                0,
                mem::size_of::<ClipVertex>() as i32,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
        }
        SkyScatterShader { shader, vao }
    }
}

// expects scene color to be bound to slot 0 and depth to slot 1
pub fn scatter_sky(scatter: &SkyScatterShader, camera: &Camera, gTime: i32) {
    scatter.shader.bind();

    scatter
        .shader
        .setUniform("inverseView", camera.get_view_matrix().invert().unwrap());
    scatter.shader.setUniform(
        "inverseProj",
        camera.get_projection_matrix().invert().unwrap(),
    );
    scatter
        .shader
        .setUniform("nearPlane", camera.projection.near);
    scatter.shader.setUniform("farPlane", camera.projection.far);
    scatter.shader.setUniform("iTime", gTime);
    scatter.shader.setUniform(
        "cameraPosition",
        camera.transform.position.to_homogeneous().truncate(),
    );
    unsafe {
        gl::BindVertexArray(scatter.vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
        gl::BindVertexArray(0);
    }
}
