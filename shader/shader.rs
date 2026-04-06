use crate::math::mat4vf::Mat4vf;
use gl::types::*;
// use glam::Mat4;
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum Uniform {
    Projection,
    View,
    Model,
    _Color,
    TextureArray1,
    TextureArray2,
    TextureArray3,
    Texture,
    _UseArray,
    Layer,
    Mvp,
}

pub struct Shader {
    pub id: u32,
    pub locs: [u8; 16],
}

impl Shader {
    pub fn new(vs: &str, fs: &str) -> Self {
        let program = { Self::init_shaders(vs, fs) };
        Self {
            id: program,
            locs: [0u8; 16],
        }
    }
    fn init_shaders(vs: &str, fs: &str) -> u32 {
        unsafe {
            // println!("!&&&&&");
            let vs = Self::compile_shader(vs, gl::VERTEX_SHADER);
            let fs = Self::compile_shader(fs, gl::FRAGMENT_SHADER);
            let prog = gl::CreateProgram();
            gl::AttachShader(prog, vs);
            gl::AttachShader(prog, fs);
            gl::LinkProgram(prog);
            let mut success = 0;
            gl::GetProgramiv(prog, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v = Vec::with_capacity(1024);
                let mut l = 0;
                gl::GetProgramInfoLog(prog, 1024, &mut l, v.as_mut_ptr() as *mut i8);
                v.set_len(l as usize);
                panic!("Failed LINKING!: {}", String::from_utf8_lossy(&v));
            }
            prog
        }
    }

    pub fn compile_shader(src: &str, kind: gl::types::GLenum) -> u32 {
        unsafe {
            let s = gl::CreateShader(kind);

            let ptr = src.as_ptr() as *const i8;
            let len = src.len() as i32;

            // Передаем указатель на строку и её длину (чтобы OpenGL не искал \0)
            gl::ShaderSource(s, 1, &ptr, &len);
            gl::CompileShader(s);

            s
        }
    }

    pub fn use_id(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_int(&self, uniform: Uniform, val: GLint) {
        let idx = uniform as usize;
        if let Some(n) = self.locs.get(idx) {
            unsafe {
                gl::Uniform1i(*n as i32, val);
            }
        }
    }
    // pub fn set_float(&self, uniform: Uniform, val: GLfloat) {
    //     let idx = uniform as usize;
    //     if let Some(n) = self.locs.get(idx) {
    //         unsafe {
    //             gl::Uniform1f(*n as i32, val);
    //         }
    //     }
    // }
    pub fn set_mat4x4(&self, uniform: Uniform, val: &Mat4vf) {
        let idx = uniform as usize;
        if let Some(n) = self.locs.get(idx) {
            unsafe {
                // Передаем указатель на первую колонку, приведенный к f32
                gl::UniformMatrix4fv(*n as i32, 1, gl::FALSE, val.cols.as_ptr() as *const f32);
            }
        }
    }
    // pub fn set_vec4(&self, uniform: Uniform, val: &glam::Vec4) {
    //     let idx = uniform as usize;
    //     if let Some(n) = self.locs.get(idx) {
    //         unsafe {
    //             // Для Vec4 используем Uniform4fv
    //             // Аргументы: (location, count, data_ptr)
    //             gl::Uniform4fv(*n as i32, 1, val.as_ref().as_ptr());
    //         }
    //     }
    // }
    pub fn set_loc(&mut self, uniform: Uniform, location: u8) {
        let idx = uniform as usize;
        if let Some(n) = self.locs.get_mut(idx) {
            *n = location;
        }
    }
}
