extern crate num;
use self::num::traits::{One, Zero};

#[derive(Debug, Default)]
pub struct Vector3<T> {
	pub x: T,
	pub y: T,
	pub z: T
}

pub type Vector3f = Vector3<f32>;
pub type Vector3d = Vector3<f64>;
pub type Vector3i = Vector3<i32>;
pub type Vector3l = Vector3<i64>;

impl<T> Vector3<T> where T: One + Zero {
	pub fn new(x: T, y: T, z: T) -> Vector3<T> {
		Vector3 { x, y, z }
	}
}