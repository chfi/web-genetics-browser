use crate::geometry::{Point, Rect};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct View {
    pub center: f32,
    pub scale: f32,
}

impl Default for View {
    #[inline]
    fn default() -> Self {
        Self {
            center: 0.0,
            scale: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ViewportDims {
    pub width: f32,
    pub height: f32,
}

impl From<Point> for ViewportDims {
    #[inline]
    fn from(point: Point) -> Self {
        Self {
            width: point.x,
            height: point.y,
        }
    }
}

impl Into<Point> for ViewportDims {
    #[inline]
    fn into(self) -> Point {
        Point {
            x: self.width,
            y: self.height,
        }
    }
}

impl From<(f32, f32)> for ViewportDims {
    #[inline]
    fn from((width, height): (f32, f32)) -> Self {
        Self { width, height }
    }
}

impl From<[f32; 2]> for ViewportDims {
    #[inline]
    fn from(dims: [f32; 2]) -> Self {
        Self {
            width: dims[0],
            height: dims[1],
        }
    }
}

impl From<[u32; 2]> for ViewportDims {
    #[inline]
    fn from(dims: [u32; 2]) -> Self {
        Self {
            width: dims[0] as f32,
            height: dims[1] as f32,
        }
    }
}
