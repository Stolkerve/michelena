use crate::{
    gl::{types::GLuint, Gl},
    gl_buffers::VertexBuffer,
};

pub struct VertexArray<'a> {
    pub id: GLuint,
    pub gl: &'a Gl,
    pub vbo: Option<VertexBuffer<'a>>,
}

impl<'a> Drop for VertexArray<'a> {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteVertexArrays(1, &mut self.id) };
    }
}

impl<'a> VertexArray<'a> {
    pub fn new(gl: &'a Gl) -> Self {
        let mut id = 0;
        unsafe { gl.GenVertexArrays(1, &mut id) };

        Self { id, gl, vbo: None }
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindVertexArray(self.id) };
    }

    pub fn unbind(&self) {
        unsafe { self.gl.BindVertexArray(0) };
    }

    pub fn set_vbo(&mut self, vbo: VertexBuffer<'a>) {
        self.vbo = Some(vbo);
    }
}
