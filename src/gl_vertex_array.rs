use crate::{
    gl::{types::GLuint, Gl},
    gl_buffers::IndexBuffer,
    gl_buffers::VertexBuffer,
};

pub struct VertexArray<'a> {
    pub id: GLuint,
    pub gl: &'a Gl,
    pub vbo: Option<VertexBuffer<'a>>,
    pub ebo: Option<IndexBuffer<'a>>,
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

        Self {
            id,
            gl,
            vbo: None,
            ebo: None,
        }
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindVertexArray(self.id) };
    }

    #[allow(dead_code)]
    pub fn unbind(&self) {
        unsafe { self.gl.BindVertexArray(0) };
    }

    pub fn set_vbo(&mut self, vbo: VertexBuffer<'a>) {
        self.vbo = Some(vbo);
    }
    pub fn set_ebo(&mut self, ebo: IndexBuffer<'a>) {
        self.ebo = Some(ebo);
    }
}
