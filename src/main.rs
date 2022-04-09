#[macro_use]
extern crate failure;

extern crate gl;
extern crate sdl2;

pub mod polys;
pub mod render_gl;
pub mod resources;

use crate::resources::Resources;
use failure::err_msg;
use std::path::Path;

fn main() {
    if let Err(e) = run() {
        println!("{}", failure_to_string(e))
    }
}

fn run() -> Result<(), failure::Error> {
    let res = Resources::from_relative_exe_path(Path::new("assets"))?;
    let sdl = sdl2::init().map_err(err_msg)?;
    let sdl_video = sdl.video().unwrap();

    let gl_attr = sdl_video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);

    let window = sdl_video
        .window("OpenGL Test", 960, 640)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| sdl_video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    use std::ffi::CString;

    /* // shaders from source
    let vert_shader =
        render_gl::Shader::from_vert_src(&gl, &CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();
    let frag_shader =
        render_gl::Shader::from_frag_src(&gl, &CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();
    let shader_program = render_gl::Program::from_shaders(&gl, &[vert_shader, frag_shader]).unwrap();
    */
    // shaders via res
    let shader_program = render_gl::Program::from_res(&gl, &res, "shaders/triangle")?;
    shader_program.set_used();

    let vertices: Vec<f32> = vec![
        // Positions        //Colors
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, //bottom right
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0, //bottom left
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, //top
    ];
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0)
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
    }
    unsafe {
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl.EnableVertexAttribArray(0); // location data
        gl.VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null(),
        );
        gl.EnableVertexAttribArray(1); // color data
        gl.VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );

        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

    unsafe {
        gl.Viewport(0, 0, 960, 640);
        gl.ClearColor(0.5, 0.75, 0.9, 1.0);
    }
    let mut sdl_event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in sdl_event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();
        unsafe {
            gl.BindVertexArray(vao);
            gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.gl_swap_window();
    }
    Ok(())
}

pub fn failure_to_string(e: failure::Error) -> String {
    use std::fmt::Write;

    let mut result = String::new();

    for (i, cause) in e
        .iter_chain()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .enumerate()
    {
        if i > 0 {
            let _ = writeln!(&mut result, "\tWhich caused the following issue:");
        }
        let _ = write!(&mut result, "{}", cause);
        if let Some(backtrace) = cause.backtrace() {
            let backtrace_str = format!("{}", backtrace);
            if backtrace_str.len() > 0 {
                let _ = writeln!(&mut result, "\tThis happened at {}", backtrace);
            } else {
                let _ = writeln!(&mut result);
            }
        } else {
            let _ = writeln!(&mut result);
        }
    }

    result
}
