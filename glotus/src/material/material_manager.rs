use std::{cell::RefCell, collections::HashMap, rc::Weak};

use log::{error, info};

use crate::shader::ShaderManager;

use super::{material::Material, uniform_value::UniformValue};

pub struct MaterialManager {
    materials: HashMap<String, Material>,
    shader_manager: Weak<RefCell<ShaderManager>>,
}

impl MaterialManager {
    pub fn new(shader_manager: Weak<RefCell<ShaderManager>>) -> Self {
        Self {
            materials: HashMap::new(),
            shader_manager,
        }
    }

    pub fn has(&self, material_name: &str) -> bool {
        self.materials.contains_key(material_name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Material> {
        self.materials.get_mut(name)
    }

    pub fn get(&self, name: &str) -> Option<&Material> {
        self.materials.get(name)
    }

    pub fn create_material(
        &mut self,
        material_name: &str,
        shader_name: &str,
        uniforms: HashMap<String, UniformValue>,
        textures: HashMap<String, u32>,
    ) {
        if let Some(shader_manager) = self.shader_manager.upgrade() {
            if shader_manager.borrow().has(shader_name) {
                let material = Material::new(shader_name, uniforms, textures);
                info!("success add shader <{:?}>", material_name);
                self.materials.insert(material_name.to_string(), material);
            } else {
                error!(
                    "fail add material <{:?}>, because shader <{:?}> not exists",
                    material_name, shader_name
                );
            }
        } else {
            error!("shader_manager has been dropped");
        }
    }
}
