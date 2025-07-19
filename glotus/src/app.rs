use crate::app;
use crate::camera::Camera;
use crate::camera::camera_operation::CameraMovement;
use crate::entity::Entity;
use crate::log_builder;
use crate::material::Material;
use crate::material::UniformValue;
use crate::mesh;
use crate::mesh::Mesh;
use crate::mesh::vertex::Vertex;
use crate::shader;
use crate::shader::Shader;
use crate::texture;
use crate::texture::FilteringMode;
use crate::texture::Texture2D;
use crate::texture::WrappingMode;
use crate::transform::Transform;
use cgmath::Matrix4;
use cgmath::Vector2;
use cgmath::Zero;
use chrono::DateTime;
use chrono::Local;
use gl::types::*;
use glfw::SwapInterval;
use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};
use log::debug;
use log::error;
use log::info;
use log::trace;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashMap;
use std::ffi::CString;
use std::iter::Map;
use std::mem;
use std::path::Path;
use std::ptr;
use std::rc::Rc;
use std::rc::Weak;

pub struct App {
    is_running: bool,
    window: Option<Rc<RefCell<PWindow>>>,
    glfw: Option<Rc<RefCell<Glfw>>>,
    event_receiver: Option<Rc<RefCell<GlfwReceiver<(f64, WindowEvent)>>>>,
    delta_time: f32,
    last_time: DateTime<Local>,
    last_cursor_pos: Vector2<f32>,
    is_first_cursor_move: bool,

    shaders: Rc<RefCell<HashMap<String, Shader>>>,
    meshes: Rc<RefCell<HashMap<String, Mesh>>>,
    materials: Rc<RefCell<HashMap<String, Material>>>,
    textures: Rc<RefCell<HashMap<String, Texture2D>>>,
    entities: Rc<RefCell<HashMap<String, Entity>>>,

    camera: Rc<RefCell<Camera>>,
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
            last_cursor_pos: Vector2 { x: 0.0, y: 0.0 },
            is_first_cursor_move: true,

            shaders: Rc::new(RefCell::new(HashMap::new())),
            meshes: Rc::new(RefCell::new(HashMap::new())),
            materials: Rc::new(RefCell::new(HashMap::new())),
            textures: Rc::new(RefCell::new(HashMap::new())),
            entities: Rc::new(RefCell::new(HashMap::new())),

            camera: Rc::new(RefCell::new(Camera::new(Transform::new()))),
        };

        log_builder::setup_logger();

        app
    }

    pub fn init_window(&mut self, width: u32, height: u32) {
        // 初始化 GLFW
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        // 设置窗口提示
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::Samples(Some(4)));

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
        // window.set_all_polling(true);
        window.set_key_polling(true); // 监听键盘输入
        window.set_scroll_polling(true); // 监听滚轮事件
        window.set_cursor_pos_polling(true); // 监听鼠标移动事件
        window.set_framebuffer_size_polling(true); // 监听窗口大小变化
        window.set_close_polling(true);

        // 开启垂直同步
        glfw.set_swap_interval(SwapInterval::Sync(1));
        // 不限制帧率
        // glfw.set_swap_interval(SwapInterval::None);

        // 加载 OpenGL 函数指针
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        // 显示版本
        unsafe {
            let version = gl::GetString(gl::VERSION);
            let version_str = std::ffi::CStr::from_ptr(version as *const _)
                .to_str()
                .unwrap_or("无法获取 OpenGL 版本");

            info!("OpenGL 版本: {}", version_str);
            let version = gl::GetString(gl::SHADING_LANGUAGE_VERSION);
            let version_str = std::ffi::CStr::from_ptr(version as *const _)
                .to_str()
                .unwrap_or("无法获取可支持的glsl 版本");
            info!("支持的 GLSL 版本: {:?}", version_str);
        }

        // 启用原始鼠标输入（避免系统加速影响）
        window.set_cursor_mode(glfw::CursorMode::Disabled);
        window.set_cursor_pos(width as f64 / 2.0, height as f64 / 2.0); // 初始居中

        // 初始化成员
        self.glfw = Some(Rc::new(RefCell::new(glfw)));
        self.window = Some(Rc::new(RefCell::new(window)));
        self.event_receiver = Some(Rc::new(RefCell::new(events)));

        // 抗锯齿
        unsafe {
            gl::Enable(gl::MULTISAMPLE);
        }

        // 初始化视口
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
        self.camera.borrow_mut().set_aspect_ratio(width, height);

        // 窗口大小改变时，视口变化
        let camera_weak = Rc::downgrade(&self.camera);
        self.window
            .as_ref()
            .unwrap()
            .borrow_mut()
            .set_framebuffer_size_callback(move |window, width, height| {
                debug!(
                    "window size change: width {:?}, height: {:?}",
                    width, height
                );
                unsafe {
                    gl::Viewport(0, 0, width, height);
                }
                camera_weak
                    .upgrade()
                    .unwrap()
                    .borrow_mut()
                    .set_aspect_ratio(width as u32, height as u32);
            });
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
                self.shaders.borrow_mut().insert(shader_name.to_string(), s);
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
                self.shaders.borrow_mut().insert(shader_name.to_string(), s);
            }
            Err(e) => {
                error!("{:}", e);
            }
        }
    }

    pub fn create_material(
        &mut self,
        material_name: &str,
        shader_name: &str,
        uniforms: HashMap<String, UniformValue>,
        textures: HashMap<String, u32>,
    ) {
        if !self.shaders.borrow().contains_key(shader_name) {
            error!(
                "fail add material <{:?}>, because shader <{:?}> not exists",
                material_name, shader_name
            );
        } else {
            let material = Material::new(shader_name, uniforms, textures);
            info!("success add shader <{:?}>", material_name);
            self.materials
                .borrow_mut()
                .insert(material_name.to_string(), material);
        }
    }

    pub fn create_mesh_from_data(
        &mut self,
        mesh_name: &str,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) {
        let mesh = Mesh::new(vertices, indices);

        info!("success add mesh <{:?}>", mesh_name);
        self.meshes.borrow_mut().insert(mesh_name.to_string(), mesh);
    }

    pub fn create_texture(
        &mut self,
        texture_name: &str,
        path: &str,
        wrapping_mode_s: WrappingMode,
        wrapping_mode_t: WrappingMode,
        filtering_mode_min: FilteringMode,
        filtering_mode_mag: FilteringMode,
    ) {
        match Texture2D::from_file(
            path,
            wrapping_mode_s,
            wrapping_mode_t,
            filtering_mode_min,
            filtering_mode_mag,
        ) {
            Ok(t) => {
                info!("success add texture <{:?}>", texture_name);
                self.textures
                    .borrow_mut()
                    .insert(texture_name.to_string(), t);
            }
            Err(e) => {
                error!("{:}", e);
            }
        }
    }

    pub fn create_entity(
        &mut self,
        entity_name: &str,
        transform: Transform,
        material_name: &str,
        mesh_name: &str,
    ) {
        if !self.materials.borrow().contains_key(material_name) {
            error!(
                "fail add entity <{:?}>, because material <{:?}> not exists",
                entity_name, material_name
            );
        } else if !self.meshes.borrow().contains_key(mesh_name) {
            error!(
                "fail add entity <{:?}>, because mesh <{:?}> not exists",
                entity_name, mesh_name
            );
        } else {
            let entity = Entity::new(transform, material_name, mesh_name);
            info!("success add entity <{:?}>", entity_name);
            self.entities
                .borrow_mut()
                .insert(entity_name.to_string(), entity);
        }
    }

    pub fn set_camera_transform(&mut self, transform: Transform) {
        self.camera.borrow_mut().set_transform(transform);
    }

    pub fn run(&mut self) {
        self.is_running = true;

        info!("app starts to running...");

        while self.is_running {
            self.calc_delta_time();

            self.glfw.as_ref().unwrap().borrow_mut().poll_events();
            self.handle_window_event();

            self.render();

            self.window.as_ref().unwrap().borrow_mut().swap_buffers();
        }

        info!("app is going to close...");
    }

    fn calc_delta_time(&mut self) {
        let current_time = Local::now();
        self.delta_time = (current_time - self.last_time).abs().as_seconds_f32();
        self.last_time = current_time;
        debug!("fps: {:?}", 1.0 / self.delta_time);
    }

    fn get_material_by_name(&self, name: &String) -> RefMut<Material> {
        let materials = self.materials.borrow_mut();
        let material = RefMut::map(materials, |m| m.get_mut(name).unwrap());
        material
    }

    fn get_mesh_by_name(&self, name: &String) -> RefMut<Mesh> {
        let meshes = self.meshes.borrow_mut();
        let mesh = RefMut::map(meshes, |m| m.get_mut(name).unwrap());
        mesh
    }

    fn get_shader_by_name(&self, name: &String) -> RefMut<Shader> {
        let shaders = self.shaders.borrow_mut();
        let shader = RefMut::map(shaders, |m| m.get_mut(name).unwrap());
        shader
    }

    fn get_texture_by_name(&self, name: &String) -> RefMut<Texture2D> {
        let textures = self.textures.borrow_mut();
        let texture = RefMut::map(textures, |m| m.get_mut(name).unwrap());
        texture
    }

    fn render(&mut self) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // 每帧清除深度缓冲

            let view_matrix = self.camera.borrow().get_view_matrix();
            let projection_matrix = self.camera.borrow().get_projection_matrix();

            for (entity_name, entity) in self.entities.borrow().iter() {
                let model_matrix = entity.transform.to_matrix();

                // 给材质注入全局变量，比如mvp
                let mut material = self.get_material_by_name(&entity.material_name);
                material.insert_uniform("model_matrix", UniformValue::Matrix4(model_matrix));
                material.insert_uniform("view_matrix", UniformValue::Matrix4(view_matrix));
                material.insert_uniform(
                    "projection_matrix",
                    UniformValue::Matrix4(projection_matrix),
                );

                // 通知opengl用这个材质，初始化
                self.bind_material(&entity.material_name, &material);

                // 通知opengl进行绘制
                self.get_mesh_by_name(&entity.mesh_name).draw();

                // 通知opengl卸载这个材质
                self.unbind_material(&material);
            }
        }
    }

    fn bind_material(&self, material_name: &String, material: &RefMut<Material>) {
        // 使用这个材质对应的shader
        let shader = self.get_shader_by_name(&material.shader_name);
        shader.bind();

        // 给shader设置所有这个材质对应的uniforms
        for (name, value) in &material.uniforms {
            match value {
                UniformValue::Float(v) => shader.set_uniform_f32(name, *v),
                UniformValue::Int(v) => shader.set_uniform_i32(name, *v),
                UniformValue::Vector3(v) => shader.set_uniform_vec3(name, v),
                UniformValue::Vector4(v) => shader.set_uniform_vec4(name, v),
                UniformValue::Matrix4(m) => shader.set_uniform_mat4(name, m),
                UniformValue::Texture(slot) => shader.set_uniform_i32(name, *slot as i32),
            }
        }

        for (texture_name, texture_slot_id) in &material.textures {
            if !self.textures.borrow().contains_key(texture_name) {
                error!(
                    "fail use material <{:?}>, because texture <{:?}> not exists",
                    material_name, texture_name
                );
            } else {
                unsafe {
                    let texture = self.get_texture_by_name(texture_name);
                    gl::ActiveTexture(gl::TEXTURE0 + texture_slot_id);
                    gl::BindTexture(gl::TEXTURE_2D, texture.get_id());
                }
            }
        }
    }

    pub fn unbind_material(&self, material: &RefMut<Material>) {
        let shader = self.get_shader_by_name(&material.shader_name);
        shader.unbind();
    }

    fn handle_window_event(&mut self) {
        for (_, event) in
            glfw::flush_messages(&mut *(self.event_receiver.as_ref().unwrap().borrow_mut()))
        {
            match event {
                WindowEvent::Key(key, _, action, _) => {
                    debug!("Trigger Key {:?} {:?}", key, action);
                    let velocity = 40.0;
                    match key {
                        Key::W => self
                            .camera
                            .clone()
                            .borrow_mut()
                            .process_move(CameraMovement::Forward { velocity }, self.delta_time),
                        Key::A => self
                            .camera
                            .clone()
                            .borrow_mut()
                            .process_move(CameraMovement::Left { velocity }, self.delta_time),
                        Key::S => self
                            .camera
                            .clone()
                            .borrow_mut()
                            .process_move(CameraMovement::Backward { velocity }, self.delta_time),
                        Key::D => self
                            .camera
                            .clone()
                            .borrow_mut()
                            .process_move(CameraMovement::Right { velocity }, self.delta_time),
                        Key::LeftShift => self
                            .camera
                            .clone()
                            .borrow_mut()
                            .process_move(CameraMovement::Down { velocity }, self.delta_time),
                        Key::Space => self
                            .camera
                            .clone()
                            .borrow_mut()
                            .process_move(CameraMovement::Up { velocity }, self.delta_time),
                        Key::Escape => {
                            debug!("Trigger WindowClose");
                            self.window
                                .as_ref()
                                .unwrap()
                                .borrow_mut()
                                .set_should_close(true);
                            self.is_running = false;
                        }
                        _ => {}
                    }
                }
                WindowEvent::Close => {
                    debug!("Trigger WindowClose");
                    self.window
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .set_should_close(true);
                    self.is_running = false;
                }
                WindowEvent::Scroll(xoffset, yoffset) => {
                    debug!("Trigger Mouse Scroll: X={}, Y={}", xoffset, yoffset);
                    self.camera
                        .clone()
                        .borrow_mut()
                        .process_zoom(yoffset as f32, 0.5);
                }
                WindowEvent::CursorPos(xpos, ypos) => {
                    debug!("Trigger Cursor Move: X={}, Y={}", xpos, ypos);
                    if self.is_first_cursor_move {
                        self.is_first_cursor_move = false;
                        self.last_cursor_pos = Vector2::new(xpos as f32, ypos as f32);
                    } else {
                        let xoffset = xpos as f32 - self.last_cursor_pos.x;
                        let yoffset = ypos as f32 - self.last_cursor_pos.y;
                        self.camera
                            .clone()
                            .borrow_mut()
                            .process_turn(xoffset, yoffset, 0.001, true);
                        self.last_cursor_pos = Vector2::new(xpos as f32, ypos as f32);
                    }
                }
                WindowEvent::MouseButton(button, action, _) => {
                    debug!("Trigger Mouse button: {:?}, Action: {:?}", button, action);
                }
                _ => (),
            };
        }
    }
}
