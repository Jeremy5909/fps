use std::{ffi::CString, fs};

use crate::{shader::Shader, whitespace_cstring_with_len};

mod uniform;

pub struct Program {
    id: gl::types::GLuint,
}
impl Program {
    pub(crate) fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };
        shaders
            .iter()
            .for_each(|shader| unsafe { gl::AttachShader(program_id, shader.id()) });

        unsafe { gl::LinkProgram(program_id) };

        shaders
            .iter()
            .for_each(|shader| unsafe { gl::DetachShader(program_id, shader.id()) });

        let mut success: gl::types::GLint = 1;
        unsafe { gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success) };
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Program { id: program_id })
    }
    pub fn from_name(name: &str) -> Result<Program, String> {
        let vert_source = fs::read_to_string(format!("{}.vert", name))
            .map_err(|_| format!("File {}.vert not found", name))?;
        eprintln!("Loaded shader {name}.vert");
        let frag_source = fs::read_to_string(format!("{}.frag", name))
            .map_err(|_| format!("File {}.frag not found", name))?;
        eprintln!("Loaded shader {name}.frag");

        let vert_cstring = CString::new(vert_source).map_err(|e| format!("{e}"))?;
        let frag_cstring = CString::new(frag_source).map_err(|e| format!("{e}"))?;

        let vert = Shader::from_source(&vert_cstring, gl::VERTEX_SHADER)?;
        let frag = Shader::from_source(&frag_cstring, gl::FRAGMENT_SHADER)?;

        Self::from_shaders(&[vert, frag])
    }
    pub(crate) fn set_used(&self) {
        unsafe { gl::UseProgram(self.id) };
    }
}
impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) };
    }
}
