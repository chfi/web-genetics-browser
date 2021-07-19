use crate::geometry::{Point, Rect};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct View {
    pub center: f32,
    pub scale: f32,
}

impl View {
    #[rustfmt::skip]
    #[inline]
    pub fn to_scaled_matrix(&self) -> glm::Mat4 {
        let scale = 1.0 / self.scale;

        let scaling =
            glm::mat4(scale, 0.0,   0.0, 0.0,
                      0.0,   scale, 0.0, 0.0,
                      0.0,   0.0,   1.0, 1.0,
                      0.0,   0.0,   0.0, 1.0);

        let x = self.center;

        let translation =
            glm::mat4(1.0, 0.0, 0.0,   x,
                      0.0, 1.0, 0.0, 0.0,
                      0.0, 0.0, 1.0, 0.0,
                      0.0, 0.0, 0.0, 1.0);

        scaling * translation
    }

    pub fn basepair_to_screen_map(&self) -> glm::Mat4 {
        let s = self.scale;
        let x = self.center;

        #[rustfmt::skip]
        let view_scale_screen =
            glm::mat4(  s, 0.0, 0.0, x - (s * 0.5),
                      0.0,   s, 0.0, 0.0,
                      0.0, 0.0, 1.0, 0.0,
                      0.0, 0.0, 0.0, 1.0);

        view_scale_screen
    }

    pub fn screen_to_basepair_map<Dims: Into<ViewportDims>>(&self, dims: Dims) -> glm::Mat4 {
        let dims = dims.into();

        let w = dims.width;

        let s = self.scale;
        let x = self.center;

        #[rustfmt::skip]
        let view_scale_screen =
            glm::mat4(s,   0.0, 0.0,   x - (w * s * 0.5),
                      0.0, s,   0.0, 0.0,
                      0.0, 0.0, 1.0, 0.0,
                      0.0, 0.0, 0.0, 1.0);

        view_scale_screen
    }
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
