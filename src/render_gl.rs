use gl;
use std;
use std::ffi::{CStr, CString};

pub struct Program {
    gl: gl::Gl,
    program_id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl.CreateProgram() };

        for shader in shaders {
            unsafe { gl.AttachShader(program_id, shader.id()) };
        }
        unsafe { gl.LinkProgram(program_id) };
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl.DetachShader(program_id, shader.id()) };
        }

        Ok(Program {
            gl: gl.clone(),
            program_id,
        })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.program_id
    }

    pub fn set_used(&self) {
        unsafe { self.gl.UseProgram(self.program_id) }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteProgram(self.program_id) }
    }
}

pub struct Shader {
    gl: gl::Gl,
    shader_id: gl::types::GLuint,
}

impl Shader {
    pub fn from_src(gl: &gl::Gl, src: &CStr, shader_type: gl::types::GLenum) -> Result<Shader, String> {
        let shader_id = shader_from_src(gl, src, shader_type)?;
        Ok(Shader { gl: gl.clone(), shader_id })
    }

    pub fn from_vert_src(gl: &gl::Gl, src: &CStr) -> Result<Shader, String> {
        Shader::from_src(gl, src, gl::VERTEX_SHADER)
    }
    pub fn from_frag_src(gl: &gl::Gl, src: &CStr) -> Result<Shader, String> {
        Shader::from_src(gl, src, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.shader_id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteShader(self.shader_id) }
    }
}

fn shader_from_src(
    gl: &gl::Gl,
    src: &CStr,
    shader_type: gl::types::GLenum,
) -> Result<gl::types::GLuint, String> {
    let shader_id = unsafe { gl.CreateShader(shader_type) };

    unsafe {
        gl.ShaderSource(shader_id, 1, &src.as_ptr(), std::ptr::null());
        gl.CompileShader(shader_id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl.GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
    }
    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl.GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error: CString = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl.GetShaderInfoLog(
                shader_id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }
        return Err(error.to_string_lossy().into_owned());
    }

    Ok(shader_id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));
    return unsafe { CString::from_vec_unchecked(buffer) };
}
