use super::shader::Program;
use super::texture::Texture;
use gl::types::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    unsafe fn vertex_attrib_pointer() {
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as GLsizei,
            (2 * std::mem::size_of::<f32>()) as *const GLvoid,
        );
    }
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    num_indices: i32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: &[u32]) -> Self {
        let (mut vao, mut vbo, mut ebo) = (0, 0, 0);
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // vertex buffer
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                gl::DYNAMIC_DRAW,
            );
            Vertex::vertex_attrib_pointer();

            // index buffer
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
                indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindVertexArray(0);
        }

        Self {
            vertices,
            vao,
            vbo,
            ebo,
            num_indices: indices.len() as i32,
        }
    }

    pub fn render(&self, program: &mut Program, texture: &Texture) {
        program.set_int("texture0", 0);
        texture.bind(0);

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.num_indices,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }

    pub fn update_data(&mut self, vertices: Vec<Vertex>) {
        self.vertices = vertices;
        unsafe {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (self.vertices.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
                self.vertices.as_ptr() as *const GLvoid,
            );
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.ebo);
            gl::DeleteBuffers(1, &mut self.vbo);
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
    }
}
