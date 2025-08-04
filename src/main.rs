mod gl_buffers;
mod vertex_array;

// #![feature(portable_simd)]
// use std::simd::{f32x4, Simd};
// use core::str;
// use std::ffi::{c_char, CStr, CString};

use gl::Gl;
use gl_buffers::{VertexAttributeTypes, VertexBuffer};
use sdl3::event::Event;

pub mod gl {
    #![allow(clippy::all)]
    include!("gl_bindings/bindings.rs");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl3 demo: Window", 800, 600)
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let gl_context = window.gl_create_context().map_err(|e| e.to_string())?;
    window
        .gl_make_current(&gl_context)
        .map_err(|e| e.to_string())?;

    let gl = Gl::load_with(|s| window.subsystem().gl_get_proc_address(s).unwrap() as *const _);

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    let vbo = VertexBuffer::new(gl.clone());

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
        }

        window.gl_swap_window();
    }

    Ok(())
}

// fn get_gl_string(variant: gl::types::GLenum) -> Option<&'static CStr> {
// unsafe {
// let s = gl::GetString(variant);
// (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
// }
// }

// println!(
//     "GL_VERSION {:?}",
//     convert_ptr_to_str(gl::GetString(gl::VERSION)).unwrap()
// );
// println!(
//     "GL_VENDOR {:?}",
//     convert_ptr_to_str(gl::GetString(gl::VENDOR)).unwrap()
// );
// println!(
//     "GL_RENDERER {:?}",
//     convert_ptr_to_str(gl::GetString(gl::RENDERER)).unwrap()
// );
