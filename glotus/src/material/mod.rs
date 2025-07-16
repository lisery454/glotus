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
}

impl Material {
    pub fn new(shader_name: &str, uniforms: HashMap<String, UniformValue>) -> Self {
        Self {
            shader_name: shader_name.to_string(),
            uniforms,
        }
    }

    pub fn set_uniform(&mut self, name: &str, value: UniformValue) {
        self.uniforms.insert(name.to_string(), value);
    }

    pub fn bind(&self, shader: &mut Shader) {
        shader.bind();
        // 设置所有uniforms
        for (name, value) in &self.uniforms {
            match value {
                UniformValue::Float(v) => shader.set_uniform_f32(name, *v),
                UniformValue::Int(v) => shader.set_uniform_i32(name, *v),
                UniformValue::Vector3(v) => shader.set_uniform_vec3(name, v),
                UniformValue::Vector4(v) => shader.set_uniform_vec4(name, v),
                UniformValue::Matrix4(m) => shader.set_uniform_mat4(name, m),
                UniformValue::Texture(slot) => shader.set_uniform_i32(name, *slot as i32),
            }
        }
    }

    pub fn unbind(&self, shader: &mut Shader) {
        shader.unbind();
    }
}
