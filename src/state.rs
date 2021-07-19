use crate::geometry::{Point, Rect};
use crate::view::{View, ViewportDims};

use crossbeam::atomic::AtomicCell;
use std::sync::Arc;

pub struct SharedState {
    pub view: Arc<AtomicCell<View>>,
}
