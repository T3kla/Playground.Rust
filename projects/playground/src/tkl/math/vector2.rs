use std::fmt::*;
use std::ops;

pub const LEFT: Vector2 = Vector2 { x: -1.0, y: 0.0 };
pub const RIGHT: Vector2 = Vector2 { x: 1.0, y: 0.0 };
pub const UP: Vector2 = Vector2 { x: 0.0, y: 1.0 };
pub const DOWN: Vector2 = Vector2 { x: 0.0, y: -1.0 };

#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new() -> Self {
        Vector2 { x: 0.0, y: 0.0 }
    }
    pub fn from(_x: f32, _y: f32) -> Self {
        Vector2 { x: _x, y: _y }
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from(a: (f32, f32)) -> Self {
        Vector2 { x: a.0, y: a.1 }
    }
}

// Operators
//    o Vec2 + Vec2
//    o Vec2 - Vec2
//    o Vec2 * Vec2
//    o Vec2 * f32
//    o Vec2 / f32

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, _rhs: Vector2) -> Vector2 {
        Vector2::from(self.x + _rhs.x, self.y + _rhs.y)
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, _rhs: Vector2) -> Vector2 {
        Vector2::from(self.x - _rhs.x, self.y - _rhs.y)
    }
}

impl ops::Mul<Vector2> for Vector2 {
    type Output = f32;
    fn mul(self, _rhs: Vector2) -> f32 {
        self.x * _rhs.x + self.y * _rhs.y
    }
}

impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, _rhs: f32) -> Vector2 {
        Vector2::from(self.x * _rhs, self.y * _rhs)
    }
}

impl ops::Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, _rhs: f32) -> Vector2 {
        Vector2::from(self.x / _rhs, self.y / _rhs)
    }
}

impl Vector2 {
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn nor(self) -> Vector2 {
        self / self.len()
    }
}

pub fn det(a: Vector2, b: Vector2) -> f32 {
    a.x * b.y - a.y * b.x
}

pub fn dot(a: Vector2, b: Vector2) -> f32 {
    a * b
}

pub fn ang(a: Vector2, b: Vector2) -> f32 {
    det(a, b).atan2(dot(a, b))
}

pub fn angf(a: Vector2) -> f32 {
    det(RIGHT, a).atan2(dot(RIGHT, a))
}

impl Debug for Vector2 {
    fn fmt(&self, a: &mut Formatter<'_>) -> Result {
        a.debug_struct("struct")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
