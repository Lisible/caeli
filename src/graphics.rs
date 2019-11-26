/*
* MIT License
*
* Copyright (c) 2019 Caeli team
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

use std::ffi::CString;

use gl;

const VERTEX_SHADER_SOURCE: &'static str = r#"
#version 330 core
layout (location = 0) in vec3 vertexPosition;
layout (location = 1) in vec3 vertexColor;

out vec4 color;

void main()
{
    color = vec4(vertexColor, 1.0);
    gl_Position = vec4(vertexPosition, 1.0);
}
"#;

const FRAGMENT_SHADER_SOURCE: &'static str = r#"
#version 330 core
out vec4 FragColor;

in vec4 color;

void main()
{
    FragColor = color;
} 
"#;

pub struct Renderer {
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    shader_program: gl::types::GLuint,
    pending_commands: Vec<RenderCommand>
}

impl Renderer {
    pub fn new() -> Renderer {
        const MESH_BUFFER_SIZE: isize = 60000;

        // Creating VAO
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao); } 

        // Creating VBO and pre-fill it
        let mut vbo = 0;
        unsafe { 
            gl::GenBuffers(1, &mut vbo); 
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 
                MESH_BUFFER_SIZE as gl::types::GLsizeiptr,
                std::ptr::null() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        // Compiling vertex shader
        let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        let vertex_shader_source_string = CString::new(VERTEX_SHADER_SOURCE).unwrap();
        unsafe {
            gl::ShaderSource(vertex_shader, 1, &vertex_shader_source_string.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex_shader);
        }

        // Compiling fragment shader
        let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        let fragment_shader_source_string = CString::new(FRAGMENT_SHADER_SOURCE).unwrap();
        unsafe {
            gl::ShaderSource(fragment_shader, 1, &fragment_shader_source_string.as_ptr(), std::ptr::null());
            gl::CompileShader(fragment_shader);
        }

        // Creating shader program
        let shader_program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program); 
        }

        let mut success = 1;
        unsafe {
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut length = 0;
            unsafe {
                gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut length);
            }

            let mut buffer = Vec::with_capacity(length as usize + 1);
            buffer.extend([b' '].iter().cycle().take(length as usize));
            let error = unsafe { CString::from_vec_unchecked(buffer) };

            unsafe {
                gl::GetProgramInfoLog(shader_program,
                                      length,
                                      std::ptr::null_mut(),
                                      error.as_ptr() as *mut gl::types::GLchar);
            }

            panic!(error.to_string_lossy().into_owned());
        }

        // Getting rid of shader objects
        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }
       
        // Setting up VAO
        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 
                6 * std::mem::size_of::<f32>() as gl::types::GLint, 
                std::ptr::null() as *const gl::types::GLvoid);

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 
                6 * std::mem::size_of::<f32>() as gl::types::GLint, 
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid);
            gl::BindVertexArray(0);
        }

        Renderer {
            vao,
            vbo,
            shader_program,
            pending_commands: vec!()
        }
    }

    pub fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, 
        color: (f32, f32, f32)) {
        let vertices = [
            x, y, 0.0, color.0, color.1, color.2,
            x + width, y, 0.0, color.0, color.1, color.2,
            x, y + height, 0.0, color.0, color.1, color.2,
            x, y + height, 0.0, color.0, color.1, color.2,
            x + width, y, 0.0, color.0, color.1, color.2,
            x + width, y + height, 0.0, color.0, color.1, color.2
        ];

        self.pending_commands.push(RenderCommand {
            vertices: vertices.to_vec(),
            mode: RenderMode::Triangle
        });
    }

    pub fn draw_line(&mut self, first_point: (f32, f32), second_point: (f32, f32), 
        color: (f32, f32, f32)) {
        let vertices = [
            first_point.0, first_point.1, 0.0, color.0, color.1, color.2,
            second_point.0, second_point.1, 0.0, color.0, color.1, color.2
        ];

        self.pending_commands.push(RenderCommand {
            vertices: vertices.to_vec(),
            mode: RenderMode::Line
        });
    }

    pub fn render(&mut self) {
        for command in self.pending_commands.iter() {
            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                gl::BufferSubData(gl::ARRAY_BUFFER,
                    0 as gl::types::GLintptr,
                    (command.vertices.len() * std::mem::size_of::<f32>() * 6) as gl::types::GLsizeiptr,
                    command.vertices.as_ptr() as *const gl::types::GLvoid);
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);


                gl::UseProgram(self.shader_program);

                gl::BindVertexArray(self.vao);

                let draw_mode = match command.mode {
                    RenderMode::Triangle => gl::TRIANGLES,
                    RenderMode::Line => gl::LINES
                };

                gl::DrawArrays(draw_mode, 0, (command.vertices.len() / 6) as gl::types::GLsizei);
                gl::BindVertexArray(0);
            }
        }

        self.pending_commands.clear();
    }

    pub fn set_clear_color(&self, red: f32, green: f32, blue: f32) {
        unsafe { gl::ClearColor(red, green, blue, 1.0); }
    }

    pub fn clear(&self) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
    }
}

struct RenderCommand {
    pub vertices: Vec<f32>,
    pub mode: RenderMode
}

enum RenderMode {
    Triangle,
    Line
}
