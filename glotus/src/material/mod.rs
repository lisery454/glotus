use std::{collections::HashMap, path::Path};

use cgmath::{Matrix3, Matrix4, Vector3, Vector4};

use crate::shader::{Shader, ShaderError};

#[derive(Debug)]
pub enum UniformValue {
    Float(f32),
    Int(i32),
    Vector3(Vector3<f32>),
    Vector4(Vector4<f32>),
    Matrix3(Matrix3<f32>),
    Matrix4(Matrix4<f32>),
    Texture(usize), // 纹理槽位
}

impl UniformValue {
    pub fn get_float(value: f32) -> UniformValue {
        UniformValue::Float(value)
    }

    pub fn get_int(value: i32) -> UniformValue {
        UniformValue::Int(value)
    }

    pub fn get_vector3_f32(x: f32, y: f32, z: f32) -> UniformValue {
        UniformValue::Vector3(Vector3 { x, y, z })
    }

    pub fn get_vector4_f32(x: f32, y: f32, z: f32, w: f32) -> UniformValue {
        UniformValue::Vector4(Vector4 { x, y, z, w })
    }

    pub fn get_texture(slot_id: usize) -> UniformValue {
        UniformValue::Texture(slot_id)
    }
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
