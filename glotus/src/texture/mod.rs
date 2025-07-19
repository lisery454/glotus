use cgmath::Matrix;
use gl::types::*;
use log::{error, warn};
use std::{ffi::CString, fs, path::Path, ptr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TextureError {
    #[error("Failed to read texture file: {0}")]
    FileReadError(String),
}

#[derive(Debug)]
pub enum WrappingMode {
    Repeat,
    MirroreroredRepeat,
    ClampToEdge,
    ClampToBorder { color: [f32; 4] },
}

#[derive(Debug)]
pub enum FilteringMode {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

pub struct Texture2D {
    id: GLuint,
}

impl Texture2D {
    pub fn get_id(&self) -> GLuint {
        self.id
    }
    pub fn from_file(
        path: &str,
        wrapping_mode_s: WrappingMode,
        wrapping_mode_t: WrappingMode,
        filtering_mode_min: FilteringMode,
        filtering_mode_mag: FilteringMode,
    ) -> Result<Self, TextureError> {
        // 1. 用 `image` 库读取图片
        let img_result = image::open(path);

        if img_result.is_err() {
            return Err(TextureError::FileReadError(path.to_string()));
        }

        let img = img_result.unwrap().flipv(); // OpenGL 的纹理坐标原点在左下，需要翻转Y轴

        // 2. 转换为 RGBA 格式
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();

        // 3. 生成 OpenGL 纹理
        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // 设置纹理参数
            Texture2D::set_wrapping_mode(gl::TEXTURE_WRAP_S, wrapping_mode_s);
            Texture2D::set_wrapping_mode(gl::TEXTURE_WRAP_T, wrapping_mode_t);

            Texture2D::set_filtering_mode(gl::TEXTURE_MIN_FILTER, filtering_mode_min);
            Texture2D::set_filtering_mode(gl::TEXTURE_MAG_FILTER, filtering_mode_mag);

            // 将图片数据上传到 GPU
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32, // 内部格式
                width as i32,
                height as i32,
                0,
                gl::RGBA,                  // 数据格式
                gl::UNSIGNED_BYTE,         // 数据类型
                rgba.as_ptr() as *const _, // 图片数据指针
            );

            // 生成 Mipmap（可选）
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(Self { id: texture_id })
    }

    fn set_wrapping_mode(wrap: GLenum, wrapping_mode: WrappingMode) {
        unsafe {
            match wrapping_mode {
                WrappingMode::Repeat => gl::TexParameteri(gl::TEXTURE_2D, wrap, gl::REPEAT as i32),
                WrappingMode::MirroreroredRepeat => {
                    gl::TexParameteri(gl::TEXTURE_2D, wrap, gl::MIRRORED_REPEAT as i32)
                }
                WrappingMode::ClampToEdge => {
                    gl::TexParameteri(gl::TEXTURE_2D, wrap, gl::CLAMP_TO_EDGE as i32)
                }
                WrappingMode::ClampToBorder { color } => {
                    gl::TexParameteri(gl::TEXTURE_2D, wrap, gl::CLAMP_TO_BORDER as i32);
                    gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, color.as_ptr());
                }
            }
        }
    }

    fn set_filtering_mode(filter: GLenum, filtering_mode: FilteringMode) {
        unsafe {
            match filtering_mode {
                FilteringMode::Nearest => {
                    gl::TexParameteri(gl::TEXTURE_2D, filter, gl::NEAREST as i32)
                }
                FilteringMode::Linear => {
                    gl::TexParameteri(gl::TEXTURE_2D, filter, gl::LINEAR as i32)
                }
                FilteringMode::NearestMipmapNearest => {
                    gl::TexParameteri(gl::TEXTURE_2D, filter, gl::NEAREST_MIPMAP_NEAREST as i32)
                }
                FilteringMode::LinearMipmapNearest => {
                    gl::TexParameteri(gl::TEXTURE_2D, filter, gl::LINEAR_MIPMAP_NEAREST as i32)
                }
                FilteringMode::NearestMipmapLinear => {
                    gl::TexParameteri(gl::TEXTURE_2D, filter, gl::NEAREST_MIPMAP_LINEAR as i32)
                }
                FilteringMode::LinearMipmapLinear => {
                    gl::TexParameteri(gl::TEXTURE_2D, filter, gl::LINEAR_MIPMAP_LINEAR as i32)
                }
            }
        }
    }
}
