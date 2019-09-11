extern crate glium;
extern crate glium_text;
extern crate cgmath;

use std::path::Path;
use glium::Surface;
use glium::glutin::{Event, self, WindowEvent};

fn main() {
    use std::fs::File;

    let mut event_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_dimensions((1024, 768).into());
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, context, &event_loop).unwrap();
    let system = glium_text::TextSystem::new(&display);

    let font = match std::env::args().nth(1) {
        Some(file) => glium_text::FontTexture::new(&display, File::open(&Path::new(&file)).unwrap(), 70),
        None => {
            match File::open(&Path::new("C:\\Windows\\Fonts\\Arial.ttf")) {
                Ok(f) => glium_text::FontTexture::new(&display, f, 70),
                Err(_) => glium_text::FontTexture::new(&display, &include_bytes!("font.ttf")[..], 70),
            }
        }
    }.unwrap();

    let mut buffer = String::new();

    println!("Type with your keyboard");

    let (w, h) = display.get_framebuffer_dimensions();

    let matrix:[[f32; 4]; 4] = cgmath::Matrix4::new(
        0.1, 0.0, 0.0, 0.0,
        0.0, 0.1 * (w as f32) / (h as f32), 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -0.9, 0.0, 0.0, 1.0f32,
    ).into();

    let mut window_should_closed = false;

    while !window_should_closed {
        let text = glium_text::TextDisplay::new(&system, &font, &buffer);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        glium_text::draw(&text, &system, &mut target, matrix, (1.0, 1.0, 0.0, 1.0));
        target.finish().unwrap();

        event_loop.poll_events(|event| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::ReceivedCharacter('\r') => buffer.clear(),
                    WindowEvent::ReceivedCharacter(c) if c as u32 == 8 => { buffer.pop(); },
                    WindowEvent::ReceivedCharacter(chr) => buffer.push(chr),
                    WindowEvent::CloseRequested => window_should_closed = true,
                    _ => ()
                }
            }
        });
    }
}
