use crate::resources::{self, Resources};
use gl;
use std;
use std::ffi::{CStr, CString};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to load resource {}", name)]
    ResourceLoad {
        name: String,
        #[cause]
        inner: resources::Error,
    },
    #[fail(display = "Failed to link program {}: {}", name, message)]
    LinkError { name: String, message: String },
    #[fail(display = "Failed to complie shader {}: {}", name, message)]
    CompileError { name: String, message: String },
    #[fail(display = "Cannot determine shader type for resource {}", name)]
    UnknownShaderType { name: String },
}

pub struct Program {
    gl: gl::Gl,
    program_id: gl::types::GLuint,
}

impl Program {
    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Program, Error> {
        const POSSIBLE_EXT: [&str; 2] = [".vert", ".frag"];

        let resource_names = POSSIBLE_EXT
            .iter()
            .map(|file_extension| format!("{}{}", name, file_extension))
            .collect::<Vec<String>>();
        let shaders = resource_names
            .iter()
            .map(|resource_name| Shader::from_res(gl, res, resource_name))
            .collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(gl, &shaders[..]).map_err(|message| Error::LinkError {
            name: name.into(),
            message,
        })
    }

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
    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Shader, Error> {
        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] =
            [(".vert", gl::VERTEX_SHADER), (".frag", gl::FRAGMENT_SHADER)];
        let shader_type = POSSIBLE_EXT
            .iter()
            .find(|&&(file_extension, _)| name.ends_with(file_extension))
            .map(|&(_, s_type)| s_type)
            .ok_or_else(|| Error::UnknownShaderType { name: name.into() })?;
        let src = res.load_cstring(name).map_err(|e| Error::ResourceLoad {
            name: name.into(),
            inner: e,
        })?;

        Shader::from_src(gl, &src, shader_type).map_err(|message| Error::CompileError {
            name: name.into(),
            message,
        })
    }

    pub fn from_src(
        gl: &gl::Gl,
        src: &CStr,
        shader_type: gl::types::GLenum,
    ) -> Result<Shader, String> {
        let shader_id = shader_from_src(gl, src, shader_type)?;
        Ok(Shader {
            gl: gl.clone(),
            shader_id,
        })
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
