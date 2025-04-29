use kurbo::{Affine, Rect, Shape, Stroke};
use peniko::{BlendMode, BrushRef, Color, Fill, Font, StyleRef};

pub type NormalizedCoord = i16;

/// A positioned glyph.
pub struct Glyph {
    pub id: u32,
    pub x: f32,
    pub y: f32,
}

/// A builder for drawing glyphs.
pub trait DrawGlyphs {
    /// Sets the font size in pixels per em units.
    fn font_size(self, size: f32) -> Self;

    /// Whether to enable hinting
    ///
    /// This performs vertical hinting only. Hinting is performed only if the glyph_transform has a uniform scale and no vertical skew or rotation.
    fn hint(self, hint: bool) -> Self;

    /// Sets the normalized design space coordinates for a variable font instance.
    fn normalized_coords(self, coords: &[NormalizedCoord]) -> Self;

    /// Sets the brush.
    fn brush(self, brush: BrushRef<'_>) -> Self;

    /// Sets an additional alpha multiplier for the brush.
    fn brush_alpha(self, alpha: f32) -> Self;

    /// Sets the per-glyph transform. This is applied to all glyphs prior to offset translation.
    /// Use `Affine::skew` with a horizontal-only skew to simulate italic text.
    fn glyph_transform(self, transform: Option<Affine>) -> Self;

    /// Encodes a fill or stroke for the given sequence of glyphs and consumes the builder.
    /// The style parameter accepts either Fill or Stroke types.
    /// This supports emoji fonts in COLR and bitmap formats. style is ignored for these fonts.
    /// For these glyphs, the given brush is used as the “foreground color”, and should be Solid for maximum compatibility.
    fn draw(self, style: StyleRef<'_>, glyphs: impl Iterator<Item = Glyph>);
}

/// The primary drawing abstraction for drawing a single 2D scene
pub trait Scene {
    /// The output type.
    /// This will usually be either a rendered scene or an encoded set of instructions with which to render a scene.
    type Output: 'static;

    /// Builder type for drawing glyphs
    type GlyphBuilder<'a>: DrawGlyphs + 'a
    where
        Self: 'a;

    /// Removes all content from the scene
    fn reset(&mut self);

    /// Pushes a new layer clipped by the specified shape and composed with previous layers using the specified blend mode.
    /// Every drawing command after this call will be clipped by the shape until the layer is popped.
    /// However, the transforms are not saved or modified by the layer stack.
    fn push_layer(
        &mut self,
        blend: impl Into<BlendMode>,
        alpha: f32,
        transform: Affine,
        clip: &impl Shape,
    );

    /// Pops the current layer.
    fn pop_layer(&mut self);

    /// Strokes a shape using the specified style and brush.
    fn stroke(
        &mut self,
        style: &Stroke,
        transform: Affine,
        brush: BrushRef<'_>,
        brush_transform: Option<Affine>,
        shape: &impl Shape,
    );

    /// Fills a shape using the specified style and brush.
    fn fill(
        &mut self,
        style: Fill,
        transform: Affine,
        brush: BrushRef<'_>,
        brush_transform: Option<Affine>,
        shape: &impl Shape,
    );

    /// Returns a builder for encoding a glyph run.
    fn draw_glyphs(&mut self, font: &Font) -> Self::GlyphBuilder<'_>;

    /// Draw a rounded rectangle blurred with a gaussian filter.
    fn draw_blurred_rounded_rect(
        &mut self,
        transform: Affine,
        rect: Rect,
        brush: Color,
        radius: f64,
        std_dev: f64,
    );

    /// Turn the scene into it's output type.
    fn finish(self) -> Self::Output;
}
