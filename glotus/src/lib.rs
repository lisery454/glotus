pub mod camera;
pub mod entity;
mod log_builder;
pub mod material;
pub mod mesh;
pub mod shader;
pub mod transform;

use camera::Camera;
use cgmath::Matrix4;
use cgmath::Zero;
use chrono::DateTime;
use chrono::Local;
use entity::Entity;
use gl::types::*;
use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};
use log::debug;
use log::error;
use log::info;
use material::Material;
use material::UniformValue;
use mesh::Mesh;
use mesh::vertex::Vertex;
use shader::Shader;
use std::collections::HashMap;
use std::ffi::CString;
use std::iter::Map;
use std::mem;
use std::path::Path;
use std::ptr;
use transform::Transform;

pub struct App {
    is_running: bool,
    window: Option<PWindow>,
    glfw: Option<Glfw>,
    event_receiver: Option<GlfwReceiver<(f64, WindowEvent)>>,
    delta_time: f32,
    last_time: DateTime<Local>,

    shaders: HashMap<String, Shader>,
    meshes: HashMap<String, Mesh>,
    materials: HashMap<String, Material>,
    entities: HashMap<String, Entity>,

    camera: Camera,
}

impl App {
    pub fn new() -> Self {
        let app = Self {
            is_running: false,
            window: None,
            glfw: None,
            event_receiver: None,
            delta_time: 0.0,
            last_time: Local::now(),

            shaders: HashMap::new(),
            meshes: HashMap::new(),
            materials: HashMap::new(),
            entities: HashMap::new(),

            camera: Camera::new(Transform::new()),
        };

        log_builder::setup_logger();

        app
    }

    pub fn init_window(&mut self, width: u32, height: u32) -> &mut Self {
        // 初始化 GLFW
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        // 设置窗口提示
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        // 创建窗口
        let (mut window, events) = glfw
            .create_window(
                width,
                height,
                "Rust GLFW opengl",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");

        // 设置窗口为当前上下文
        window.make_current();
        window.set_key_polling(true); // 监听键盘输入
        window.set_scroll_polling(true); // 监听滚轮事件
        window.set_cursor_pos_polling(true); // 监听鼠标移动事件
        window.set_framebuffer_size_polling(true); // 监听窗口大小变化
        window.set_close_polling(true);

        // 加载 OpenGL 函数指针
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        self.glfw = Some(glfw);
        self.window = Some(window);
        self.event_receiver = Some(events);

        self
    }

    pub fn create_shader_from_source(
        &mut self,
        shader_name: &str,
        vertex_source: &str,
        fragment_source: &str,
    ) -> &mut Self {
        match shader::Shader::from_sources(vertex_source, fragment_source) {
            Ok(s) => {
                info!("success add shader <{:?}>", shader_name);
                self.shaders.insert(shader_name.to_string(), s);
            }
            Err(e) => {
                error!("{:}", e);
            }
        }

        self
    }

    pub fn create_shader_from_file(
        &mut self,
        shader_name: &str,
        vertex_path: &str,
        fragment_path: &str,
    ) -> &mut Self {
        match shader::Shader::from_files(Path::new(vertex_path), Path::new(fragment_path)) {
            Ok(s) => {
                info!("success add shader <{:?}>", shader_name);
                self.shaders.insert(shader_name.to_string(), s);
            }
            Err(e) => {
                error!("{:}", e);
            }
        }

        self
    }

    pub fn create_material(
        &mut self,
        material_name: &str,
        shader_name: &str,
        uniforms: HashMap<String, UniformValue>,
    ) -> &mut Self {
        if !self.shaders.contains_key(shader_name) {
            error!(
                "fail add material <{:?}>, because shader <{:?}> not exists",
                material_name, shader_name
            );
        } else {
            let material = Material::new(shader_name, uniforms);
            info!("success add shader <{:?}>", material_name);
            self.materials.insert(material_name.to_string(), material);
        }
        self
    }

    pub fn create_mesh_from_data(
        &mut self,
        mesh_name: &str,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) -> &mut Self {
        let mesh = mesh::Mesh::new(vertices, indices);

        info!("success add mesh <{:?}>", mesh_name);
        self.meshes.insert(mesh_name.to_string(), mesh);

        self
    }

    pub fn create_entity(
        &mut self,
        entity_name: &str,
        transform: Transform,
        material_name: &str,
        mesh_name: &str,
    ) -> &mut Self {
        if !self.materials.contains_key(material_name) {
            error!(
                "fail add entity <{:?}>, because material <{:?}> not exists",
                entity_name, material_name
            );
        } else if !self.meshes.contains_key(mesh_name) {
            error!(
                "fail add entity <{:?}>, because mesh <{:?}> not exists",
                entity_name, mesh_name
            );
        } else {
            let entity = Entity::new(transform, material_name, mesh_name);
            info!("success add entity <{:?}>", entity_name);
            self.entities.insert(entity_name.to_string(), entity);
        }

        self
    }

    pub fn run(&mut self) -> () {
        self.is_running = true;

        info!("app starts to running...");

        while self.is_running {
            self.calc_delta_time();

            self.glfw.as_mut().unwrap().poll_events();
            self.handle_window_event();

            self.render();

            self.window.as_mut().unwrap().swap_buffers();
        }

        info!("app is going to close...");
    }

    fn calc_delta_time(&mut self) {
        let current_time = Local::now();
        self.delta_time = (current_time - self.last_time).abs().as_seconds_f32();
        self.last_time = current_time;
    }

    fn render(&mut self) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // 每帧清除深度缓冲

            // let view_matrix = self.camera.transform.get_view_matrix();
            // let projection_matrix = self.camera.lock().unwrap().get_projection_matrix();
            let view_matrix = Matrix4::zero();
            let projection_matrix = Matrix4::zero();

            for (entity_name, entity) in &mut self.entities {
                let model_matrix = entity.transform.to_matrix();

                let material_name = &entity.material_name;
                let material = self.materials.get_mut(material_name).unwrap();
                material.set_uniform("model_matrix", UniformValue::Matrix4(model_matrix));
                material.set_uniform("view_matrix", UniformValue::Matrix4(view_matrix));
                material.set_uniform(
                    "projection_matrix",
                    UniformValue::Matrix4(projection_matrix),
                );

                let shader_name = &material.shader_name;
                let shader = self.shaders.get_mut(shader_name).unwrap();

                material.bind(shader);

                let mesh_name = &entity.mesh_name;
                let mesh = self.meshes.get_mut(mesh_name).unwrap();
                mesh.draw();

                material.unbind(shader);
            }
        }
    }

    fn handle_window_event(&mut self) {
        for (_, event) in glfw::flush_messages(self.event_receiver.as_ref().unwrap()) {
            match event {
                WindowEvent::Key(key, _, action, _) => {
                    debug!("Trigger Key {:?} {:?}", key, action);
                }
                WindowEvent::Close => {
                    debug!("Trigger WindowClose");
                    self.window.as_mut().unwrap().set_should_close(true);
                    self.is_running = false;
                }
                WindowEvent::Scroll(xoffset, yoffset) => {
                    debug!("Trigger Mouse Scroll: X={}, Y={}", xoffset, yoffset);
                }
                WindowEvent::CursorPos(xpos, ypos) => {
                    debug!("Trigger Cursor Move: X={}, Y={}", xpos, ypos);
                }
                WindowEvent::MouseButton(button, action, _) => {
                    debug!("Trigger Mouse button: {:?}, Action: {:?}", button, action);
                }
                _ => (),
            };
        }
    }
}
