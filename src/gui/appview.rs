//! Draws the GUI.

use sdl2::event::{Event, WindowEvent};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use crate::config::{Action, Config, KeyMap};

/// The app view. This is what handles all of the drawing of things to the screen.
pub struct AppView<'a> {
    canvas: &'a mut Canvas<Window>,
    config: &'a Config,

    /// How many times we've zoomed in/out.
    ///
    /// 0 means 1:1 scale, -N means zoom out N times, +N means zoom in N times.
    zoom_level: i32,
    /// The factor to zoom in by.
    ///
    /// For example, a zoom factor of 2 means each zoom level doubles the size of the image when
    /// zooming in, and halves it when zooming out.
    zoom_factor: f32,
    /// Position of the image.
    ///
    /// This position is relative to the center of the window. Therefore this point lies at the
    /// *center* of the image, **NOT** the top left corner!
    image_position: Point,
    /// Bounding-box of the image, relative to [`Self::image_position`].
    image_rect: Rect,
    /// An SDL2 texture which holds the image data.
    image_texture: Texture<'a>,

    /// If true, the view has changed and the window should be redrawn.
    dirty: bool,
    /// If false, the app has exited and the window should be closed.
    is_running: bool,
}

impl<'a> AppView<'a> {
    /// Construct a new AppView.
    pub fn new(
        canvas: &'a mut Canvas<Window>,
        config: &'a Config,
        image_texture: Texture<'a>,
    ) -> Self {
        let t = image_texture.query();
        Self {
            canvas,
            config,
            zoom_level: 0,
            zoom_factor: 2.0,
            image_position: Point::new(0, 0),
            image_rect: Rect::new(
                -((t.width / 2) as i32),
                -((t.height / 2) as i32),
                t.width,
                t.height,
            ),
            image_texture,
            dirty: true,
            is_running: true,
        }
    }

    /// Check if the app is running or terminated.
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Redraws the view, but only if the dirty flag is set.
    ///
    /// This function clears the dirty flag.
    pub fn present(&mut self) {
        self.dirty.then(|| self.do_draw());
    }

    /// Handle an SDL2 input event.
    ///
    /// If a key which is bound to an action is pressed, that action will be executed.
    /// If a quit event occurs, the `is_running` flag will be set to `false`.
    /// This function may set the dirty bit.
    pub fn handle_event(&mut self, event: &Event) {
        if let Some(c) = Command::try_from_event_using_keymap(event, &self.config.binds) {
            self.do_command(&c);
        } else {
            match event {
                Event::Quit { .. } => self.is_running = false,

                Event::Window {
                    win_event: WindowEvent::Exposed,
                    ..
                } => self.dirty = true,

                _ => (),
            }
        }
    }

    /// Execute a viewer command.
    pub fn do_command(&mut self, command: &Command) {
        match command {
            Command::Translate(xrel, yrel) => self.translate(*xrel, *yrel),
            Command::Wrap(a) => match a {
                Action::ShiftUp => self.translate(0, -i32::from(self.config.shift_amount)),
                Action::ShiftDown => self.translate(0, self.config.shift_amount.into()),
                Action::ShiftLeft => self.translate(-i32::from(self.config.shift_amount), 0),
                Action::ShiftRight => self.translate(self.config.shift_amount.into(), 0),
                Action::ResetTransform => self.reset_transform(),
                Action::ZoomIn => self.zoom_in(),
                Action::ZoomOut => self.zoom_out(),
                Action::ToggleFullscreen => self.toggle_fullscreen(),
                Action::Quit => self.quit(),
            },
        };
    }

    // --- Drawing --- /////////////////////////////////////

    fn do_draw(&mut self) {
        let (ow, oh) = self.canvas.output_size().unwrap();
        let rect = self
            .transformed_rect()
            .right_shifted(ow as i32 / 2)
            .bottom_shifted(oh as i32 / 2);

        self.draw_background();
        let _ = self.canvas.copy(&self.image_texture, None, Some(rect));
        self.canvas.present();
        self.dirty = false;
    }

    fn draw_background(&mut self) {
        self.canvas
            .set_draw_color(self.config.grid.color_light.as_rgb());
        self.canvas.clear();

        let canvas_size = self
            .canvas
            .output_size()
            .expect("canvas should have a size");

        let grid_columns = canvas_size.0.div_ceil(self.config.grid.size);
        let grid_rows = canvas_size.1.div_ceil(self.config.grid.size);

        let grid_size = self.config.grid.size;
        let rects = (0..=grid_rows)
            .flat_map(|row| {
                ((row & 1)..=grid_columns).step_by(2).map(move |col| {
                    let width = grid_size;
                    let height = grid_size;
                    let x = col.saturating_mul(width) as i32;
                    let y = row.saturating_mul(height) as i32;
                    Rect::new(x, y, width, height)
                })
            })
            .collect::<Vec<_>>();

        let dark = self.config.grid.color_dark.as_rgb();
        self.canvas.set_draw_color(dark);
        let _ = self.canvas.fill_rects(rects.as_slice());
    }

    // --- Space Conversions --- ///////////////////////////

    fn scale(&self) -> f32 {
        self.zoom_factor.powf(self.zoom_level as f32)
    }

    fn scaled_rect(&self) -> Rect {
        Rect::new(
            (self.image_rect.x as f32 * self.scale()) as i32,
            (self.image_rect.y as f32 * self.scale()) as i32,
            (self.image_rect.w as f32 * self.scale()) as u32,
            (self.image_rect.h as f32 * self.scale()) as u32,
        )
    }

    fn transformed_rect(&self) -> Rect {
        self.scaled_rect()
            .bottom_shifted(self.image_position.y())
            .right_shifted(self.image_position.x())
    }

    // --- Commands --- ////////////////////////////////////////

    fn translate(&mut self, dx: i32, dy: i32) {
        let (ow, oh) = self.canvas.output_size().unwrap();

        let o_rect = Rect::new(ow as i32 / -2, oh as i32 / -2, ow, oh);
        let rect = self.scaled_rect();

        let (wider, narrower) = if o_rect.width() > rect.width() {
            (o_rect, rect)
        } else {
            (rect, o_rect)
        };
        let (taller, shorter) = if o_rect.height() > rect.height() {
            (o_rect, rect)
        } else {
            (rect, o_rect)
        };

        let min_x = wider.left() + narrower.right();
        let min_y = taller.top() + shorter.bottom();
        let max_x = wider.right() + narrower.left();
        let max_y = taller.bottom() + shorter.top();

        let delta = Point::new(dx, dy);
        let new_pos = self.image_position + delta;

        let clamped_x = new_pos.x().clamp(min_x, max_x);
        let clamped_y = new_pos.y().clamp(min_y, max_y);

        self.image_position = Point::new(clamped_x, clamped_y);

        self.dirty = true;
    }

    fn reset_transform(&mut self) {
        self.image_position = Point::new(0, 0);
        self.zoom_level = 0;
        self.dirty = true;
    }

    fn zoom_in(&mut self) {
        self.zoom_level += 1;
        self.dirty = true;
    }
    fn zoom_out(&mut self) {
        self.zoom_level -= 1;
        self.dirty = true;
    }

    fn toggle_fullscreen(&mut self) {
        let w = self.canvas.window_mut();
        let t = match w.fullscreen_state() {
            sdl2::video::FullscreenType::Off => sdl2::video::FullscreenType::True,
            sdl2::video::FullscreenType::True => sdl2::video::FullscreenType::Off,
            sdl2::video::FullscreenType::Desktop => sdl2::video::FullscreenType::Off,
        };
        let _ = w.set_fullscreen(t);
    }

    fn quit(&mut self) {
        self.is_running = false;
    }
}

// --- Command --- /////////////////////////////////////////

/// Commands extend the basic [`Action`] enum with more advanced options.
///
/// This is made necessary by mouse interaction, since actions only define shifts--translations by
/// fixed amounts--whereas the mouse should move the image fluidly.
#[derive(Clone, Copy, Debug)]
pub enum Command {
    /// Translate the image by an arbitrary distance.
    Translate(i32, i32),
    /// Wraps a basic action so it can be passed through.
    Wrap(Action),
}

impl Command {
    /// Try to construct a new `Command` from a given event, using the given keybind set.
    pub fn try_from_event_using_keymap(event: &Event, binds: &KeyMap) -> Option<Self> {
        match event {
            Event::MouseMotion {
                mousestate,
                xrel,
                yrel,
                ..
            } => mousestate
                .left()
                .then_some(Command::Translate(*xrel, *yrel)),

            Event::KeyDown { keycode, .. } => {
                if let Some(key) = *keycode {
                    binds.get(&key.into()).map(|action| {
                        log::info!("mapped {key} to {action:?}");
                        Command::Wrap(*action)
                    })
                } else {
                    None
                }
            }

            _ => None,
        }
    }
}
