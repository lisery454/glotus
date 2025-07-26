use std::collections::HashMap;

use log::{error, info};

use super::{
    texture_mode::{FilteringMode, WrappingMode},
    texture2d::Texture2D,
};

pub struct TextureManager {
    textures: HashMap<String, Texture2D>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
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
                self.textures.insert(texture_name.to_string(), t);
            }
            Err(e) => {
                error!("{:}", e);
            }
        }
    }
}
