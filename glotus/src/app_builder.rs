use std::{cell::RefCell, collections::HashMap, rc::Rc};

use log::error;

use crate::{
    app::App,
    material::UniformValue,
    mesh::vertex::Vertex,
    texture::{FilteringMode, WrappingMode},
    transform::Transform,
};

pub struct AppBuilder {
    app: Rc<RefCell<App>>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            app: Rc::new(RefCell::new(App::new())),
        }
    }

    pub fn init_window(&mut self, width: u32, height: u32) -> &mut Self {
        self.app.borrow_mut().init_window(width, height);
        self
    }

    pub fn create_shader_from_source(
        &mut self,
        shader_name: &str,
        vertex_source: &str,
        fragment_source: &str,
    ) -> &mut Self {
        self.app.borrow_mut().create_shader_from_source(
            shader_name,
            vertex_source,
            fragment_source,
        );
        self
    }

    pub fn create_shader_from_file(
        &mut self,
        shader_name: &str,
        vertex_path: &str,
        fragment_path: &str,
    ) -> &mut Self {
        self.app
            .borrow_mut()
            .create_shader_from_file(shader_name, vertex_path, fragment_path);
        self
    }

    pub fn create_material(
        &mut self,
        material_name: &str,
        shader_name: &str,
        uniforms: HashMap<String, UniformValue>,
        textures: HashMap<String, u32>,
    ) -> &mut Self {
        self.app
            .borrow_mut()
            .create_material(material_name, shader_name, uniforms, textures);
        self
    }

    pub fn create_texture(
        &mut self,
        texture_name: &str,
        path: &str,
        wrapping_mode_s: WrappingMode,
        wrapping_mode_t: WrappingMode,
        filtering_mode_min: FilteringMode,
        filtering_mode_mag: FilteringMode,
    ) -> &mut Self {
        self.app.borrow_mut().create_texture(
            texture_name,
            path,
            wrapping_mode_s,
            wrapping_mode_t,
            filtering_mode_min,
            filtering_mode_mag,
        );
        self
    }

    pub fn create_mesh_from_data(
        &mut self,
        mesh_name: &str,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) -> &mut Self {
        self.app
            .borrow_mut()
            .create_mesh_from_data(mesh_name, vertices, indices);
        self
    }

    pub fn create_entity(
        &mut self,
        entity_name: &str,
        transform: Transform,
        material_name: &str,
        mesh_name: &str,
    ) -> &mut Self {
        self.app
            .borrow_mut()
            .create_entity(entity_name, transform, material_name, mesh_name);
        self
    }

    pub fn set_camera_transform(&mut self, transform: Transform) -> &mut Self {
        self.app.borrow_mut().set_camera_transform(transform);
        self
    }

    pub fn run(&mut self) {
        self.app.borrow_mut().run();
    }
}
