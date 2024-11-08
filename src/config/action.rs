use enum_iterator::Sequence;
use serde::Deserialize;

/// This enum lists all of the basic operations supported by the viewer.
#[derive(Clone, Copy, Debug, Deserialize, Sequence)]
pub enum Action {
    /// Shift the image up by a fixed amount.
    ShiftUp,
    /// Shift the image down by a fixed amount.
    ShiftDown,
    /// Shift the image left by a fixed amount.
    ShiftLeft,
    /// Shift the image right by a fixed amount.
    ShiftRight,
    /// Clear the image transformations to reset it to the center of the screen.
    ResetTransform,
    /// Zoom in by a fixed factor.
    ZoomIn,
    /// Zoom out by a fixed factor.
    ZoomOut,
    /// Toggle the viewer window's fullscreen status.
    ToggleFullscreen,
    /// Exit the viewer.
    Quit,
}
