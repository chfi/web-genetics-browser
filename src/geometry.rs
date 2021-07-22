use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::{borrow::Cow, mem};

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self { position: [x, y] }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rect {
    min: Point,
    max: Point,
}

impl Point {
    pub const ZERO: Self = Point { x: 0.0, y: 0.0 };

    #[inline]
    pub fn new<T: Into<f32>>(x: T, y: T) -> Self {
        let x = x.into();
        let y = y.into();
        Self { x, y }
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.x.hypot(self.y)
    }

    pub fn toward(&self, other: Point) -> Point {
        let diff = *self - other;
        diff / diff.length()
    }

    #[inline]
    pub fn dist(&self, other: Point) -> f32 {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        x_diff.hypot(y_diff)
    }

    #[inline]
    pub fn dist_sqr(&self, other: Point) -> f32 {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        x_diff.powi(2) + y_diff.powi(2)
    }

    #[inline]
    pub fn vertex(&self) -> Vertex {
        Vertex {
            position: [self.x, self.y],
        }
    }
}

impl From<(f32, f32)> for Point {
    #[inline]
    fn from((x, y): (f32, f32)) -> Point {
        Point { x, y }
    }
}

impl From<(f64, f64)> for Point {
    #[inline]
    fn from((x, y): (f64, f64)) -> Point {
        let x = x as f32;
        let y = y as f32;
        Point { x, y }
    }
}

impl From<(i32, i32)> for Point {
    #[inline]
    fn from((x, y): (i32, i32)) -> Point {
        let x = x as f32;
        let y = y as f32;
        Point { x, y }
    }
}

impl Into<egui::Pos2> for Point {
    #[inline]
    fn into(self) -> egui::Pos2 {
        egui::Pos2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<egui::Vec2> for Point {
    #[inline]
    fn into(self) -> egui::Vec2 {
        egui::Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Rect {
    #[inline]
    pub fn new<T: Into<Point>>(p0: T, p1: T) -> Self {
        let p0 = p0.into();
        let p1 = p1.into();

        let min = Point {
            x: p0.x.min(p1.x),
            y: p0.y.min(p1.y),
        };

        let max = Point {
            x: p0.x.max(p1.x),
            y: p0.y.max(p1.y),
        };

        Self { min, max }
    }
    #[inline]
    pub fn everywhere() -> Self {
        let min = Point {
            x: std::f32::MIN,
            y: std::f32::MIN,
        };

        let max = Point {
            x: std::f32::MAX,
            y: std::f32::MAX,
        };

        Self { min, max }
    }

    #[inline]
    pub fn nowhere() -> Self {
        let min = Point {
            x: std::f32::MAX,
            y: std::f32::MAX,
        };

        let max = Point {
            x: std::f32::MIN,
            y: std::f32::MIN,
        };

        Self { min, max }
    }

    #[inline]
    pub fn min(&self) -> Point {
        self.min
    }

    #[inline]
    pub fn max(&self) -> Point {
        self.max
    }

    #[inline]
    pub fn contains(&self, p: Point) -> bool {
        self.min.x <= p.x && self.max.x >= p.x && self.min.y <= p.y && self.max.y >= p.y
    }

    #[inline]
    pub fn union(&self, other: Self) -> Self {
        let min = Point {
            x: self.min.x.min(other.min.x),
            y: self.min.y.min(other.min.y),
        };

        let max = Point {
            x: self.max.x.max(other.max.x),
            y: self.max.y.max(other.max.y),
        };

        Self { min, max }
    }
}

impl From<(Point, Point)> for Rect {
    #[inline]
    fn from((p0, p1): (Point, Point)) -> Self {
        Self::new(p0, p1)
    }
}

impl Into<egui::Rect> for Rect {
    #[inline]
    fn into(self) -> egui::Rect {
        egui::Rect::from_min_max(self.min.into(), self.max.into())
    }
}

macro_rules! impl_assign_binop {
    ($trait:ident, Rhs = $rhs:ty, $opfn:ident, $opassfn:ident) => {
        impl $trait<$rhs> for Point {
            #[inline]
            fn $opassfn(&mut self, other: $rhs) {
                *self = self.$opfn(other);
            }
        }
    };
    ($trait:ident, $opfn:ident, $opassfn:ident) => {
        impl_assign_binop!($trait, Rhs = Point, $opfn, $opassfn);
    };
}

macro_rules! impl_ref_binop {
    ($trait:ident, $rhs:ty, $opfn:ident) => {
        impl $trait<$rhs> for Point {
            type Output = Self;
            #[inline]
            fn $opfn(self, other: $rhs) -> Self {
                self.$opfn(*other)
            }
        }
    };
}

macro_rules! impl_ref_assign_binop {
    ($trait:ident, $rhs:ty, $opfn:ident) => {
        impl $trait<$rhs> for Point {
            #[inline]
            fn $opfn(&mut self, other: $rhs) {
                self.$opfn(*other)
            }
        }
    };
}

macro_rules! impl_point_ops {
    ($trait:ident, $traitass:ident, $opfn:ident, $opassfn:ident) => {
        impl $trait for Point {
            type Output = Self;

            #[inline]
            fn $opfn(self, other: Self) -> Self {
                Self {
                    x: f32::$opfn(self.x, other.x),
                    y: f32::$opfn(self.y, other.y),
                }
            }
        }

        impl_assign_binop!($traitass, $opfn, $opassfn);
        impl_ref_binop!($trait, &Point, $opfn);
        impl_ref_assign_binop!($traitass, &Point, $opassfn);
    };
}

impl_point_ops!(Add, AddAssign, add, add_assign);
impl_point_ops!(Sub, SubAssign, sub, sub_assign);

impl Mul<f32> for Point {
    type Output = Self;

    #[inline]
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl_assign_binop!(MulAssign, Rhs = f32, mul, mul_assign);
impl_ref_binop!(Mul, &f32, mul);
impl_ref_assign_binop!(MulAssign, &f32, mul_assign);

impl Div<f32> for Point {
    type Output = Self;

    #[inline]
    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl_assign_binop!(DivAssign, Rhs = f32, div, div_assign);
impl_ref_binop!(Div, &f32, div);
impl_ref_assign_binop!(DivAssign, &f32, div_assign);

pub fn example_vertices() -> Vec<Vertex> {
    let mut data: Vec<Vertex> = Vec::new();

    let del = 0.03;

    for i in 0..50 {
        let x = ((i as f32) / 25.0) - 1.0;
        let y = (x * 2.0).sin();

        let top = Vertex::new(x, y);
        let right = Vertex::new(x + del, y - del);

        data.push(top);
        data.push(top);
        data.push(top);
    }

    data
}
