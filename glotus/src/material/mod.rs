use std::{collections::HashMap, path::Path};

use cgmath::{Matrix4, Vector3, Vector4};

use crate::shader::{Shader, ShaderError};

#[derive(Debug)]
pub enum UniformValue {
    Float(f32),
    Int(i32),
    Vector3(Vector3<f32>),
    Vector4(Vector4<f32>),
    Matrix4(Matrix4<f32>),
    Texture(usize), // 纹理槽位
}

#[derive(Debug)]
pub struct Material {
    pub shader_name: String,
    pub uniforms: HashMap<String, UniformValue>,
    pub textures: HashMap<String, u32>,
}

impl Material {
    pub fn new(
        shader_name: &str,
        uniforms: HashMap<String, UniformValue>,
        textures: HashMap<String, u32>,
    ) -> Self {
        Self {
            shader_name: shader_name.to_string(),
            uniforms,
            textures,
        }
    }

    pub fn insert_uniform(&mut self, name: &str, value: UniformValue) {
        self.uniforms.insert(name.to_string(), value);
    }
}
