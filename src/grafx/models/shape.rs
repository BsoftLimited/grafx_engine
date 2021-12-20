use crate::grafx::{physics::Color, resources::getBoxVertices};
use gl::types::{ GLint, GLsizei, GLsizeiptr, GLfloat};
use std::{ mem, ptr};
use std::os::raw::c_void;

#[allow(non_snake_case)]
pub struct Shape{ vao: u32, size: i32,}

#[allow(non_snake_case)]
impl Shape{
    pub fn getVertexArrayBuffer(&self)->u32{ self.vao}
    pub fn getIndexCount(&self)->i32{ self.size }

    pub fn Box()->Self{
        let (vertices, indices) = getBoxVertices(1.0, 1.0, 1.0, Color::White());
        let (mut vbo, mut ebo, mut vao) = (0, 0, 0);
        unsafe{
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, &vertices[0] as *const f32 as *const c_void, gl::STATIC_DRAW, );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * mem::size_of::<GLint>()) as GLsizeiptr, &indices[0] as *const u32 as *const c_void, gl::STATIC_DRAW,);

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 12 * mem::size_of::<GLfloat>() as GLsizei, ptr::null(), );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 12 * mem::size_of::<GLfloat>() as GLsizei, (3 * mem::size_of::<GLfloat>()) as *const c_void);

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2, 4, gl::FLOAT, gl::FALSE, 12 * mem::size_of::<GLfloat>() as GLsizei, (6 * mem::size_of::<GLfloat>()) as *const c_void);

            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(3, 2, gl::FLOAT, gl::FALSE, 12 * mem::size_of::<GLfloat>() as GLsizei, (10 * mem::size_of::<GLfloat>()) as *const c_void);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Shape{ vao, size:indices.len() as i32,}
    }
}