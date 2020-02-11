#![deny(unused_must_use)]
#![feature(duration_extras)]

extern crate cgmath;
extern crate gl;
extern crate glfw;
extern crate graphics;
extern crate libc;
extern crate serde_json;

mod camera;
mod post;
mod rectangle_shape;
mod transform;

use camera::Camera;
use glfw::{Action, Context, Key};
use graphics::{Mesh, OpenGLContext, Shader, Texture2D};
use rectangle_shape::RectangleShape;
use transform::Transform;

use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::PathBuf;
use std::ptr;
use std::time;

use cgmath::prelude::*;
use cgmath::{Deg, Matrix4, PerspectiveFov, Quaternion, Rad, Vector3};

use std::sync::{Arc, Mutex};

use gl::types::*;

pub fn read_file_contents(filename: &str) -> String {
    let mut f = File::open(filename).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    buffer
}

fn load_shader(path: PathBuf) -> Shader {
    let file = read_file_contents(path.to_str().unwrap());
    let data: serde_json::Value = serde_json::from_str(&file).expect("Failed to read shader file.");

    let shader = Shader::new();
    shader
        .attach(
            &read_file_contents(data["vertex"].as_str().unwrap()),
            gl::VERTEX_SHADER,
        )
        .unwrap();
    shader
        .attach(
            &read_file_contents(data["fragment"].as_str().unwrap()),
            gl::FRAGMENT_SHADER,
        )
        .unwrap();
    shader.compile().unwrap();
    return shader;
}

struct BasicVertex {
    position: Vector3<f32>,
}

impl BasicVertex {
    pub fn from_pos(x: f32, y: f32, z: f32) -> BasicVertex {
        BasicVertex {
            position: Vector3::new(x, y, z),
        }
    }
}

fn main() {
    let mut opengl = OpenGLContext::new();
    let r = RectangleShape::new(1280.0, 720.0);
    println!("{:?}", r);

    // Create framebuffer
    let mut fbo: GLuint = 0;
    let mut cl: GLuint = 0;
    let mut depth: GLuint = 0;

    unsafe {
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        gl::GenTextures(1, &mut depth);
        gl::BindTexture(gl::TEXTURE_2D, depth);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::DEPTH_COMPONENT as i32,
            1280,
            720,
            0,
            gl::DEPTH_COMPONENT,
            gl::FLOAT,
            ptr::null_mut(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::CLAMP_TO_BORDER as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::CLAMP_TO_BORDER as i32,
        );
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::DEPTH_ATTACHMENT,
            gl::TEXTURE_2D,
            depth,
            0,
        );

        gl::GenTextures(1, &mut cl);
        gl::BindTexture(gl::TEXTURE_2D, cl);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            1280,
            720,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            ::std::ptr::null_mut(),
        );

        gl::TextureParameteri(cl, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TextureParameteri(cl, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            cl,
            0,
        );
    }

    let voxelshade = Shader::new();
    voxelshade
        .attach(
            &read_file_contents("assets/shaders/voxel.vs"),
            gl::VERTEX_SHADER,
        )
        .unwrap();
    voxelshade
        .attach(
            &read_file_contents("assets/shaders/voxel.fs"),
            gl::FRAGMENT_SHADER,
        )
        .unwrap();
    voxelshade
        .attach(
            &read_file_contents("assets/shaders/voxel.gs"),
            gl::GEOMETRY_SHADER,
        )
        .unwrap();
    voxelshade.compile().unwrap();
    voxelshade.bind();

    let shader = Shader::new();
    shader
        .attach(
            &read_file_contents("assets/shaders/tri.vs"),
            gl::VERTEX_SHADER,
        )
        .unwrap();
    shader
        .attach(
            &read_file_contents("assets/shaders/tri.fs"),
            gl::FRAGMENT_SHADER,
        )
        .unwrap();
    shader.compile().unwrap();
    shader.bind();

    let draw_fullscreen = Shader::new();
    draw_fullscreen
        .attach(
            &read_file_contents("assets/shaders/draw_fullscreen.vs"),
            gl::VERTEX_SHADER,
        )
        .unwrap();
    draw_fullscreen
        .attach(
            &read_file_contents("assets/shaders/draw_fullscreen.fs"),
            gl::FRAGMENT_SHADER,
        )
        .unwrap();
    draw_fullscreen.compile().unwrap();
    draw_fullscreen.bind();

    // let shader = load_shader(PathBuf::from("assets/shaders/schema.json"));

    let mesh = Mesh::load_ply(PathBuf::from("assets/meshes/cube.ply"));
    println!("{:?}", mesh);

    let dirt = Texture2D::new(PathBuf::from("assets/textures/dirt.jpg"), gl::SRGB8);

    let albedo = Texture2D::new(
        PathBuf::from("assets/textures/harshbricks-albedo.png"),
        gl::SRGB8_ALPHA8,
    );
    let roughness = Texture2D::new(
        PathBuf::from("assets/textures/harshbricks-roughness.png"),
        gl::R8,
    );
    let normal = Texture2D::new(
        PathBuf::from("assets/textures/harshbricks-normal.png"),
        gl::RGB8,
    );

    let mut camera = Camera::new(
        Transform::default(),
        PerspectiveFov {
            fovy: Rad::from(Deg(75.0)),
            aspect: 1280.0 / 720.0,
            near: 0.1,
            far: 100.0,
        },
    );
    camera.transform.position.z = -3.0;

    shader.setUniform("perspective", camera.get_projection_matrix());
    #[derive(Copy, Clone, Debug)]
    enum VoxelType {
        VOID,
        GROUND,
    }

    #[derive(Copy, Clone, Debug)]
    struct Voxel {
        voxel_type: VoxelType,
    }

    impl Voxel {
        pub fn void() -> Voxel {
            Voxel {
                voxel_type: VoxelType::VOID,
            }
        }
    }

    const CHUNK_DIM: u32 = 8;
    const CHUNK_HEIGHT: u32 = 16;
    const CHUNK_N_VOXELS: usize = (CHUNK_DIM * CHUNK_DIM * CHUNK_HEIGHT) as usize;

    struct Chunk {
        voxels: [Voxel; CHUNK_N_VOXELS],
    }

    impl Chunk {
        pub fn void() -> Chunk {
            Chunk {
                voxels: [Voxel::void(); CHUNK_N_VOXELS],
            }
        }

        pub fn iter_mut<F>(&mut self, mut f: F)
        where
            F: FnMut((u32, u32, u32), &mut Voxel),
        {
            for z in 0..CHUNK_DIM {
                for y in 0..CHUNK_HEIGHT {
                    for x in 0..CHUNK_DIM {
                        f((x, y, z), self.voxel_mut(x, y, z))
                    }
                }
            }
        }

        pub fn iter<F>(&self, f: F)
        where
            F: Fn((u32, u32, u32), &Voxel),
        {
            for z in 0..CHUNK_DIM {
                for y in 0..CHUNK_HEIGHT {
                    for x in 0..CHUNK_DIM {
                        f((x, y, z), self.voxel(x, y, z))
                    }
                }
            }
        }

        pub fn voxel_mut(&mut self, x: u32, y: u32, z: u32) -> &mut Voxel {
            &mut self.voxels[(z + y * CHUNK_DIM + x * (CHUNK_DIM * CHUNK_DIM)) as usize]
        }

        pub fn voxel(&self, x: u32, y: u32, z: u32) -> &Voxel {
            &self.voxels[(z + y * CHUNK_DIM + x * (CHUNK_DIM * CHUNK_DIM)) as usize]
        }

        pub fn gen_flat(ground: u32) -> Chunk {
            let mut chunk = Chunk::void();

            chunk.iter_mut(|(_, _, z), v| {
                *v = if z < ground {
                    Voxel {
                        voxel_type: VoxelType::GROUND,
                    }
                } else {
                    Voxel::void()
                }
            });

            chunk
        }

        pub fn gen_vertex_array(&mut self) -> (u32, u32) {
            let mut vertices = Vec::new();
            let mut indices = Vec::new();

            let mut i = 0;
            self.iter_mut(|(x, y, z), _| {
                // top plane
                vertices.push(BasicVertex::from_pos(
                    x as f32 + 0.0,
                    y as f32 + 0.0,
                    z as f32 + 1.0,
                ));
                vertices.push(BasicVertex::from_pos(
                    x as f32 + 1.0,
                    y as f32 + 0.0,
                    z as f32 + 1.0,
                ));
                vertices.push(BasicVertex::from_pos(
                    x as f32 + 1.0,
                    y as f32 + 1.0,
                    z as f32 + 1.0,
                ));
                vertices.push(BasicVertex::from_pos(
                    x as f32 + 0.0,
                    y as f32 + 1.0,
                    z as f32 + 1.0,
                ));
                // bottom plane
                vertices.push(BasicVertex::from_pos(
                    x as f32 + 0.0,
                    y as f32 + 0.0,
                    z as f32 + 0.0,
                ));
                vertices.push(BasicVertex::from_pos(
                    x as f32 + 1.0,
                    y as f32 + 0.0,
                    z as f32 + 0.0,
                ));
                vertices.push(BasicVertex::from_pos(
                    x as f32 + 1.0,
                    y as f32 + 1.0,
                    z as f32 + 0.0,
                ));
                vertices.push(BasicVertex::from_pos(
                    x as f32 + 0.0,
                    y as f32 + 1.0,
                    z as f32 + 0.0,
                ));

                indices.append(&mut vec![
                    // top face
                    i + 0,
                    i + 1,
                    i + 2,
                    i + 0,
                    i + 2,
                    i + 3,
                    // bottom face
                    i + 4,
                    i + 5,
                    i + 6,
                    i + 4,
                    i + 6,
                    i + 7,
                    // left face
                    i + 0,
                    i + 4,
                    i + 3,
                    i + 3,
                    i + 4,
                    i + 7,
                    // right face
                    i + 1,
                    i + 2,
                    i + 6,
                    i + 1,
                    i + 6,
                    i + 5,
                    // front face
                    i + 0,
                    i + 1,
                    i + 5,
                    i + 0,
                    i + 5,
                    i + 4,
                    // back face
                    i + 2,
                    i + 3,
                    i + 7,
                    i + 2,
                    i + 7,
                    i + 6,
                ]);

                i += 8;
            });

            let mut vao = 0;
            let mut vbo = 0;
            let mut ebo = 0;

            unsafe {
                gl::CreateVertexArrays(1, &mut vao);
                gl::CreateBuffers(1, &mut vbo);
                gl::CreateBuffers(1, &mut ebo);

                gl::BindVertexArray(vao);

                gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                gl::NamedBufferData(
                    vbo,
                    (mem::size_of::<BasicVertex>() * vertices.len()) as isize,
                    vertices.as_ptr() as *const GLvoid,
                    gl::STATIC_DRAW,
                );

                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                gl::NamedBufferData(
                    ebo,
                    (mem::size_of::<u32>() * indices.len()) as isize,
                    indices.as_ptr() as *const GLvoid,
                    gl::STATIC_DRAW,
                );

                // Positions
                gl::VertexAttribPointer(
                    0,
                    3,
                    gl::FLOAT,
                    0,
                    mem::size_of::<BasicVertex>() as i32,
                    ptr::null(),
                );
                gl::EnableVertexAttribArray(0);

                gl::BindVertexArray(0);
            }

            (vao, indices.len() as u32)
        }
    }

    let mut chunk = Chunk::gen_flat(32);
    let (chunk_vao, chunk_indices_len) = chunk.gen_vertex_array();

    camera.transform.position.y = 0.0;

    let command_buffer = Arc::new(Mutex::new(Vec::new()));
    {
        let command_buffer = command_buffer.clone();
        let _input_thread = ::std::thread::spawn(move || loop {
            let mut buffer = String::new();
            ::std::io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read line from stdin");
            command_buffer
                .lock()
                .expect("Failed to get lock on command buffer.")
                .push(buffer);
        });
    }

    let scatter = post::SkyScatterShader::load();

    let mut last_time = time::Instant::now();
    let mut total_time = 0.0;
    while !opengl.window.should_close() {
        let time = time::Instant::now();
        let delta_time = time.duration_since(last_time).subsec_millis() as f32 / 1000.0;
        total_time += delta_time * 1000.0;
        last_time = time;


        {
            let mut command_buffer = command_buffer
                .lock()
                .expect("Failed to get lock on command buffer.");
            for command in command_buffer.drain(..) {
                println!("{:?}", command.as_str());
                match command.as_str() {
                    "quit\r\n" => {
                        print!("Shutting down...");
                        opengl.window.set_should_close(true);
                    }
                    _ => {
                        println!("Unknown command: {}", command);
                    }
                }
            }
        }

        opengl.poll_events();
        for (_, event) in glfw::flush_messages(&opengl.events) {
            match event {
                glfw::WindowEvent::Close => {
                    opengl.window.set_should_close(true);
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    opengl.window.set_should_close(true);
                }
                _ => {}
            }
        }

        let movement_speed = 1.0;

        if opengl.window.get_key(Key::W) == Action::Press {
            camera.transform.position += camera.transform.forward() * movement_speed * delta_time;
        }
        if opengl.window.get_key(Key::S) == Action::Press {
            camera.transform.position += camera.transform.forward() * -movement_speed * delta_time;
        }
        if opengl.window.get_key(Key::A) == Action::Press {
            camera.transform.position += camera.transform.right() * -movement_speed * delta_time;
        }
        if opengl.window.get_key(Key::D) == Action::Press {
            camera.transform.position += camera.transform.right() * movement_speed * delta_time;
        }
        if opengl.window.get_key(Key::Space) == Action::Press {
            camera.transform.position += camera.transform.up() * movement_speed * delta_time;
        }
        if opengl.window.get_key(Key::LeftShift) == Action::Press {
            camera.transform.position += camera.transform.down() * movement_speed * delta_time;
        }

        if opengl.window.get_key(Key::Q) == Action::Press {
            camera.transform.rotation =
                Quaternion::from_angle_y(Deg(-15.0 * delta_time)) * camera.transform.rotation;
        }
        if opengl.window.get_key(Key::E) == Action::Press {
            camera.transform.rotation =
                Quaternion::from_angle_y(Deg(15.0 * delta_time)) * camera.transform.rotation;
        }
        if opengl.window.get_key(Key::Y) == Action::Press {
            camera.transform.rotation =
                // Quaternion::from_axis_angle(Vector3::new(1.0, 0.0, 0.0), Deg(15.0 * delta_time))
				Quaternion::from_angle_x(Deg(15.0 * delta_time))

				* camera.transform.rotation;
        }
        if opengl.window.get_key(Key::X) == Action::Press {
            camera.transform.rotation =
                Quaternion::from_angle_x(Deg(-15.0 * delta_time)) * camera.transform.rotation;
        }

        shader.setUniform("view", camera.get_view_matrix());

        albedo.bind(0);
        roughness.bind(1);
        normal.bind(3);
        shader.bind();
        shader.setUniform("gTime", total_time as i32);
        shader.setUniform(
            "cameraPos",
            camera.transform.position.to_homogeneous().truncate(),
        );
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                gl::Enable(gl::DEPTH_TEST);
                gl::DepthFunc(gl::LESS);
                gl::Enable(gl::BLEND);
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            }

            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

            shader.setUniform(
                "model",
                Matrix4::<f32>::from_translation(Vector3::new(4.0, 0.0, 0.0)),
            );
            mesh.draw();

            shader.setUniform("model", Matrix4::<f32>::identity());

            voxelshade.bind();
            voxelshade.setUniform("projection", camera.get_projection_matrix());
            voxelshade.setUniform("view", camera.get_view_matrix());
            voxelshade.setUniform(
                "cameraPos",
                camera.transform.position.to_homogeneous().truncate(),
            );
            voxelshade.setUniform("gTime", total_time as i32);
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            dirt.bind(0);

            gl::BindVertexArray(chunk_vao);
            gl::DrawElements(
                gl::TRIANGLES,
                chunk_indices_len as i32,
                gl::UNSIGNED_INT,
                0 as *const GLvoid,
            );
            gl::BindVertexArray(0);

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl::Disable(gl::DEPTH_TEST);
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }

            scatter.shader.bind();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, cl);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, depth);

            post::scatter_sky(&scatter, &camera, total_time as i32);

            // draw_fullscreen.bind();
            // draw_fullscreen.setUniform(
            // 	"projection",
            // 	Matrix4::from(cgmath::Ortho {
            // 		left: 0.0,
            // 		top: 0.0,
            // 		bottom: 720.0,
            // 		right: 1280.0,
            // 		near: 0.1,
            // 		far: 100.0,
            // 	}),
            // );
            // gl::ActiveTexture(gl::TEXTURE0);
            // gl::BindTexture(gl::TEXTURE_2D, cl);
            // r.draw();
        }

        opengl.window.swap_buffers();
    }
}
