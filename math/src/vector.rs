extern crate num;
use self::num::traits::{ One, Zero };

#[derive(Debug, Default)]
pub struct Vector4<T> {
	pub x: T,
	pub y: T,
	pub z: T,
	pub w: T
}

impl<T> Vector4<T> 
	where T: One + Zero {
		
	pub fn position(x: T, y: T, z: T) -> Vector4<T> {
		Vector4 { x, y, z, w: T::one() }
	}
	
	pub fn direction(x: T, y: T, z: T) -> Vector4<T> {
		Vector4 { x, y, z, w: T::zero() }
	}
}

pub type Vector4f = Vector4<f32>;
pub type Vector4d = Vector4<f64>;
pub type Vector4i = Vector4<i32>;
pub type Vector4l = Vector4<i64>;
