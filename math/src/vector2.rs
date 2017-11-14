extern crate num;
use self::num::traits::{One, Zero};

#[derive(Debug, Default)]
pub struct Vector2<T> {
	pub x: T,
	pub y: T
}

pub type Vector2f = Vector2<f32>;
pub type Vector2d = Vector2<f64>;
pub type Vector2i = Vector2<i32>;
pub type Vector2l = Vector2<i64>;

impl<T> Vector2<T> where T: One + Zero {
	pub fn new(x: T, y: T) -> Vector2<T> {
		Vector2 { x, y }
	}
}