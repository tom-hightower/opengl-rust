use gl;

// TODO: Store multiple array buffers for each gl context
// ( vbo: Vec<gl::types::GLuint>, )
pub struct ArrayBuffer {
    vbo: gl::types::GLuint,
    gl: gl::Gl,
}

impl ArrayBuffer {
    pub fn new(gl: &gl::Gl) -> Self {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo);
        }
        ArrayBuffer {
            vbo,
            gl: gl.clone(),
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            self.gl.BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            )
        }
    }
}

impl Drop for ArrayBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &mut self.vbo);
        }
    }
}

// TODO: Same multiple store as ArrayBuffer
pub struct VertexArray {
    vao: gl::types::GLuint,
    gl: gl::Gl,
}

impl VertexArray {
    pub fn new(gl: &gl::Gl) -> Self {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
        }
        VertexArray {
            vao,
            gl: gl.clone(),
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteVertexArrays(1, &mut self.vao);
        }
    }
}
