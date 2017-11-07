
use std::marker::Copy;
use std::ops::{Add, Mul, Div, Sub, Index, IndexMut};
extern crate num;
use self::num::traits::{Num, Float, NumCast};

use vector4::Vector4;

#[derive(Debug, Default)]
pub struct Matrix4<T> {
	pub data: [[T; 4]; 4]
}

// Define some convenience types
pub type Matrix4f = Matrix4<f32>;
pub type Matrix4d = Matrix4<f64>;
pub type Matrix4i = Matrix4<i32>;
pub type Matrix4l = Matrix4<i64>;



// Impl index
impl<T> Index<usize> for Matrix4<T> {
	type Output = [T; 4];
	
	fn index(&self, index: usize) -> &[T; 4] {
		&self.data[index]
	}
}

// Impl indexMut
impl<T> IndexMut<usize> for Matrix4<T> {
	fn index_mut(&mut self, index: usize) -> &mut [T; 4] {
		&mut self.data[index]
	}
}

// Functions!
impl<T> Matrix4<T> where T: Copy + Default + Num {	
	// Creates a new translation matrix
	pub fn translate(x: T, y: T, z: T) -> Matrix4<T> where T: num::Num + num::One {
		let mut mat = Matrix4::default();
		mat[0][3] = x;
		mat[1][3] = y;
		mat[2][3] = z;
		mat[0][0] = T::one();
		mat[1][1] = T::one();
		mat[2][2] = T::one();
		mat[3][3] = T::one();
		
		return mat;
	}
	
	// Multiply the matrix by a vector
	pub fn mul_vec(&self, rhs: Vector4<T>) -> Vector4<T> where T: Mul<Output=T> + Add<Output=T> + Copy {
		Vector4 {
			x: self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z + self[0][3] * rhs.w,
			y: self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z + self[1][3] * rhs.w,
			z: self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z + self[2][3] * rhs.w,
			w: self[3][0] * rhs.x + self[3][1] * rhs.y + self[3][2] * rhs.z + self[3][3] * rhs.w
		}
	}
}

// Some operations can really only be supported if we are dealing with a floating point T
impl<T> Matrix4<T> where T: Float + NumCast + Copy + Default {
	pub fn perspective(fov: T, aspect: T, zNear: T, zFar: T) -> Matrix4<T> {
		let mut mat = Matrix4::default();
		let _2: T =  NumCast::from(2usize).unwrap();
		
		let rad = fov;
		let tanHalfFov = (rad / _2).tan();
		
		mat[0][0] = T::one() / (aspect * tanHalfFov);
		mat[1][1] = T::one() / (tanHalfFov);
		mat[2][2] = -(zFar + zNear) / (zFar - zNear);
		mat[2][3] = -T::one();
		mat[3][2] = -(_2 * zFar * zNear) / (zFar - zNear);
		
		return mat;
	}
}


