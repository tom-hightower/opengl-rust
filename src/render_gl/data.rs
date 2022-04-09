use gl;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct gl_vertex {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

impl gl_vertex {
    pub fn new(d0: f32, d1: f32, d2: f32) -> gl_vertex {
        gl_vertex { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }

    pub fn sub(v1: &gl_vertex, v2: &gl_vertex) -> gl_vertex {
        let d0: f32 = v1.d0 - v2.d0;
        let d1: f32 = v1.d1 - v2.d1;
        let d2: f32 = v1.d2 - v2.d2;
        gl_vertex { d0, d1, d2 }
    }

    pub fn add(v1: &gl_vertex, v2: &gl_vertex) -> gl_vertex {
        let d0: f32 = v1.d0 + v2.d0;
        let d1: f32 = v1.d1 + v2.d1;
        let d2: f32 = v1.d2 + v2.d2;
        gl_vertex { d0, d1, d2 }
    }

    pub fn dot(v1: &gl_vertex, v2: &gl_vertex) -> f32 {
        (v1.d0 * v2.d0) + (v1.d1 * v2.d1) + (v1.d2 * v2.d2)
    }

    pub fn cross(v1: &gl_vertex, v2: &gl_vertex) -> gl_vertex {
        let d0 = (v1.d1 * v2.d2) - (v1.d2 * v2.d1);
        let d1 = (v1.d2 * v2.d0) - (v1.d0 * v2.d2);
        let d2 = (v1.d0 * v2.d1) - (v1.d1 * v2.d0);
        gl_vertex { d0, d1, d2 }
    }

    pub fn copy(&self) -> gl_vertex {
        gl_vertex {
            d0: self.d0,
            d1: self.d1,
            d2: self.d2,
        }
    }

    pub fn mag(&self) -> f32 {
        f32::sqrt((self.d0 * self.d0) + (self.d1 * self.d1) + (self.d2 * self.d2))
    }

    pub fn norm(&mut self) {
        let mag = self.mag();
        self.d0 = self.d0 / mag;
        self.d1 = self.d1 / mag;
        self.d2 = self.d2 / mag;
    }

    pub fn mult(&mut self, factor: f32) {
        self.d0 = self.d0 * factor;
        self.d1 = self.d1 * factor;
        self.d2 = self.d2 * factor;
    }
}

impl From<(f32, f32, f32)> for gl_vertex {
    fn from(other: (f32, f32, f32)) -> Self {
        gl_vertex::new(other.0, other.1, other.2)
    }
}
