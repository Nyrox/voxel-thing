extern crate gl;
extern crate stb_image;

use self::stb_image::image::{LoadResult, Image};
use std::path::PathBuf;
use std::fmt;
use std::fmt::{Debug};

use gl::types::*;

pub struct Texture2D {	
	pub image: Image<u8>,
	pub format: GLenum,
	pub handle: GLuint
}

impl Debug for Texture2D {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture2D {{ format: {}, handle: {} }}", self.format, self.handle)
    }
}

impl Texture2D {
	pub fn new(path: PathBuf, format: GLenum) -> Texture2D {
		let image = stb_image::image::load(path);
		let image = match image {
			LoadResult::ImageU8(data) => data,
			LoadResult::ImageF32(..) => panic!("Found floating point texture, use Texture2D_HDR instead."),
			LoadResult::Error(string) => panic!(string)
		};
		
		let mut handle = 0;
		unsafe { gl::CreateTextures(gl::TEXTURE_2D, 1, &mut handle) };
		
		let obj = Texture2D { image, format, handle };
		obj.allocate();
		let pixel_formats = [0, gl::RED, gl::RG, gl::RGB, gl::RGBA];		
		
		unsafe {
			gl::TextureSubImage2D(obj.handle, 0, 0, 0, obj.image.width as i32, obj.image.height as i32, pixel_formats[obj.image.depth], gl::UNSIGNED_BYTE, obj.image.data.as_ptr() as *const GLvoid);
			gl::TextureParameteri(handle, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
			gl::TextureParameteri(handle, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
			gl::TextureParameteri(handle, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
			gl::TextureParameteri(handle, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
			gl::GenerateTextureMipmap(obj.handle);
		}
		obj
	}
	
	fn allocate(&self) {
		unsafe {
			gl::TextureStorage2D(self.handle, self.get_mipmap_levels(), self.format, self.image.width as i32, self.image.height as i32);
		}
	}
	
	fn get_mipmap_levels(&self) -> i32 {
		1 + (self.image.width.max(self.image.height) as f32).log2().floor() as i32;
		1
	}
	
	pub fn bind(&self, texture_unit: u32) {
		unsafe {
			gl::ActiveTexture(gl::TEXTURE0 + texture_unit);
			gl::BindTexture(gl::TEXTURE_2D, self.handle);
		}
	}
}