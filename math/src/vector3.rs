extern crate num;
use self::num::traits::{One, Zero, Float, NumCast, Num};
use std::ops::{Add, Mul, Div, Sub, Index, IndexMut};

#[derive(Debug, Default, Clone, Copy)]
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
	
	pub fn up() -> Vector3<T> {
		Vector3::new(T::zero(), T::one(), T::zero())
	}
}

impl<T> Vector3<T> where T: Float {
	pub fn normalize(&mut self) -> Vector3<T> {
		let len = self.len();
		Vector3::new(self.x / len, self.y / len, self.z / len)
	}
	
	pub fn len(&self) -> T {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}
	
	pub fn cross(lhs: Vector3<T>, rhs: Vector3<T>) -> Vector3<T>  {
		Vector3::new(
			lhs.y * rhs.z - lhs.z * rhs.y,
			lhs.z * rhs.x - lhs.x * rhs.z,
			lhs.x * rhs.y - lhs.y * rhs.x
		)
	}
	
	pub fn dot(&self, rhs: Vector3<T>) -> T {
		self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
	}
}

impl<T> Sub for Vector3<T> where T: Num {
	type Output = Vector3<T>;
	
	fn sub(self, rhs: Vector3<T>) -> Vector3<T> {
		Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
	}
}