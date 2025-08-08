use std::ffi::c_void;

use crate::gl::{self, Gl};
use gl::types::{GLint, GLuint};

pub enum VertexAttributeDataTypes {
    F32,
}

impl VertexAttributeDataTypes {
    pub fn get_size(&self) -> GLint {
        match self {
            VertexAttributeDataTypes::F32 => return 4,
        }
    }
}

pub enum VertexAttributeTypes {
    Scalar(VertexAttributeDataTypes),
    Vec2(VertexAttributeDataTypes),
    Vec3(VertexAttributeDataTypes),
    Vec4(VertexAttributeDataTypes),
}

impl VertexAttributeTypes {
    fn get_size(&self) -> gl::types::GLint {
        match self {
            VertexAttributeTypes::Scalar(data_types) => {
                return self.get_components_count() * data_types.get_size()
            }
            VertexAttributeTypes::Vec2(data_types) => {
                return self.get_components_count() * data_types.get_size()
            }
            VertexAttributeTypes::Vec3(data_types) => {
                return self.get_components_count() * data_types.get_size()
            }
            VertexAttributeTypes::Vec4(data_types) => {
                return self.get_components_count() * data_types.get_size()
            }
        }
    }

    fn get_components_count(&self) -> GLint {
        match self {
            VertexAttributeTypes::Scalar(_) => return 1,
            VertexAttributeTypes::Vec2(_) => return 2,
            VertexAttributeTypes::Vec3(_) => return 3,
            VertexAttributeTypes::Vec4(_) => return 4,
        }
    }
}

pub struct VertexBuffer<'a> {
    pub id: GLuint,
    pub gl: &'a Gl,
}

impl<'a> VertexBuffer<'a> {
    pub fn new(gl: &'a Gl) -> Self {
        let mut id = 0;
        unsafe { gl.GenBuffers(1, &mut id) };
        Self { id, gl }
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindBuffer(gl::ARRAY_BUFFER, self.id) };
    }

    pub fn unbind(&self) {
        unsafe { self.gl.BindBuffer(gl::ARRAY_BUFFER, 0) };
    }

    pub fn alloc(&self, size: isize) {
        unsafe {
            self.bind();
            self.gl
                .BufferData(gl::ARRAY_BUFFER, size, std::ptr::null(), gl::DYNAMIC_DRAW);
        }
    }

    pub fn insert_data(&self, offset: isize, data: &[f32]) {
        unsafe {
            self.bind();
            self.gl.BufferSubData(
                gl::ARRAY_BUFFER,
                offset,
                data.len() as isize * 4,
                data.as_ptr() as *const c_void,
            );
        }
    }

    pub fn set_attributes(&self, attrs: &Vec<VertexAttributeTypes>) {
        let mut stride = 0;
        for attr in attrs.iter() {
            stride += attr.get_size()
        }
        println!("stride {}", stride);

        let mut offset = 0;
        for i in 0..attrs.len() {
            let attr = attrs.get(i).unwrap();
            println!(
                "index {} size {} stride {} offset {}",
                i,
                attr.get_components_count(),
                stride,
                offset
            );
            unsafe {
                self.gl.VertexAttribPointer(
                    i as u32,
                    attr.get_components_count(),
                    gl::FLOAT,
                    gl::FALSE,
                    stride,
                    &offset as *const i32 as *const c_void,
                );
                self.gl.EnableVertexAttribArray(i as u32);
            }
            offset += attr.get_size();
        }
    }
}

impl<'a> Drop for VertexBuffer<'a> {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteBuffers(1, &self.id as *const u32) };
    }
}

pub struct IndexBuffer {
    pub id: u32,
}
