pub mod entity;
mod log_builder;
pub mod material;
pub mod mesh;
pub mod shader;
pub mod transform;

use gl::types::*;
use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};
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

pub struct App {
    is_running: bool,
    window: Option<PWindow>,
    glfw: Option<Glfw>,
    event_receiver: Option<GlfwReceiver<(f64, WindowEvent)>>,

    shaders: HashMap<String, Shader>,
    meshes: HashMap<String, Mesh>,
    materials: HashMap<String, Material>,
}

impl App {
    pub fn new() -> Self {
        let app = Self {
            is_running: false,
            window: None,
            glfw: None,
            event_receiver: None,

            shaders: HashMap::new(),
            meshes: HashMap::new(),
            materials: HashMap::new(),
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
                self.shaders.insert(shader_name.to_owned(), s);
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
                self.shaders.insert(shader_name.to_owned(), s);
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
            self.materials.insert(material_name.to_owned(), material);
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
        self.meshes.insert(mesh_name.to_owned(), mesh);

        self
    }

    pub fn run(&mut self) -> () {
        self.is_running = true;

        info!("app starts to running...");

        while self.is_running {
            self.glfw.as_mut().unwrap().poll_events();
            for (_, event) in glfw::flush_messages(self.event_receiver.as_ref().unwrap()) {
                match event {
                    WindowEvent::Key(key, _, action, _) => {
                        info!("Trigger Key {:?} {:?}", key, action);
                    }
                    WindowEvent::Close => {
                        info!("Trigger WindowClose");
                        self.window.as_mut().unwrap().set_should_close(true);
                        self.is_running = false;
                    }
                    WindowEvent::Scroll(xoffset, yoffset) => {
                        info!("Trigger Mouse Scroll: X={}, Y={}", xoffset, yoffset);
                    }
                    WindowEvent::CursorPos(xpos, ypos) => {
                        info!("Trigger Cursor Move: X={}, Y={}", xpos, ypos);
                    }
                    WindowEvent::MouseButton(button, action, _) => {
                        println!("Trigger Mouse button: {:?}, Action: {:?}", button, action);
                    }
                    _ => (),
                };
            }

            self.window.as_mut().unwrap().swap_buffers();
        }

        info!("app is going to close...");
    }
}
