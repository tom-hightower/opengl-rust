use gl;

// -----------------------------
// gl_vertex_f32 (f32_f32_f32)

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct gl_vertex_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

impl gl_vertex_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32) -> gl_vertex_f32 {
        gl_vertex_f32 { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
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

    pub fn sub(v1: &gl_vertex_f32, v2: &gl_vertex_f32) -> gl_vertex_f32 {
        let d0: f32 = v1.d0 - v2.d0;
        let d1: f32 = v1.d1 - v2.d1;
        let d2: f32 = v1.d2 - v2.d2;
        gl_vertex_f32 { d0, d1, d2 }
    }

    pub fn add(v1: &gl_vertex_f32, v2: &gl_vertex_f32) -> gl_vertex_f32 {
        let d0: f32 = v1.d0 + v2.d0;
        let d1: f32 = v1.d1 + v2.d1;
        let d2: f32 = v1.d2 + v2.d2;
        gl_vertex_f32 { d0, d1, d2 }
    }

    pub fn dot(v1: &gl_vertex_f32, v2: &gl_vertex_f32) -> f32 {
        (v1.d0 * v2.d0) + (v1.d1 * v2.d1) + (v1.d2 * v2.d2)
    }

    pub fn cross(v1: &gl_vertex_f32, v2: &gl_vertex_f32) -> gl_vertex_f32 {
        let d0 = (v1.d1 * v2.d2) - (v1.d2 * v2.d1);
        let d1 = (v1.d2 * v2.d0) - (v1.d0 * v2.d2);
        let d2 = (v1.d0 * v2.d1) - (v1.d1 * v2.d0);
        gl_vertex_f32 { d0, d1, d2 }
    }

    pub fn copy(&self) -> gl_vertex_f32 {
        gl_vertex_f32 {
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

impl From<(f32, f32, f32)> for gl_vertex_f32 {
    fn from(other: (f32, f32, f32)) -> Self {
        gl_vertex_f32::new(other.0, other.1, other.2)
    }
}

// ---------------------------
// u2_u10_u10_u10_rev_float

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u2_u10_u10_u10_rev_float {
    pub inner: ::vec_2_10_10_10::Vector,
}

impl From<(f32, f32, f32, f32)> for u2_u10_u10_u10_rev_float {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        u2_u10_u10_u10_rev_float {
            inner: ::vec_2_10_10_10::Vector::new(other.0, other.1, other.2, other.3),
        }
    }
}

impl u2_u10_u10_u10_rev_float {
    pub fn new(d0: f32, d1: f32, d2: f32, d3: f32) -> Self {
        u2_u10_u10_u10_rev_float {
            inner: ::vec_2_10_10_10::Vector::new(d0, d1, d2, d3),
        }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::UNSIGNED_INT_2_10_10_10_REV,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

// ---------------------------
// i8_

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_ {
    pub d0: i8,
}

impl From<i8> for i8_ {
    fn from(other: i8) -> Self {
        i8_::new(other)
    }
}

impl i8_ {
    pub fn new(d0: i8) -> i8_ {
        i8_ { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            1,
            gl::BYTE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

// ---------------------------
// i8_float

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_float {
    pub d0: i8,
}

impl From<i8> for i8_float {
    fn from(other: i8) -> Self {
        i8_float::new(other)
    }
}

impl i8_float {
    pub fn new(d0: i8) -> i8_float {
        i8_float { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            1,
            gl::BYTE,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

// --------------------------
// i8_i8_i8

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_i8_i8 {
    pub d0: i8,
    pub d1: i8,
    pub d2: i8,
}

impl From<(i8, i8, i8)> for i8_i8_i8 {
    fn from(other: (i8, i8, i8)) -> Self {
        i8_i8_i8::new(other.0, other.1, other.2)
    }
}

impl i8_i8_i8 {
    pub fn new(d0: i8, d1: i8, d2: i8) -> i8_i8_i8 {
        i8_i8_i8 { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            3,
            gl::BYTE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );

        gl.EnableVertexAttribArray(location as gl::types::GLuint);
    }
}

// ----------------------------
// i8_i8_i8_i8

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_i8_i8_i8 {
    pub d0: i8,
    pub d1: i8,
    pub d2: i8,
    pub d3: i8,
}

impl From<(i8, i8, i8, i8)> for i8_i8_i8_i8 {
    fn from(other: (i8, i8, i8, i8)) -> Self {
        i8_i8_i8_i8::new(other.0, other.1, other.2, other.3)
    }
}

impl i8_i8_i8_i8 {
    pub fn new(d0: i8, d1: i8, d2: i8, d3: i8) -> i8_i8_i8_i8 {
        i8_i8_i8_i8 { d0, d1, d2, d3 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            4,
            gl::BYTE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

// ------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_i8_float {
    pub d0: i8,
    pub d1: i8,
}

impl i8_i8_float {
    pub fn new(d0: i8, d1: i8) -> i8_i8_float {
        i8_i8_float { d0, d1 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            2,
            gl::BYTE,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i8, i8)> for i8_i8_float {
    fn from(other: (i8, i8)) -> Self {
        i8_i8_float::new(other.0, other.1)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_i8_i8_float {
    pub d0: i8,
    pub d1: i8,
    pub d2: i8,
}

impl i8_i8_i8_float {
    pub fn new(d0: i8, d1: i8, d2: i8) -> i8_i8_i8_float {
        i8_i8_i8_float { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::BYTE,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i8, i8, i8)> for i8_i8_i8_float {
    fn from(other: (i8, i8, i8)) -> Self {
        i8_i8_i8_float::new(other.0, other.1, other.2)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_i8_i8_i8_float {
    pub d0: i8,
    pub d1: i8,
    pub d2: i8,
    pub d3: i8,
}

impl i8_i8_i8_i8_float {
    pub fn new(d0: i8, d1: i8, d2: i8, d3: i8) -> i8_i8_i8_i8_float {
        i8_i8_i8_i8_float { d0, d1, d2, d3 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::BYTE,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i8, i8, i8, i8)> for i8_i8_i8_i8_float {
    fn from(other: (i8, i8, i8, i8)) -> Self {
        i8_i8_i8_i8_float::new(other.0, other.1, other.2, other.3)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i16_ {
    pub d0: i16,
}

impl i16_ {
    pub fn new(d0: i16) -> i16_ {
        i16_ { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            1,
            gl::SHORT,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<i16> for i16_ {
    fn from(other: i16) -> Self {
        i16_::new(other)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i16_i16 {
    pub d0: i16,
    pub d1: i16,
}

impl i16_i16 {
    pub fn new(d0: i16, d1: i16) -> i16_i16 {
        i16_i16 { d0, d1 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            2,
            gl::SHORT,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i16, i16)> for i16_i16 {
    fn from(other: (i16, i16)) -> Self {
        i16_i16::new(other.0, other.1)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i16_i16_i16 {
    pub d0: i16,
    pub d1: i16,
    pub d2: i16,
}

impl i16_i16_i16 {
    pub fn new(d0: i16, d1: i16, d2: i16) -> i16_i16_i16 {
        i16_i16_i16 { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            3,
            gl::SHORT,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i16, i16, i16)> for i16_i16_i16 {
    fn from(other: (i16, i16, i16)) -> Self {
        i16_i16_i16::new(other.0, other.1, other.2)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i16_i16_i16_i16 {
    pub d0: i16,
    pub d1: i16,
    pub d2: i16,
    pub d3: i16,
}

impl i16_i16_i16_i16 {
    pub fn new(d0: i16, d1: i16, d2: i16, d3: i16) -> i16_i16_i16_i16 {
        i16_i16_i16_i16 { d0, d1, d2, d3 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            4,
            gl::SHORT,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i16, i16, i16, i16)> for i16_i16_i16_i16 {
    fn from(other: (i16, i16, i16, i16)) -> Self {
        i16_i16_i16_i16::new(other.0, other.1, other.2, other.3)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i16_float {
    pub d0: i16,
}

impl i16_float {
    pub fn new(d0: i16) -> i16_float {
        i16_float { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            1,
            gl::SHORT,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<i16> for i16_float {
    fn from(other: i16) -> Self {
        i16_float::new(other)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i16_i16_float {
    pub d0: i16,
    pub d1: i16,
}

impl i16_i16_float {
    pub fn new(d0: i16, d1: i16) -> i16_i16_float {
        i16_i16_float { d0, d1 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            2,
            gl::SHORT,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i16, i16)> for i16_i16_float {
    fn from(other: (i16, i16)) -> Self {
        i16_i16_float::new(other.0, other.1)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i16_i16_i16_float {
    pub d0: i16,
    pub d1: i16,
    pub d2: i16,
}

impl i16_i16_i16_float {
    pub fn new(d0: i16, d1: i16, d2: i16) -> i16_i16_i16_float {
        i16_i16_i16_float { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::SHORT,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i16, i16, i16)> for i16_i16_i16_float {
    fn from(other: (i16, i16, i16)) -> Self {
        i16_i16_i16_float::new(other.0, other.1, other.2)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i16_i16_i16_i16_float {
    pub d0: i16,
    pub d1: i16,
    pub d2: i16,
    pub d3: i16,
}

impl i16_i16_i16_i16_float {
    pub fn new(d0: i16, d1: i16, d2: i16, d3: i16) -> i16_i16_i16_i16_float {
        i16_i16_i16_i16_float { d0, d1, d2, d3 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::SHORT,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i16, i16, i16, i16)> for i16_i16_i16_i16_float {
    fn from(other: (i16, i16, i16, i16)) -> Self {
        i16_i16_i16_i16_float::new(other.0, other.1, other.2, other.3)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32_ {
    pub d0: i32,
}

impl i32_ {
    pub fn new(d0: i32) -> i32_ {
        i32_ { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            1,
            gl::INT,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<i32> for i32_ {
    fn from(other: i32) -> Self {
        i32_::new(other)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32_i32 {
    pub d0: i32,
    pub d1: i32,
}

impl i32_i32 {
    pub fn new(d0: i32, d1: i32) -> i32_i32 {
        i32_i32 { d0, d1 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            2,
            gl::INT,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i32, i32)> for i32_i32 {
    fn from(other: (i32, i32)) -> Self {
        i32_i32::new(other.0, other.1)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32_i32_i32 {
    pub d0: i32,
    pub d1: i32,
    pub d2: i32,
}

impl i32_i32_i32 {
    pub fn new(d0: i32, d1: i32, d2: i32) -> i32_i32_i32 {
        i32_i32_i32 { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            3,
            gl::INT,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i32, i32, i32)> for i32_i32_i32 {
    fn from(other: (i32, i32, i32)) -> Self {
        i32_i32_i32::new(other.0, other.1, other.2)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32_i32_i32_i32 {
    pub d0: i32,
    pub d1: i32,
    pub d2: i32,
    pub d3: i32,
}

impl i32_i32_i32_i32 {
    pub fn new(d0: i32, d1: i32, d2: i32, d3: i32) -> i32_i32_i32_i32 {
        i32_i32_i32_i32 { d0, d1, d2, d3 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            4,
            gl::INT,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i32, i32, i32, i32)> for i32_i32_i32_i32 {
    fn from(other: (i32, i32, i32, i32)) -> Self {
        i32_i32_i32_i32::new(other.0, other.1, other.2, other.3)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32_float {
    pub d0: i32,
}

impl i32_float {
    pub fn new(d0: i32) -> i32_float {
        i32_float { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            1,
            gl::INT,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<i32> for i32_float {
    fn from(other: i32) -> Self {
        i32_float::new(other)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32_i32_float {
    pub d0: i32,
    pub d1: i32,
}

impl i32_i32_float {
    pub fn new(d0: i32, d1: i32) -> i32_i32_float {
        i32_i32_float { d0, d1 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            2,
            gl::INT,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i32, i32)> for i32_i32_float {
    fn from(other: (i32, i32)) -> Self {
        i32_i32_float::new(other.0, other.1)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32_i32_i32_float {
    pub d0: i32,
    pub d1: i32,
    pub d2: i32,
}

impl i32_i32_i32_float {
    pub fn new(d0: i32, d1: i32, d2: i32) -> i32_i32_i32_float {
        i32_i32_i32_float { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::INT,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i32, i32, i32)> for i32_i32_i32_float {
    fn from(other: (i32, i32, i32)) -> Self {
        i32_i32_i32_float::new(other.0, other.1, other.2)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32_i32_i32_i32_float {
    pub d0: i32,
    pub d1: i32,
    pub d2: i32,
    pub d3: i32,
}

impl i32_i32_i32_i32_float {
    pub fn new(d0: i32, d1: i32, d2: i32, d3: i32) -> i32_i32_i32_i32_float {
        i32_i32_i32_i32_float { d0, d1, d2, d3 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::INT,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i32, i32, i32, i32)> for i32_i32_i32_i32_float {
    fn from(other: (i32, i32, i32, i32)) -> Self {
        i32_i32_i32_i32_float::new(other.0, other.1, other.2, other.3)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u8_ {
    pub d0: u8,
}

impl u8_ {
    pub fn new(d0: u8) -> u8_ {
        u8_ { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            1,
            gl::UNSIGNED_BYTE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<u8> for u8_ {
    fn from(other: u8) -> Self {
        u8_::new(other)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u8_u8 {
    pub d0: u8,
    pub d1: u8,
}

impl u8_u8 {
    pub fn new(d0: u8, d1: u8) -> u8_u8 {
        u8_u8 { d0, d1 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            2,
            gl::UNSIGNED_BYTE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u8, u8)> for u8_u8 {
    fn from(other: (u8, u8)) -> Self {
        u8_u8::new(other.0, other.1)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u8_u8_u8 {
    pub d0: u8,
    pub d1: u8,
    pub d2: u8,
}

impl u8_u8_u8 {
    pub fn new(d0: u8, d1: u8, d2: u8) -> u8_u8_u8 {
        u8_u8_u8 { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            3,
            gl::UNSIGNED_BYTE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u8, u8, u8)> for u8_u8_u8 {
    fn from(other: (u8, u8, u8)) -> Self {
        u8_u8_u8::new(other.0, other.1, other.2)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u8_u8_u8_u8 {
    pub d0: u8,
    pub d1: u8,
    pub d2: u8,
    pub d3: u8,
}

impl u8_u8_u8_u8 {
    pub fn new(d0: u8, d1: u8, d2: u8, d3: u8) -> u8_u8_u8_u8 {
        u8_u8_u8_u8 { d0, d1, d2, d3 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            4,
            gl::UNSIGNED_BYTE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u8, u8, u8, u8)> for u8_u8_u8_u8 {
    fn from(other: (u8, u8, u8, u8)) -> Self {
        u8_u8_u8_u8::new(other.0, other.1, other.2, other.3)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u8_float {
    pub d0: u8,
}

impl u8_float {
    pub fn new(d0: u8) -> u8_float {
        u8_float { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            1,
            gl::UNSIGNED_BYTE,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<u8> for u8_float {
    fn from(other: u8) -> Self {
        u8_float::new(other)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u8_u8_float {
    pub d0: u8,
    pub d1: u8,
}

impl u8_u8_float {
    pub fn new(d0: u8, d1: u8) -> u8_u8_float {
        u8_u8_float { d0, d1 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            2,
            gl::UNSIGNED_BYTE,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u8, u8)> for u8_u8_float {
    fn from(other: (u8, u8)) -> Self {
        u8_u8_float::new(other.0, other.1)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u8_u8_u8_float {
    pub d0: u8,
    pub d1: u8,
    pub d2: u8,
}

impl u8_u8_u8_float {
    pub fn new(d0: u8, d1: u8, d2: u8) -> u8_u8_u8_float {
        u8_u8_u8_float { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::UNSIGNED_BYTE,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u8, u8, u8)> for u8_u8_u8_float {
    fn from(other: (u8, u8, u8)) -> Self {
        u8_u8_u8_float::new(other.0, other.1, other.2)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u8_u8_u8_u8_float {
    pub d0: u8,
    pub d1: u8,
    pub d2: u8,
    pub d3: u8,
}

impl u8_u8_u8_u8_float {
    pub fn new(d0: u8, d1: u8, d2: u8, d3: u8) -> u8_u8_u8_u8_float {
        u8_u8_u8_u8_float { d0, d1, d2, d3 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::UNSIGNED_BYTE,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u8, u8, u8, u8)> for u8_u8_u8_u8_float {
    fn from(other: (u8, u8, u8, u8)) -> Self {
        u8_u8_u8_u8_float::new(other.0, other.1, other.2, other.3)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_ {
    pub d0: u16,
}

impl u16_ {
    pub fn new(d0: u16) -> u16_ {
        u16_ { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            1,
            gl::UNSIGNED_SHORT,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<u16> for u16_ {
    fn from(other: u16) -> Self {
        u16_::new(other)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_u16 {
    pub d0: u16,
    pub d1: u16,
}

impl u16_u16 {
    pub fn new(d0: u16, d1: u16) -> u16_u16 {
        u16_u16 { d0, d1 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            2,
            gl::UNSIGNED_SHORT,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u16, u16)> for u16_u16 {
    fn from(other: (u16, u16)) -> Self {
        u16_u16::new(other.0, other.1)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_u16_u16 {
    pub d0: u16,
    pub d1: u16,
    pub d2: u16,
}

impl u16_u16_u16 {
    pub fn new(d0: u16, d1: u16, d2: u16) -> u16_u16_u16 {
        u16_u16_u16 { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            3,
            gl::UNSIGNED_SHORT,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u16, u16, u16)> for u16_u16_u16 {
    fn from(other: (u16, u16, u16)) -> Self {
        u16_u16_u16::new(other.0, other.1, other.2)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_u16_u16_u16 {
    pub d0: u16,
    pub d1: u16,
    pub d2: u16,
    pub d3: u16,
}

impl u16_u16_u16_u16 {
    pub fn new(d0: u16, d1: u16, d2: u16, d3: u16) -> u16_u16_u16_u16 {
        u16_u16_u16_u16 { d0, d1, d2, d3 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            4,
            gl::UNSIGNED_SHORT,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u16, u16, u16, u16)> for u16_u16_u16_u16 {
    fn from(other: (u16, u16, u16, u16)) -> Self {
        u16_u16_u16_u16::new(other.0, other.1, other.2, other.3)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_float {
    pub d0: u16,
}

impl u16_float {
    pub fn new(d0: u16) -> u16_float {
        u16_float { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            1,
            gl::UNSIGNED_SHORT,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<u16> for u16_float {
    fn from(other: u16) -> Self {
        u16_float::new(other)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_u16_float {
    pub d0: u16,
    pub d1: u16,
}

impl u16_u16_float {
    pub fn new(d0: u16, d1: u16) -> u16_u16_float {
        u16_u16_float { d0, d1 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            2,
            gl::UNSIGNED_SHORT,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u16, u16)> for u16_u16_float {
    fn from(other: (u16, u16)) -> Self {
        u16_u16_float::new(other.0, other.1)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_u16_u16_float {
    pub d0: u16,
    pub d1: u16,
    pub d2: u16,
}

impl u16_u16_u16_float {
    pub fn new(d0: u16, d1: u16, d2: u16) -> u16_u16_u16_float {
        u16_u16_u16_float { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::UNSIGNED_SHORT,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u16, u16, u16)> for u16_u16_u16_float {
    fn from(other: (u16, u16, u16)) -> Self {
        u16_u16_u16_float::new(other.0, other.1, other.2)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_u16_u16_u16_float {
    pub d0: u16,
    pub d1: u16,
    pub d2: u16,
    pub d3: u16,
}

impl u16_u16_u16_u16_float {
    pub fn new(d0: u16, d1: u16, d2: u16, d3: u16) -> u16_u16_u16_u16_float {
        u16_u16_u16_u16_float { d0, d1, d2, d3 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::UNSIGNED_SHORT,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u16, u16, u16, u16)> for u16_u16_u16_u16_float {
    fn from(other: (u16, u16, u16, u16)) -> Self {
        u16_u16_u16_u16_float::new(other.0, other.1, other.2, other.3)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u32_ {
    pub d0: u32,
}

impl u32_ {
    pub fn new(d0: u32) -> u32_ {
        u32_ { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            1,
            gl::UNSIGNED_INT,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<u32> for u32_ {
    fn from(other: u32) -> Self {
        u32_::new(other)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u32_u32 {
    pub d0: u32,
    pub d1: u32,
}

impl u32_u32 {
    pub fn new(d0: u32, d1: u32) -> u32_u32 {
        u32_u32 { d0, d1 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            2,
            gl::UNSIGNED_INT,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u32, u32)> for u32_u32 {
    fn from(other: (u32, u32)) -> Self {
        u32_u32::new(other.0, other.1)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u32_u32_u32 {
    pub d0: u32,
    pub d1: u32,
    pub d2: u32,
}

impl u32_u32_u32 {
    pub fn new(d0: u32, d1: u32, d2: u32) -> u32_u32_u32 {
        u32_u32_u32 { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            3,
            gl::UNSIGNED_INT,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u32, u32, u32)> for u32_u32_u32 {
    fn from(other: (u32, u32, u32)) -> Self {
        u32_u32_u32::new(other.0, other.1, other.2)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u32_u32_u32_u32 {
    pub d0: u32,
    pub d1: u32,
    pub d2: u32,
    pub d3: u32,
}

impl u32_u32_u32_u32 {
    pub fn new(d0: u32, d1: u32, d2: u32, d3: u32) -> u32_u32_u32_u32 {
        u32_u32_u32_u32 { d0, d1, d2, d3 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            4,
            gl::UNSIGNED_INT,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u32, u32, u32, u32)> for u32_u32_u32_u32 {
    fn from(other: (u32, u32, u32, u32)) -> Self {
        u32_u32_u32_u32::new(other.0, other.1, other.2, other.3)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u32_float {
    pub d0: u32,
}

impl u32_float {
    pub fn new(d0: u32) -> u32_float {
        u32_float { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            1,
            gl::UNSIGNED_INT,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<u32> for u32_float {
    fn from(other: u32) -> Self {
        u32_float::new(other)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u32_u32_float {
    pub d0: u32,
    pub d1: u32,
}

impl u32_u32_float {
    pub fn new(d0: u32, d1: u32) -> u32_u32_float {
        u32_u32_float { d0, d1 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            2,
            gl::UNSIGNED_INT,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u32, u32)> for u32_u32_float {
    fn from(other: (u32, u32)) -> Self {
        u32_u32_float::new(other.0, other.1)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u32_u32_u32_float {
    pub d0: u32,
    pub d1: u32,
    pub d2: u32,
}

impl u32_u32_u32_float {
    pub fn new(d0: u32, d1: u32, d2: u32) -> u32_u32_u32_float {
        u32_u32_u32_float { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::UNSIGNED_INT,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u32, u32, u32)> for u32_u32_u32_float {
    fn from(other: (u32, u32, u32)) -> Self {
        u32_u32_u32_float::new(other.0, other.1, other.2)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u32_u32_u32_u32_float {
    pub d0: u32,
    pub d1: u32,
    pub d2: u32,
    pub d3: u32,
}

impl u32_u32_u32_u32_float {
    pub fn new(d0: u32, d1: u32, d2: u32, d3: u32) -> u32_u32_u32_u32_float {
        u32_u32_u32_u32_float { d0, d1, d2, d3 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::UNSIGNED_INT,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u32, u32, u32, u32)> for u32_u32_u32_u32_float {
    fn from(other: (u32, u32, u32, u32)) -> Self {
        u32_u32_u32_u32_float::new(other.0, other.1, other.2, other.3)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f16_ {
    pub d0: ::half::f16,
}

impl f16_ {
    pub fn new(d0: ::half::f16) -> f16_ {
        f16_ { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            1,
            gl::HALF_FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<::half::f16> for f16_ {
    fn from(other: ::half::f16) -> Self {
        f16_::new(other)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f16_f16 {
    pub d0: ::half::f16,
    pub d1: ::half::f16,
}

impl f16_f16 {
    pub fn new(d0: ::half::f16, d1: ::half::f16) -> f16_f16 {
        f16_f16 { d0, d1 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            2,
            gl::HALF_FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(::half::f16, ::half::f16)> for f16_f16 {
    fn from(other: (::half::f16, ::half::f16)) -> Self {
        f16_f16::new(other.0, other.1)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f16_f16_f16 {
    pub d0: ::half::f16,
    pub d1: ::half::f16,
    pub d2: ::half::f16,
}

impl f16_f16_f16 {
    pub fn new(d0: ::half::f16, d1: ::half::f16, d2: ::half::f16) -> f16_f16_f16 {
        f16_f16_f16 { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::HALF_FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(::half::f16, ::half::f16, ::half::f16)> for f16_f16_f16 {
    fn from(other: (::half::f16, ::half::f16, ::half::f16)) -> Self {
        f16_f16_f16::new(other.0, other.1, other.2)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f16_f16_f16_f16 {
    pub d0: ::half::f16,
    pub d1: ::half::f16,
    pub d2: ::half::f16,
    pub d3: ::half::f16,
}

impl f16_f16_f16_f16 {
    pub fn new(
        d0: ::half::f16,
        d1: ::half::f16,
        d2: ::half::f16,
        d3: ::half::f16,
    ) -> f16_f16_f16_f16 {
        f16_f16_f16_f16 { d0, d1, d2, d3 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::HALF_FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(::half::f16, ::half::f16, ::half::f16, ::half::f16)> for f16_f16_f16_f16 {
    fn from(other: (::half::f16, ::half::f16, ::half::f16, ::half::f16)) -> Self {
        f16_f16_f16_f16::new(other.0, other.1, other.2, other.3)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_ {
    pub d0: f32,
}

impl f32_ {
    pub fn new(d0: f32) -> f32_ {
        f32_ { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            1,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<f32> for f32_ {
    fn from(other: f32) -> Self {
        f32_::new(other)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32 {
    pub d0: f32,
    pub d1: f32,
}

impl f32_f32 {
    pub fn new(d0: f32, d1: f32) -> f32_f32 {
        f32_f32 { d0, d1 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f32, f32)> for f32_f32 {
    fn from(other: (f32, f32)) -> Self {
        f32_f32::new(other.0, other.1)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
    pub d3: f32,
}

impl f32_f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32, d3: f32) -> f32_f32_f32_f32 {
        f32_f32_f32_f32 { d0, d1, d2, d3 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f32, f32, f32, f32)> for f32_f32_f32_f32 {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        f32_f32_f32_f32::new(other.0, other.1, other.2, other.3)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f64_ {
    pub d0: f64,
}

impl f64_ {
    pub fn new(d0: f64) -> f64_ {
        f64_ { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribLPointer(
            location as gl::types::GLuint,
            1,
            gl::DOUBLE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<f64> for f64_ {
    fn from(other: f64) -> Self {
        f64_::new(other)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f64_f64 {
    pub d0: f64,
    pub d1: f64,
}

impl f64_f64 {
    pub fn new(d0: f64, d1: f64) -> f64_f64 {
        f64_f64 { d0, d1 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribLPointer(
            location as gl::types::GLuint,
            2,
            gl::DOUBLE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f64, f64)> for f64_f64 {
    fn from(other: (f64, f64)) -> Self {
        f64_f64::new(other.0, other.1)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f64_f64_f64 {
    pub d0: f64,
    pub d1: f64,
    pub d2: f64,
}

impl f64_f64_f64 {
    pub fn new(d0: f64, d1: f64, d2: f64) -> f64_f64_f64 {
        f64_f64_f64 { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribLPointer(
            location as gl::types::GLuint,
            3,
            gl::DOUBLE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f64, f64, f64)> for f64_f64_f64 {
    fn from(other: (f64, f64, f64)) -> Self {
        f64_f64_f64::new(other.0, other.1, other.2)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f64_f64_f64_f64 {
    pub d0: f64,
    pub d1: f64,
    pub d2: f64,
    pub d3: f64,
}

impl f64_f64_f64_f64 {
    pub fn new(d0: f64, d1: f64, d2: f64, d3: f64) -> f64_f64_f64_f64 {
        f64_f64_f64_f64 { d0, d1, d2, d3 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribLPointer(
            location as gl::types::GLuint,
            4,
            gl::DOUBLE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f64, f64, f64, f64)> for f64_f64_f64_f64 {
    fn from(other: (f64, f64, f64, f64)) -> Self {
        f64_f64_f64_f64::new(other.0, other.1, other.2, other.3)
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i2_i10_i10_i10_rev {
    pub inner: u32,
}

impl i2_i10_i10_i10_rev {
    pub fn new(inner: u32) -> i2_i10_i10_i10_rev {
        i2_i10_i10_i10_rev { inner }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::INT_2_10_10_10_REV,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u2_u10_u10_u10_rev {
    pub inner: ::vec_2_10_10_10::Vector,
}

impl u2_u10_u10_u10_rev {
    pub fn new(inner: ::vec_2_10_10_10::Vector) -> u2_u10_u10_u10_rev {
        u2_u10_u10_u10_rev { inner }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::UNSIGNED_INT_2_10_10_10_REV,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f32, f32, f32, f32)> for u2_u10_u10_u10_rev {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        u2_u10_u10_u10_rev {
            inner: ::vec_2_10_10_10::Vector::new(other.0, other.1, other.2, other.3),
        }
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u10_u11_u11_rev {
    pub inner: u32,
}

impl u10_u11_u11_rev {
    pub fn new(inner: u32) -> u10_u11_u11_rev {
        u10_u11_u11_rev { inner }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::UNSIGNED_INT_10F_11F_11F_REV,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i2_i10_i10_i10_rev_float {
    pub inner: u32,
}

impl i2_i10_i10_i10_rev_float {
    pub fn new(inner: u32) -> i2_i10_i10_i10_rev_float {
        i2_i10_i10_i10_rev_float { inner }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::INT_2_10_10_10_REV,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

// ---------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u10_u11_u11_rev_float {
    pub inner: u32,
}

impl u10_u11_u11_rev_float {
    pub fn new(inner: u32) -> u10_u11_u11_rev_float {
        u10_u11_u11_rev_float { inner }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::UNSIGNED_INT_10F_11F_11F_REV,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}
