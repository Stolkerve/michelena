use crate::gl;
use crate::gl::{types::GLchar, types::GLenum, types::GLuint, Gl};
use anyhow::Result;

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::str;

#[allow(dead_code)]
pub enum UniformType {
    U32(u32),
    I32(i32),
    F32(f32),
    F64(f64),
    Fv2(f32, f32),
    Fv3(f32, f32, f32),
    Fv4(f32, f32, f32, f32),
    M3(*const f32),
    M4(*const f32),
}

pub enum ShaderType {
    VERTEX,
    FRAGMENT,
}

impl ShaderType {
    pub fn get_gl_value(&self) -> GLenum {
        match self {
            ShaderType::VERTEX => 0x8B31,
            ShaderType::FRAGMENT => 0x8B30,
        }
    }
}

pub struct Shader<'a> {
    pub gl: &'a Gl,
    pub program: u32,
    pub uniforms_location: HashMap<String, i32>,
}

impl<'a> Shader<'a> {
    pub fn new(gl: &'a Gl) -> Self {
        Self {
            program: 0,
            uniforms_location: HashMap::new(),
            gl,
        }
    }

    pub fn bind(&self) {
        unsafe { self.gl.UseProgram(self.program) }
    }

    #[allow(dead_code)]
    pub fn unbind(&self) {
        unsafe { self.gl.UseProgram(0) }
    }

    pub fn load_from_memory(
        &mut self,
        vertex_shader: &str,
        fragment_shader: &str,
    ) -> Result<(), String> {
        let vertex = self.compile_shader(vertex_shader, ShaderType::VERTEX)?;
        let fragment = self.compile_shader(fragment_shader, ShaderType::FRAGMENT)?;

        let program = self.create_shader_program(&vertex, &fragment)?;
        self.delete_shaders(&vertex, &fragment);

        self.program = program;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn load_from_file(
        &mut self,
        vertex_shader: &str,
        fragment_shader: &str,
    ) -> Result<(), String> {
        let mut vertex_file = File::open(vertex_shader).expect("");
        let mut fragment_file = File::open(fragment_shader).expect("");
        let mut vertex_source = String::new();
        let mut fragment_source = String::new();
        vertex_file.read_to_string(&mut vertex_source).expect("");
        fragment_file
            .read_to_string(&mut fragment_source)
            .expect("");

        let vertex = self.compile_shader(vertex_source.as_str(), ShaderType::VERTEX)?;
        let fragment = self.compile_shader(fragment_source.as_str(), ShaderType::FRAGMENT)?;

        let program = self.create_shader_program(&vertex, &fragment)?;
        self.delete_shaders(&vertex, &fragment);

        self.program = program;
        return Ok(());
    }

    #[allow(dead_code)]
    pub fn set_uniform(&mut self, name: &str, v: UniformType) {
        match v {
            UniformType::U32(v) => unsafe {
                self.gl.Uniform1ui(self.get_uniform_locacion(name), v);
            },
            UniformType::I32(v) => unsafe {
                self.gl.Uniform1i(self.get_uniform_locacion(name), v);
            },
            UniformType::M3(m) => unsafe {
                self.gl
                    .UniformMatrix3fv(self.get_uniform_locacion(name), 1, gl::FALSE, m)
            },
            UniformType::M4(m) => unsafe {
                self.gl
                    .UniformMatrix4fv(self.get_uniform_locacion(name), 1, gl::FALSE, m)
            },
            UniformType::F32(v) => unsafe {
                self.gl.Uniform1f(self.get_uniform_locacion(name), v);
            },
            UniformType::F64(v) => unsafe {
                self.gl.Uniform1d(self.get_uniform_locacion(name), v);
            },
            UniformType::Fv2(x, y) => unsafe {
                self.gl.Uniform2f(self.get_uniform_locacion(name), x, y);
            },
            UniformType::Fv3(x, y, z) => unsafe {
                self.gl.Uniform3f(self.get_uniform_locacion(name), x, y, z);
            },
            UniformType::Fv4(x, y, z, w) => unsafe {
                self.gl
                    .Uniform4f(self.get_uniform_locacion(name), x, y, z, w);
            },
        }
    }

    fn get_uniform_locacion(&mut self, name: &str) -> i32 {
        if self.uniforms_location.contains_key(name) {
            return self.uniforms_location[name];
        }
        unsafe {
            let c_name = CString::new(name.as_bytes()).unwrap();
            let location = self.gl.GetUniformLocation(self.program, c_name.as_ptr());
            self.uniforms_location.insert(name.to_string(), location);
        }
        self.uniforms_location[name]
    }

    fn create_shader_program(&self, vertex: &u32, fragment: &u32) -> Result<GLuint, String> {
        unsafe {
            let program = self.gl.CreateProgram();
            self.gl.AttachShader(program, *vertex);
            self.gl.AttachShader(program, *fragment);
            self.gl.LinkProgram(program);

            let mut success = 0;
            self.gl.GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut info_log: [i8; 512] = [0; 512];
                self.gl.GetProgramInfoLog(
                    program,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                return Err(String::from_utf8_unchecked(std::mem::transmute(
                    info_log.to_vec(),
                )));
            }
            Ok(program)
        }
    }

    fn compile_shader(&self, shader: &str, r#type: ShaderType) -> Result<GLuint, String> {
        unsafe {
            let id = self.gl.CreateShader(r#type.get_gl_value());
            let c_str_shader = CString::new(shader.as_bytes()).unwrap();
            self.gl
                .ShaderSource(id, 1, &c_str_shader.as_ptr(), std::ptr::null());
            self.gl.CompileShader(id);

            let mut success = 0;

            self.gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut info_log: [i8; 512] = [0; 512];
                self.gl.GetShaderInfoLog(
                    id,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                return Err(String::from_utf8_unchecked(std::mem::transmute(
                    info_log.to_vec(),
                )));
            }

            Ok(id)
        }
    }

    fn delete_shaders(&self, vertex: &u32, fragment: &u32) {
        unsafe {
            self.gl.DeleteShader(*vertex);
            self.gl.DeleteShader(*fragment);
        }
    }
}

impl<'a> Drop for Shader<'a> {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.program);
        }
    }
}
