use crate::camera::Camera;
use crate::camera::CameraMovement;
use crate::entity::entity_manager::EntityManager;
use crate::light::Light;
use crate::log_builder;
use crate::material::Material;
use crate::material::MaterialManager;
use crate::material::UniformValue;
use crate::mesh::MeshManager;
use crate::mesh::Vertex;
use crate::shader::ShaderManager;
use crate::texture::{FilteringMode, TextureManager, WrappingMode};
use crate::transform::Transform;
use cgmath::Vector2;
use glfw::SwapInterval;
use glfw::ffi::glfwGetTime;
use glfw::{Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};
use log::debug;
use log::error;
use log::info;
use log::warn;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct App {
    is_running: bool,
    window: Option<Rc<RefCell<PWindow>>>,
    glfw: Option<Rc<RefCell<Glfw>>>,
    event_receiver: Option<Rc<RefCell<GlfwReceiver<(f64, WindowEvent)>>>>,
    delta_time: f32,
    last_time: f32,
    last_cursor_pos: Vector2<f32>,
    is_first_cursor_move: bool,

    shader_manager: Rc<RefCell<ShaderManager>>,
    material_manager: Rc<RefCell<MaterialManager>>,
    mesh_manager: Rc<RefCell<MeshManager>>,
    texture_manager: Rc<RefCell<TextureManager>>,
    entity_manager: Rc<RefCell<EntityManager>>,

    light: Rc<RefCell<Light>>,
    camera: Rc<RefCell<Camera>>,
}

// shader
impl App {
    pub fn create_shader_from_source(
        &self,
        shader_name: &str,
        vertex_source: &str,
        fragment_source: &str,
    ) {
        self.shader_manager.borrow_mut().create_shader_from_source(
            shader_name,
            vertex_source,
            fragment_source,
        );
    }

    pub fn create_shader_from_file(
        &self,
        shader_name: &str,
        vertex_path: &str,
        fragment_path: &str,
    ) {
        self.shader_manager.borrow_mut().create_shader_from_file(
            shader_name,
            vertex_path,
            fragment_path,
        );
    }
}

// material
impl App {
    pub fn create_material(
        &self,
        material_name: &str,
        shader_name: &str,
        uniforms: HashMap<String, UniformValue>,
        textures: HashMap<String, u32>,
    ) {
        self.material_manager.borrow_mut().create_material(
            material_name,
            shader_name,
            uniforms,
            textures,
        );
    }
}

// mesh
impl App {
    pub fn create_mesh_from_data(&self, mesh_name: &str, vertices: Vec<Vertex>, indices: Vec<u32>) {
        self.mesh_manager
            .borrow_mut()
            .create_mesh_from_data(mesh_name, vertices, indices);
    }
}

// texture
impl App {
    pub fn create_texture(
        &mut self,
        texture_name: &str,
        path: &str,
        wrapping_mode_s: WrappingMode,
        wrapping_mode_t: WrappingMode,
        filtering_mode_min: FilteringMode,
        filtering_mode_mag: FilteringMode,
    ) {
        self.texture_manager.borrow_mut().create_texture(
            texture_name,
            path,
            wrapping_mode_s,
            wrapping_mode_t,
            filtering_mode_min,
            filtering_mode_mag,
        );
    }
}

// entity
impl App {
    pub fn create_entity(
        &self,
        entity_name: &str,
        transform: Transform,
        material_name: &str,
        mesh_name: &str,
    ) {
        self.entity_manager.borrow_mut().create_entity(
            entity_name,
            transform,
            material_name,
            mesh_name,
        );
    }
}

// camera
impl App {
    pub fn set_camera_transform(&mut self, transform: Transform) {
        self.camera.borrow_mut().set_transform(transform);
    }
}

// light
impl App {
    pub fn set_light_transform(&mut self, transform: Transform) {
        self.light.borrow_mut().set_transform(transform);
    }

    pub fn set_light_color(&mut self, color: [f32; 4]) {
        self.light.borrow_mut().set_color(color);
    }
}

// main
impl App {
    pub fn new() -> Self {
        let shader_manager = Rc::new(RefCell::new(ShaderManager::new()));
        let material_manager = Rc::new(RefCell::new(MaterialManager::new(Rc::downgrade(
            &shader_manager,
        ))));

        let mesh_manager = Rc::new(RefCell::new(MeshManager::new()));
        let texture_manager = Rc::new(RefCell::new(TextureManager::new()));
        let entity_manager = Rc::new(RefCell::new(EntityManager::new(
            Rc::downgrade(&mesh_manager),
            Rc::downgrade(&material_manager),
        )));

        let app = Self {
            is_running: false,
            window: None,
            glfw: None,
            event_receiver: None,
            delta_time: 0.0,
            last_time: 0.0,
            last_cursor_pos: Vector2 { x: 0.0, y: 0.0 },
            is_first_cursor_move: true,

            shader_manager,
            material_manager,
            mesh_manager,
            texture_manager,
            entity_manager,

            light: Rc::new(RefCell::new(Light::new())),
            camera: Rc::new(RefCell::new(Camera::new())),
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
            .set_framebuffer_size_callback(move |_window, width, height| {
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
}

// utils
impl App {
    fn calc_delta_time(&mut self) {
        unsafe {
            let current_time = glfwGetTime() as f32;
            self.delta_time = (current_time - self.last_time).abs();
            self.last_time = current_time;
        }
        debug!("fps: {:?}", 1.0 / self.delta_time);
    }

    fn render(&mut self) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // 每帧清除深度缓冲

            let view_matrix = self.camera.borrow().get_view_matrix();
            let projection_matrix = self.camera.borrow().get_projection_matrix();
            let light_color = self.light.borrow().get_color();
            let light_position = self.light.borrow().get_transform().get_position().get_arr();
            let view_position = self
                .camera
                .borrow()
                .get_transform()
                .get_position()
                .get_arr();

            for (_entity_name, entity) in self.entity_manager.borrow().iter_entities() {
                let model_matrix = entity.transform.to_matrix();
                let normal_matrix = entity.transform.to_normal_matrix();

                // 给材质注入全局变量，比如mvp
                let mut material_manager = self.material_manager.borrow_mut();
                let material = material_manager.get_mut(&entity.material_name);
                match material {
                    Some(material) => {
                        material.insert_uniform("light_color", UniformValue::Vector4(light_color));
                        material.insert_uniform(
                            "light_position",
                            UniformValue::Vector3(light_position),
                        );
                        material
                            .insert_uniform("view_position", UniformValue::Vector3(view_position));
                        material
                            .insert_uniform("model_matrix", UniformValue::Matrix4(model_matrix));

                        material
                            .insert_uniform("normal_matrix", UniformValue::Matrix3(normal_matrix));
                        material.insert_uniform("view_matrix", UniformValue::Matrix4(view_matrix));
                        material.insert_uniform(
                            "projection_matrix",
                            UniformValue::Matrix4(projection_matrix),
                        );

                        // 通知opengl用这个材质，初始化
                        self.bind_material(&entity.material_name, material);

                        // 通知opengl进行绘制
                        let mesh_manager = self.mesh_manager.borrow();
                        let mesh = mesh_manager.get(&entity.mesh_name);
                        match mesh {
                            Some(mesh) => mesh.draw(),
                            None => warn!("not found mesh: <{}>", &entity.mesh_name),
                        }

                        // 通知opengl卸载这个材质
                        self.unbind_material(&material);
                    }
                    None => warn!("not found material: <{}>", &entity.material_name),
                }
            }
        }
    }

    fn bind_material(&self, material_name: &String, material: &mut Material) {
        let shader_manager = self.shader_manager.borrow();
        let shader = shader_manager.get(material.shader_name.as_str());
        match shader {
            Some(shader) => {
                shader.bind();

                // 给shader设置所有这个材质对应的uniforms
                for (name, value) in &material.uniforms {
                    match value {
                        UniformValue::Float(v) => shader.set_uniform_f32(name, *v),
                        UniformValue::Int(v) => shader.set_uniform_i32(name, *v),
                        UniformValue::Vector3(v) => shader.set_uniform_vec3(name, v),
                        UniformValue::Vector4(v) => shader.set_uniform_vec4(name, v),
                        UniformValue::Matrix3(m) => shader.set_uniform_mat3(name, m),
                        UniformValue::Matrix4(m) => shader.set_uniform_mat4(name, m),
                        UniformValue::Texture(slot) => shader.set_uniform_i32(name, *slot as i32),
                    }
                }

                for (texture_name, texture_slot_id) in &material.textures {
                    let texture_manager = self.texture_manager.borrow();
                    let texture = texture_manager.get(&texture_name);

                    match texture {
                        Some(texture) => unsafe {
                            gl::ActiveTexture(gl::TEXTURE0 + texture_slot_id);
                            gl::BindTexture(gl::TEXTURE_2D, texture.get_id());
                        },
                        None => error!(
                            "fail use material <{:?}>, because texture <{:?}> not exists",
                            material_name, texture_name
                        ),
                    }
                }
            }
            None => error!("not find shader: <{}>", material.shader_name.as_str()),
        }
    }

    fn unbind_material(&self, material: &Material) {
        let shader_manager = self.shader_manager.borrow();
        let shader = shader_manager.get(material.shader_name.as_str());
        match shader {
            Some(shader) => shader.unbind(),
            None => error!("not find shader: <{}>", material.shader_name.as_str()),
        }
    }

    fn handle_window_event(&mut self) {
        for (_, event) in
            glfw::flush_messages(&mut *(self.event_receiver.as_ref().unwrap().borrow_mut()))
        {
            match event {
                WindowEvent::Key(key, _, action, _) => {
                    debug!("Trigger Key {:?} {:?}", key, action);
                    if key == Key::Escape {
                        debug!("Trigger WindowClose");
                        self.window
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .set_should_close(true);
                        self.is_running = false;
                    }

                    let velocity = 40.0;
                    let movement = match key {
                        Key::W => Some(CameraMovement::Forward),
                        Key::A => Some(CameraMovement::Left),
                        Key::S => Some(CameraMovement::Backward),
                        Key::D => Some(CameraMovement::Right),
                        Key::LeftShift => Some(CameraMovement::Down),
                        Key::Space => Some(CameraMovement::Up),
                        _ => None,
                    };
                    if let Some(movement) = movement {
                        self.camera
                            .borrow_mut()
                            .process_move(movement, velocity, self.delta_time);
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
