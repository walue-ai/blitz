use std::{ops::DerefMut, sync::Arc};

use peniko::{
    kurbo::{Affine, Shape, Stroke},
    BrushRef, Fill,
};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub trait HasHandles: HasDisplayHandle + HasWindowHandle {}
impl<T> HasHandles for T where T: HasDisplayHandle + HasWindowHandle {}

pub trait Renderable {
    fn render(renderer: &mut impl Renderer);
}

pub trait Renderer {
    type Options;
    fn begin(&mut self) -> Self;
    fn push_transform(&mut self, transform: Affine);
    fn push_clip(&mut self, shape: &impl Shape);
    fn pop_layer(&mut self);
    fn stroke<'b>(&mut self, style: &Stroke, brush: impl Into<BrushRef<'b>>, shape: &impl Shape);
    fn fill<'b>(&mut self, style: Fill, brush: impl Into<BrushRef<'b>>, shape: &impl Shape);
}

pub trait ImageRenderer: DerefMut<Target = Self::Renderer> {
    type Renderer: Renderer;
    type Output;
    fn finish(&mut self) -> Self::Output;
}

pub trait WindowRender: DerefMut<Target = Self::Renderer> {
    type Renderer: Renderer;
    type Options;
    type Output;
    fn new<W>(options: Self::Options, window_handle: Arc<dyn HasHandles>) -> Self;
    fn is_active(&self) -> bool;
    fn resume(&mut self);
    fn suspend(&mut self);
    fn set_size(&mut self, physical_width: u32, physical_height: u32);
    fn render(scale: f64, width: u32, height: u32);
}
