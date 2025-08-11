mod gl_buffers;
mod gl_vertex_array;
mod shader;
mod texture;
// mod vertex_array;

// #![feature(portable_simd)]
// use std::simd::{f32x4, Simd};
// use core::str;
// use std::ffi::{c_char, CStr, CString};

use gl::Gl;
use gl_buffers::{IndexBuffer, VertexAttributeDataTypes, VertexAttributeTypes, VertexBuffer};
use gl_vertex_array::VertexArray;
use sdl3::{
    event::Event,
    sys::video::{
        SDL_GL_SetAttribute, SDL_GL_CONTEXT_MAJOR_VERSION, SDL_GL_CONTEXT_MINOR_VERSION,
        SDL_GL_CONTEXT_PROFILE_CORE, SDL_GL_CONTEXT_PROFILE_MASK,
    },
};
use shader::Shader;
use std::ffi::c_void;

pub mod gl {
    #![allow(clippy::all)]
    include!("gl_bindings/bindings.rs");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;
    video_subsystem
        .gl_load_library_default()
        .map_err(|e| e.to_string())?;
    unsafe {
        if !SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 4) {
            panic!("{:?}", sdl3::get_error());
        }
        if !SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 1) {
            panic!("{:?}", sdl3::get_error());
        }
        if !SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE) {
            panic!("{:?}", sdl3::get_error());
        }
    }

    let window = video_subsystem
        .window("rust-sdl3 demo: Window", 800, 600)
        .resizable()
        .position_centered()
        .high_pixel_density()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let gl_context = window.gl_create_context().map_err(|e| e.to_string())?;
    window
        .gl_make_current(&gl_context)
        .map_err(|e| e.to_string())?;

    let gl = Gl::load_with(|s| {
        if let Some(f) = window.subsystem().gl_get_proc_address(s) {
            return f as *const _;
        }
        println!("OpenGL function not found {}", s);
        return std::ptr::null() as *const c_void;
    });

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    let vertices = [
        Vertex::new([0.5, 0.5, 0.0], [1.0, 0.0, 0.0]),
        Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0]),
        Vertex::new([-0.5, -0.5, 0.0], [0.0, 0.0, 1.0]),
        Vertex::new([-0.5, 0.5, 0.0], [1.0, 1.0, 1.0]),
    ];
    let indices = vec![
        0, 1, 3, // first Triangle
        1, 2, 3, // second Triangle
    ];

    let mut vao = VertexArray::new(&gl);
    vao.bind();
    let vbo = VertexBuffer::new(&gl);
    vbo.bind();
    vbo.alloc((vertices.len() * std::mem::size_of::<Vertex>()) as isize);
    println!(
        "{}",
        (vertices.len() * std::mem::size_of::<Vertex>()) as isize
    );
    vbo.insert_data(
        0,
        (vertices.len() * std::mem::size_of::<Vertex>()) as isize,
        vertices.as_ptr().cast(),
    );
    vbo.set_attributes(&vec![
        VertexAttributeTypes::Vec3(VertexAttributeDataTypes::F32), // vertices
        VertexAttributeTypes::Vec3(VertexAttributeDataTypes::F32), // color
    ]);
    let ebo = IndexBuffer::new(&gl);
    ebo.bind();
    ebo.alloc(indices.len());
    ebo.insert_data(0, indices.len(), indices.as_ptr());
    vao.set_vbo(vbo);
    vao.set_ebo(ebo);

    let mut shader = Shader::new(&gl);
    shader
        .load_from_memory(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)
        .unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Window { win_event, .. } => {
                    match win_event {
                        sdl3::event::WindowEvent::Resized(w, h) => unsafe {
                            gl.Viewport(0, 0, w, h);
                        },
                        _ => {}
                    };
                }
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
        shader.bind();
        vao.bind();
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
            gl.ClearColor(0.1, 0.1, 0.1, 1.0);
            gl.DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
        }
        window.gl_swap_window();
    }

    Ok(())
}

const VERTEX_SHADER_SOURCE: &str = r"
#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

out vec3 oColor;

void main() {
   gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
   oColor = aColor;
}
";

const FRAGMENT_SHADER_SOURCE: &str = r"
#version 330 core
out vec4 FragColor;
in vec3 oColor;

void main()
{
    FragColor = vec4(oColor, 1.0f);
} 
";

#[repr(C)]
struct Vertex {
    pub vertice: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    fn new(vertice: [f32; 3], color: [f32; 3]) -> Self {
        Self { vertice, color }
    }
}
