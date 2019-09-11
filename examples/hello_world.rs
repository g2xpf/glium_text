extern crate glium;
extern crate glium_text;
extern crate cgmath;

use glium::Surface;
use glium::glutin::{Event, self, WindowEvent};

fn main() {
    let mut event_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_dimensions((1024, 768).into());
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, context, &event_loop).unwrap();
    let system = glium_text::TextSystem::new(&display);

    let font = glium_text::FontTexture::new(&display, &include_bytes!("font.ttf")[..], 70).unwrap();

    let text = glium_text::TextDisplay::new(&system, &font, "Hello world!");
    let text_width = text.get_width();
    println!("Text width: {:?}", text_width);

    let mut window_should_closed = false;

    let (w, h) = display.get_framebuffer_dimensions();

    let matrix:[[f32; 4]; 4] = cgmath::Matrix4::new(
        2.0 / text_width, 0.0, 0.0, 0.0,
        0.0, 2.0 * (w as f32) / (h as f32) / text_width, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -1.0, -1.0, 0.0, 1.0f32,
        ).into();

    while !window_should_closed {

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        glium_text::draw(&text, &system, &mut target, matrix, (1.0, 1.0, 0.0, 1.0));
        target.finish().unwrap();

        event_loop.poll_events(|event| match event {
            Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => window_should_closed = true,
            _ => (),
        });
    }
}
