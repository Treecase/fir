//! Fir SDL2 frontend.

use appview::AppView;
use texture::TextureFactory;

use crate::config::Config;
use crate::meta;

use image::ImageReader;
use std::error::Error;
use std::path::PathBuf;

mod appview;
mod texture;

/// Entry point of the GUI.
///
/// This function serves as the core event loop of the program. It initializes SDL2, creates the
/// window, loads and displays each image file, and manages the event loop.
///
/// TODO: At present, each image is displayed in a unique window, one-by-one. This means that the
/// user has to use the quit action to display the next image, and can't go back to the previous
/// one. In the future, it would be better to have actions for moving forward and backward through
/// the image list, so the quit action actually just exits the whole program.
pub fn start(files: Vec<PathBuf>, config: Config) -> Result<(), Box<dyn Error>> {
    let context = sdl2::init()?;
    let video = context.video()?;
    let window = video.window(meta::NAME, 0, 0).resizable().build()?;

    let mut canvas = window.into_canvas().build()?;
    let img_factory = TextureFactory::new(canvas.texture_creator());

    for path in files {
        let image = ImageReader::open(&path)?.decode()?;
        let window = canvas.window_mut();
        let _ = window.set_size(image.width(), image.height());
        let _ = window.set_title(format!("{} - {}", meta::NAME, path.display()).as_str());

        let image_texture = img_factory.construct_from_image(&image)?;
        let mut view = AppView::new(&mut canvas, &config, image_texture);
        view.present();

        let mut events = context.event_pump()?;
        'mainloop: loop {
            for event in events.poll_iter() {
                view.handle_event(&event);
                if !view.is_running() {
                    break 'mainloop;
                }
            }
            view.present();
        }
    }
    Ok(())
}
