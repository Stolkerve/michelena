mod gl_buffers;
mod gl_vertex_array;
// mod vertex_array;

// #![feature(portable_simd)]
// use std::simd::{f32x4, Simd};
// use core::str;
// use std::ffi::{c_char, CStr, CString};

use std::{
    ffi::{c_void, CStr, CString},
    str::FromStr,
};

use gl::{
    types::{GLint, GLuint},
    Gl,
};
use gl_buffers::{VertexAttributeDataTypes, VertexAttributeTypes, VertexBuffer};
use gl_vertex_array::VertexArray;
use sdl3::{
    event::Event,
    sys::video::{
        SDL_GL_SetAttribute, SDL_GL_CONTEXT_MAJOR_VERSION, SDL_GL_CONTEXT_MINOR_VERSION,
        SDL_GL_CONTEXT_PROFILE_CORE, SDL_GL_CONTEXT_PROFILE_MASK,
    },
};

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
        -0.5, -0.5, 0.0, //
        0.5, -0.5, 0.0, //
        0.0, 0.5, 0.0f32,
    ];

    let mut vao = VertexArray::new(&gl);
    vao.bind();
    let vbo = VertexBuffer::new(&gl);
    vbo.bind();
    vbo.alloc(VertexAttributeDataTypes::F32.get_size() as isize * (vertices.len() as isize));
    vbo.insert_data(0, &vertices);
    vbo.set_attributes(&vec![
        VertexAttributeTypes::Vec3(VertexAttributeDataTypes::F32), // vertices
                                                                   // VertexAttributeTypes::Vec3(VertexAttributeDataTypes::F32), // colors
                                                                   // VertexAttributeTypes::Vec2(VertexAttributeDataTypes::F32), // UV
    ]);
    vao.set_vbo(vbo);

    let vertex_shader: GLuint;
    let fragment_shader: GLuint;
    let shader_program: GLuint;
    unsafe {
        vertex_shader = gl.CreateShader(gl::VERTEX_SHADER);
        gl.ShaderSource(
            vertex_shader,
            1,
            &CString::from_str(VERTEX_SHADER_SOURCE).unwrap().as_ptr(),
            std::ptr::null(),
        );
        gl.CompileShader(vertex_shader);
        let mut success: GLint = 0;
        gl.GetShaderiv(
            vertex_shader,
            gl::COMPILE_STATUS,
            &mut success as *mut GLint,
        );
        if success == 0 {
            let mut info_log: [i8; 512] = [0; 512];
            gl.GetShaderInfoLog(
                vertex_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr(),
            );
            println!("{:?}", CStr::from_ptr(info_log.as_ptr()).to_str());
        }

        fragment_shader = gl.CreateShader(gl::FRAGMENT_SHADER);
        gl.ShaderSource(
            fragment_shader,
            1,
            &CString::from_str(FRAGMENT_SHADER_SOURCE).unwrap().as_ptr(),
            std::ptr::null(),
        );
        gl.CompileShader(fragment_shader);
        let mut success: GLint = 0;
        gl.GetShaderiv(
            fragment_shader,
            gl::COMPILE_STATUS,
            &mut success as *mut GLint,
        );
        if success == 0 {
            let mut info_log: [i8; 512] = [0; 512];
            gl.GetShaderInfoLog(
                fragment_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr(),
            );
            println!("{:?}", CStr::from_ptr(info_log.as_ptr()).to_str());
        }

        shader_program = gl.CreateProgram();
        gl.AttachShader(shader_program, vertex_shader);
        gl.AttachShader(shader_program, fragment_shader);
        gl.LinkProgram(shader_program);
        gl.GetShaderiv(fragment_shader, gl::LINK_STATUS, &mut success as *mut GLint);
        if success == 0 {
            let mut info_log: [i8; 512] = [0; 512];
            gl.GetShaderInfoLog(
                fragment_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr(),
            );
            println!("{:?}", CStr::from_ptr(info_log.as_ptr()).to_str());
        }
        gl.DeleteShader(vertex_shader);
        gl.DeleteShader(fragment_shader);
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
            gl.ClearColor(0.1, 0.1, 0.1, 1.0);
            gl.UseProgram(shader_program);
            vao.bind();
            gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.gl_swap_window();
    }

    Ok(())
}

const VERTEX_SHADER_SOURCE: &str = r"
#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

void main() {
   gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
";

const FRAGMENT_SHADER_SOURCE: &str = r"
#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
} 
";
