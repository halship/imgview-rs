use gl::types::*;
use image::{DynamicImage, GenericImageView, ImageResult};
use std::path::Path;

pub struct Texture {
    id: GLuint,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn from_path<P: AsRef<Path>>(path: P) -> ImageResult<Self> {
        let img = image::open(&path)?;
        Ok(Self::from_img(img))
    }

    pub fn from_img(img: DynamicImage) -> Self {
        let (width, height) = img.dimensions();
        let data = img.flipv().to_rgba().into_raw();

        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const GLvoid,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Self { id, width, height }
    }

    pub fn bind(&self, n: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + n);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.id);
        }
    }
}
