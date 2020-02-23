use cgmath::Vector2;
use glium::glutin::dpi;

use std::ops::{Add, Mul};

/// Used to represent logical pixel coordinates and dimensions.
///
/// This struct is distinct from `PhysicalVector` which represents
/// physical pixel coordinates and dimensions to avoid
/// confusion when dealing with scaled dpi scenarios.
#[derive(Copy, Clone, Debug)]
pub struct LogicalVector {
    pub vec: Vector2<f32>,
}
impl LogicalVector {
    pub fn new(x: f32, y: f32) -> Self {
        LogicalVector { vec: Vector2::new(x, y) }
    }
}
impl Default for LogicalVector {
    fn default() -> LogicalVector {
        LogicalVector { vec: Vector2::<f32>::new(0.0, 0.0) }
    }
}
impl Add for LogicalVector {
    type Output = Self;
    fn add(self, other: LogicalVector) -> Self::Output {
        (self.vec + other.vec).into()
    }
}
impl<T: Into<f32>> Mul<T> for LogicalVector {
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        (self.vec * other.into()).into()
    }
}
impl<T: Into<f32>> From<Vector2<T>> for LogicalVector {
    fn from(other: Vector2<T>) -> LogicalVector {
        LogicalVector { vec: Vector2::new(other.x.into(), other.y.into()) }
    }
}

impl From<dpi::LogicalSize<f32>> for LogicalVector {
    fn from(other: dpi::LogicalSize<f32>) -> LogicalVector {
        LogicalVector { vec: Vector2::new(other.width, other.height) }
    }
}
impl Into<dpi::LogicalSize<f32>> for LogicalVector {
    fn into(self) -> dpi::LogicalSize<f32> {
        dpi::LogicalSize::<f32> { width: self.vec.x, height: self.vec.y }
    }
}

impl From<dpi::LogicalPosition<f32>> for LogicalVector {
    fn from(other: dpi::LogicalPosition<f32>) -> LogicalVector {
        LogicalVector { vec: Vector2::new(other.x, other.y) }
    }
}
impl Into<dpi::LogicalPosition<f32>> for LogicalVector {
    fn into(self) -> dpi::LogicalPosition<f32> {
        dpi::LogicalPosition::<f32> { x: self.vec.x, y: self.vec.y }
    }
}

pub trait FromPhysical<T> {
    fn from_physical(source: T, scale_factor: f32) -> Self;
}
impl<T: Into<f64>> FromPhysical<dpi::PhysicalSize<T>> for LogicalVector {
    fn from_physical(source: dpi::PhysicalSize<T>, scale_factor: f32) -> Self {
        let vec = Vector2::new(source.width.into() as f32, source.height.into() as f32);
        LogicalVector { vec: vec / scale_factor }
    }
}
impl<T: Into<f64>> FromPhysical<dpi::PhysicalPosition<T>> for LogicalVector {
    fn from_physical(source: dpi::PhysicalPosition<T>, scale_factor: f32) -> Self {
        let vec = Vector2::new(source.x.into() as f32, source.y.into() as f32);
        LogicalVector { vec: vec / scale_factor }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct LogicalRect {
    /// The position of the top left corner of this rectangle
    pub pos: LogicalVector,
    pub size: LogicalVector,
}
impl LogicalRect {
    #[inline]
    pub fn left(&self) -> f32 {
        self.pos.vec.x
    }
    #[inline]
    pub fn right(&self) -> f32 {
        self.pos.vec.x + self.size.vec.x
    }
    #[inline]
    pub fn bottom(&self) -> f32 {
        self.pos.vec.y + self.size.vec.y
    }
    #[inline]
    pub fn top(&self) -> f32 {
        self.pos.vec.y
    }
    #[inline]
    pub fn center(&self) -> LogicalVector {
        self.pos + self.size * 0.5
    }
    pub fn contains(&self, point: LogicalVector) -> bool {
        point.vec.x > self.pos.vec.x
            && point.vec.x < self.pos.vec.x + self.size.vec.x
            && point.vec.y > self.pos.vec.y
            && point.vec.y < self.pos.vec.y + self.size.vec.y
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Length {
    Fixed(f32),
    Stretch { min: f32, max: f32 },
}
impl Default for Length {
    fn default() -> Length {
        Length::Fixed(256.0)
    }
}
#[derive(Debug, Copy, Clone)]
pub enum Alignment {
    Start,
    Center,
    End,
}
impl Default for Alignment {
    fn default() -> Alignment {
        Alignment::Start
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct WidgetPlacement {
    pub width: Length,
    pub height: Length,
    pub horizontal_align: Alignment,
    pub vertical_align: Alignment,
    pub ignore_layout: bool,
    pub margin_left: f32,
    pub margin_right: f32,
    pub margin_top: f32,
    pub margin_bottom: f32,
}

/// Used to represent physical pixel coordinates and dimensions.
///
/// See `LogicalVector`
#[derive(Copy, Clone, Debug)]
pub struct PhysicalVector {
    pub vec: Vector2<f32>,
}

// TODO implement stuff for physical vector


pub trait PickDimension {
    fn vec_mut(v: &mut LogicalVector) -> &mut f32;
    fn vec(v: LogicalVector) -> f32;
    fn margin_start_mut(placement: &mut WidgetPlacement) -> &mut f32;
    fn margin_start(placement: &WidgetPlacement) -> f32;
    fn margin_end_mut(placement: &mut WidgetPlacement) -> &mut f32;
    fn margin_end(placement: &WidgetPlacement) -> f32;
    fn alignment_mut(placement: &mut WidgetPlacement) -> &mut Alignment;
    fn alignment(placement: &WidgetPlacement) -> Alignment;
    fn extent_mut(placement: &mut WidgetPlacement) -> &mut Length;
    fn extent(placement: &WidgetPlacement) -> Length;
    fn rect_pos_mut(rect: &mut LogicalRect) -> &mut f32;
    fn rect_pos(rect: &LogicalRect) -> f32;
    fn rect_size_mut(rect: &mut LogicalRect) -> &mut f32;
    fn rect_size(rect: &LogicalRect) -> f32;
}
pub struct HorDim {}
impl PickDimension for HorDim {
    fn vec_mut(v: &mut LogicalVector) -> &mut f32 {
        &mut v.vec.x
    }
    fn vec(v: LogicalVector) -> f32 {
        v.vec.x
    }
    fn margin_start_mut(placement: &mut WidgetPlacement) -> &mut f32 {
        &mut placement.margin_left
    }
    fn margin_start(placement: &WidgetPlacement) -> f32 {
        placement.margin_left
    }
    fn margin_end_mut(placement: &mut WidgetPlacement) -> &mut f32 {
        &mut placement.margin_right
    }
    fn margin_end(placement: &WidgetPlacement) -> f32 {
        placement.margin_right
    }
    fn alignment_mut(placement: &mut WidgetPlacement) -> &mut Alignment {
        &mut placement.horizontal_align
    }
    fn alignment(placement: &WidgetPlacement) -> Alignment {
        placement.horizontal_align
    }
    fn extent_mut(placement: &mut WidgetPlacement) -> &mut Length {
        &mut placement.width
    }
    fn extent(placement: &WidgetPlacement) -> Length {
        placement.width
    }
    fn rect_pos_mut(rect: &mut LogicalRect) -> &mut f32 {
        &mut rect.pos.vec.x
    }
    fn rect_pos(rect: &LogicalRect) -> f32 {
        rect.pos.vec.x
    }
    fn rect_size_mut(rect: &mut LogicalRect) -> &mut f32 {
        &mut rect.size.vec.x
    }
    fn rect_size(rect: &LogicalRect) -> f32 {
        rect.size.vec.x
    }
}
pub struct VerDim {}
impl PickDimension for VerDim {
    fn vec_mut(v: &mut LogicalVector) -> &mut f32 {
        &mut v.vec.y
    }
    fn vec(v: LogicalVector) -> f32 {
        v.vec.y
    }
    fn margin_start_mut(placement: &mut WidgetPlacement) -> &mut f32 {
        &mut placement.margin_top
    }
    fn margin_start(placement: &WidgetPlacement) -> f32 {
        placement.margin_top
    }
    fn margin_end_mut(placement: &mut WidgetPlacement) -> &mut f32 {
        &mut placement.margin_bottom
    }
    fn margin_end(placement: &WidgetPlacement) -> f32 {
        placement.margin_bottom
    }
    fn alignment_mut(placement: &mut WidgetPlacement) -> &mut Alignment {
        &mut placement.vertical_align
    }
    fn alignment(placement: &WidgetPlacement) -> Alignment {
        placement.vertical_align
    }
    fn extent_mut(placement: &mut WidgetPlacement) -> &mut Length {
        &mut placement.height
    }
    fn extent(placement: &WidgetPlacement) -> Length {
        placement.height
    }
    fn rect_pos_mut(rect: &mut LogicalRect) -> &mut f32 {
        &mut rect.pos.vec.y
    }
    fn rect_pos(rect: &LogicalRect) -> f32 {
        rect.pos.vec.y
    }
    fn rect_size_mut(rect: &mut LogicalRect) -> &mut f32 {
        &mut rect.size.vec.y
    }
    fn rect_size(rect: &LogicalRect) -> f32 {
        rect.size.vec.y
    }
}
