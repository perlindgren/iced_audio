//! Common types for textures in `iced_audio`.

/// The texture padding around a bounding rectangle. This is useful when the
/// texture is larger than the intended bounds of the widget.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TexturePadding {
    /// Padding above the bounding rectangle in pixels
    pub top: f32,
    /// Padding below the bounding rectangle in pixels
    pub bottom: f32,
    /// Padding to the left of the bounding rectangle in pixels
    pub left: f32,
    /// Padding to the right of the bounding rectangle in pixels
    pub right: f32,
}

impl Default for TexturePadding {
    fn default() -> Self {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
        }
    }
}

impl TexturePadding {
    /// Creates a new `TexturePadding` with `top`, `bottom`, `left`, and `right`
    /// all set to `padding`.
    pub fn from_single(padding: f32) -> Self {
        Self {
            top: padding,
            bottom: padding,
            left: padding,
            right: padding,
        }
    }

    /// Creates a new `TexturePadding`
    ///
    /// # Arguments
    /// * `h_padding` - padding for `left` and `right`
    /// * `v_padding` - padding for `top` and `bottom`
    pub fn from_v_h(h_padding: f32, v_padding: f32) -> Self {
        Self {
            top: v_padding,
            bottom: v_padding,
            left: h_padding,
            right: h_padding,
        }
    }
}
