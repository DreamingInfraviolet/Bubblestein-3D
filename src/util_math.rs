use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::Div;
use na;
use std::cmp::Ord;

#[derive(Copy, Clone)]
pub struct Size {
    pub w : u32,
    pub h : u32,
}

impl Add for Size {
    type Output = Size;
    fn add(self, other: Size) -> Size {
        Size { w: self.w + other.w, h: self.h + other.h }
    }
}

impl Mul for Size {
    type Output = Size;
    fn mul(self, other: Size) -> Size {
        Size { w: self.w * other.w, h: self.h * other.h }
    }
}

impl Sub for Size {
    type Output = Size;
    fn sub(self, other: Size) -> Size {
        Size { w: self.w - other.w, h: self.h - other.h }
    }
}

impl Div for Size {
    type Output = Size;
    fn div(self, other: Size) -> Size {
        Size { w: self.w / other.w, h: self.h / other.h }
    }
}

pub fn lerp(v1 : na::Vector2<f64>, v2 : na::Vector2<f64>, t : f64) -> na::Vector2<f64> {
    v1 + (v2 - v1) * t
}

pub fn clamp<T>(x : T, min : T, max : T) -> T 
    where T : Ord {
    if x > max { max } else if x < min { min } else { x }
}