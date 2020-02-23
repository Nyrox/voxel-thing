#![deny(unused_must_use)]

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
use std::path::PathBuf;
use std::ptr;
use std::time;

use cgmath::prelude::*;
use cgmath::{Deg, Matrix4, PerspectiveFov, Quaternion, Rad, Vector3};

use std::sync::{Arc, Mutex};

use gl::types::*;

mod world;
use world::*;

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
            far: 1000.0,
        },
    );
    camera.transform.position.z = -3.0;
    camera.transform.position.y = chunk::CHUNK_HEIGHT as f32 - 4.0;
    let mut world = World::empty();

    for x in (-10)..10 {
        for z in (-10)..10 {
            world.gen_chunk((x, z));
        }
    }

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
    let mut renderer = world::WorldRenderer::new(camera.clone());

    let mut velocity = Vector3::<f32>::zero();

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

        let movement_speed = 15.0;
        let movement_cap = 2.0;

        let mut input_x = false;
        let mut input_z = false;

        if opengl.window.get_key(Key::W) == Action::Press {
            velocity += camera.transform.forward() * movement_speed * delta_time;
            input_z = true;
        }
        if opengl.window.get_key(Key::S) == Action::Press {
            velocity += camera.transform.forward() * -movement_speed * delta_time;
            input_z = true;
        }
        if opengl.window.get_key(Key::A) == Action::Press {
            velocity += camera.transform.right() * -movement_speed * delta_time;
            input_x = true;
        }
        if opengl.window.get_key(Key::D) == Action::Press {
            velocity += camera.transform.right() * movement_speed * delta_time;
            input_x = true;
        }
        if opengl.window.get_key(Key::Space) == Action::Press {
            // camera.transform.position += camera.transform.up() * movement_speed * delta_time;
            velocity.y = 10.0;
        }
        if opengl.window.get_key(Key::LeftShift) == Action::Press {
            camera.transform.position += camera.transform.down() * movement_speed * delta_time;
        }

        {
            let fwd = velocity.mul_element_wise(Vector3::new(1.0, 0.0, 0.0));
            let rgt = velocity.mul_element_wise(Vector3::new(0.0, 0.0, 1.0));

            let fwd = if fwd.magnitude() < 0.000000001 {
                Vector3::zero()
            } else {
                fwd.normalize() * fwd.magnitude().min(movement_cap)
            };

            let rgt = if rgt.magnitude() < 0.000000001 {
                Vector3::zero()
            } else {
                rgt.normalize() * rgt.magnitude().min(movement_cap)
            };

            velocity = fwd + rgt + Vector3::new(0.0, velocity.y, 0.0);
        }

        // if !input_z {
        //     velocity -= 0.3 * velocity.mul_element_wise(camera.transform.forward());
        // }
        // if !input_x {
        //     velocity -= 0.3 * velocity.mul_element_wise(camera.transform.right());
        // }
        if opengl.window.get_key(Key::Q) == Action::Press {
            camera.transform.rotation =
                Quaternion::from_angle_y(Deg(-30.0 * delta_time)) * camera.transform.rotation;
        }
        if opengl.window.get_key(Key::E) == Action::Press {
            camera.transform.rotation =
                Quaternion::from_angle_y(Deg(30.0 * delta_time)) * camera.transform.rotation;
        }
        if opengl.window.get_key(Key::Y) == Action::Press {
            camera.transform.rotation = Quaternion::from_axis_angle(camera.transform.right(), Deg(15.0 * delta_time))
				// Quaternion::from_angle_x(Deg(15.0 * delta_time))

				* camera.transform.rotation;
        }
        if opengl.window.get_key(Key::X) == Action::Press {
            camera.transform.rotation =
                Quaternion::from_axis_angle(camera.transform.right(), Deg(-15.0 * delta_time))
                    * camera.transform.rotation;
            // Quaternion::from_angle_x(Deg(-15.0 * delta_time)) * camera.transform.rotation;
        }

        // physics

        const gravity: f32 = -9.810;

        velocity += delta_time * gravity * Vector3::new(0.0, 1.0, 0.0);
        camera.transform.position.y -= 1.0;

        // x
        {
            camera.transform.position.x += velocity.x * delta_time;
            let voxel_coords = world.voxel_from_world(camera.transform.position);

            if world.voxel(voxel_coords).is_solid() {
                camera.transform.position.x -= velocity.x * delta_time;
                velocity.x = 0.0;
                dbg!("Collision X");
            }
        }

        // y
        {
            camera.transform.position.y += velocity.y * delta_time;
            let voxel_coords = world.voxel_from_world(camera.transform.position);

            if world.voxel(voxel_coords).is_solid() {
                camera.transform.position.y -= velocity.y * delta_time;
                velocity.y = 0.0;
                // dbg!("Collision Y");
            }
        }

        // y
        {
            camera.transform.position.z += velocity.z * delta_time;
            let voxel_coords = world.voxel_from_world(camera.transform.position);

            if world.voxel(voxel_coords).is_solid() {
                camera.transform.position.z -= velocity.z * delta_time;
                velocity.z = 0.0;
                dbg!("Collision Z");
            }
        }
        camera.transform.position.y += 1.0;

        renderer.camera = camera.clone();

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

            dirt.bind(0);
            world.render(&renderer);

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
        }

        opengl.window.swap_buffers();
    }
}
