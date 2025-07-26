use std::collections::HashMap;

use super::uniform_value::UniformValue;

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
