use core::f64;
use std::any::Any;

pub trait Unit: Any {
    fn to_pixels(&self, available: Option<f64>) -> f64;

    fn as_any(&self) -> &dyn Any;
}

impl dyn Unit {
    pub fn downcast_ref<T: Unit + 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pixels(f64);

impl Pixels {
    pub fn new(value: f64) -> Self {
        Self(value)
    }
}

impl Unit for Pixels {
    fn to_pixels(&self, _available: Option<f64>) -> f64 {
        self.0
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Percent(f64);

impl Percent {
    pub fn new(value: f64) -> Self {
        Self(value)
    }
}

impl Unit for Percent {
    fn to_pixels(&self, available: Option<f64>) -> f64 {
        available
            .map(|available| available * self.0)
            .unwrap_or(f64::INFINITY)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Into<taffy::Dimension> for Pixels {
    fn into(self) -> taffy::Dimension {
        taffy::Dimension::length(self.0 as f32)
    }
}

impl Into<taffy::Dimension> for Percent {
    fn into(self) -> taffy::Dimension {
        taffy::Dimension::percent(self.0 as f32)
    }
}
