use gl::types::*;
use std::collections::HashMap;
use std::ffi::CString;

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn from_source(source: &str, kind: GLenum) -> Result<Self, String> {
        let source = CString::new(source).unwrap();
        let id = unsafe { gl::CreateShader(kind) };

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success = 0;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
        if success == 0 {
            let mut len = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = vec![b' '; len as usize];
            unsafe {
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
            }
            return Err(format!("{}", String::from_utf8(error).unwrap()));
        }

        Ok(Self { id })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct Program {
    id: GLuint,
    locations: HashMap<String, GLint>,
}

impl Program {
    pub fn from_sources(vert_source: &str, frag_source: &str) -> Result<Self, String> {
        let vert_shader = Shader::from_source(vert_source, gl::VERTEX_SHADER)?;
        let frag_shader = Shader::from_source(frag_source, gl::FRAGMENT_SHADER)?;

        let id = unsafe { gl::CreateProgram() };

        unsafe {
            gl::AttachShader(id, vert_shader.id());
            gl::AttachShader(id, frag_shader.id());
            gl::LinkProgram(id);
        }

        let mut success = 0;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut len = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = vec![b' '; len as usize];
            unsafe {
                gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
            }
            return Err(format!("{}", String::from_utf8(error).unwrap()));
        }

        unsafe {
            gl::DetachShader(id, vert_shader.id());
            gl::DetachShader(id, frag_shader.id());
        }

        Ok(Self {
            id,
            locations: HashMap::new(),
        })
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_int(&mut self, name: &str, value: i32) {
        let location = self
            .locations
            .entry(name.into())
            .or_insert(unsafe { gl::GetUniformLocation(self.id, name.as_ptr() as *const GLchar) });
        unsafe {
            gl::Uniform1i(*location, value);
        }
    }
}
