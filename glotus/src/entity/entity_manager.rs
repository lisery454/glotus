use std::{cell::RefCell, collections::HashMap, rc::Weak};

use log::{error, info};

use crate::{material::MaterialManager, mesh::MeshManager, transform::Transform};

use super::entity::Entity;

pub struct EntityManager {
    entities: HashMap<String, Entity>,

    mesh_manager: Weak<RefCell<MeshManager>>,
    material_manager: Weak<RefCell<MaterialManager>>,
}

impl EntityManager {
    pub fn new(
        mesh_manager: Weak<RefCell<MeshManager>>,
        material_manager: Weak<RefCell<MaterialManager>>,
    ) -> Self {
        Self {
            entities: HashMap::new(),
            mesh_manager,
            material_manager,
        }
    }

    pub fn iter_entities(&self) -> std::collections::hash_map::Iter<'_, String, Entity> {
        self.entities.iter()
    }

    pub fn create_entity(
        &mut self,
        entity_name: &str,
        transform: Transform,
        material_name: &str,
        mesh_name: &str,
    ) {
        if let Some(material_manager) = self.material_manager.upgrade() {
            if let Some(mesh_manager) = self.mesh_manager.upgrade() {
                if !material_manager.borrow().has(material_name) {
                    error!(
                        "fail add entity <{:?}>, because material <{:?}> not exists",
                        entity_name, material_name
                    );
                } else if !mesh_manager.borrow().has(mesh_name) {
                    error!(
                        "fail add entity <{:?}>, because mesh <{:?}> not exists",
                        entity_name, mesh_name
                    );
                } else {
                    let entity = Entity::new(transform, material_name, mesh_name);
                    info!("success add entity <{:?}>", entity_name);
                    self.entities.insert(entity_name.to_string(), entity);
                }
            } else {
                error!("mesh_manager has been dropped");
            }
        } else {
            error!("material_manager has been dropped");
        }
    }
}
