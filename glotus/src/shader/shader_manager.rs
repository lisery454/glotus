use std::{collections::HashMap, path::Path};

use log::{error, info};

use super::shader::Shader;

pub struct ShaderManager {
    shaders: HashMap<String, Shader>,
}

impl ShaderManager {
    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
        }
    }

    pub fn has(&self, shader_name: &str) -> bool{
        self.shaders.contains_key(shader_name)
    }

    pub fn get(&self, shader_name: &str) -> Option<&Shader> {
        self.shaders.get(shader_name)
    }

    pub fn create_shader_from_source(
        &mut self,
        shader_name: &str,
        vertex_source: &str,
        fragment_source: &str,
    ) {
        match Shader::from_sources(vertex_source, fragment_source) {
            Ok(s) => {
                info!("success add shader <{:?}>", shader_name);
                self.shaders.insert(shader_name.to_string(), s);
            }
            Err(e) => {
                error!("{:}", e);
            }
        }
    }

    pub fn create_shader_from_file(
        &mut self,
        shader_name: &str,
        vertex_path: &str,
        fragment_path: &str,
    ) {
        match Shader::from_files(Path::new(vertex_path), Path::new(fragment_path)) {
            Ok(s) => {
                info!("success add shader <{:?}>", shader_name);
                self.shaders.insert(shader_name.to_string(), s);
            }
            Err(e) => {
                error!("{:}", e);
            }
        }
    }
}
