use std::iter::once;
use std::sync::Arc;

use cosmic_text::{FontSystem, SwashCache};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::draw::draw;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 600;

const FONT: &[u8] = include_bytes!("../assets/fonts/Oswald-Bold.ttf");

mod draw;

fn main() -> Result<(), Error> {
    let mut state = State {
        font_system: FontSystem::new_with_fonts(once(cosmic_text::fontdb::Source::Binary(
            Arc::new(FONT),
        ))),
        swash_cache: SwashCache::new(),
    };

    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels + Dear ImGui")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    event_loop.run(move |event, _elwt, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::MouseInput {
                state: ElementState::Released,
                ..
            } => {
                window.request_redraw();
            }
            _ => (),
        },
        Event::RedrawRequested(_) => {
            let _ = draw(&mut state, &mut pixels);
            let _ = pixels.render();
        }
        _ => (),
    });
}

pub struct State {
    font_system: FontSystem,
    swash_cache: SwashCache,
}
