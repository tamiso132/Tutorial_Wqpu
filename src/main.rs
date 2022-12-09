<<<<<<< HEAD
mod Test;
use pollster;
fn main() {
    println!("Hello, world!");
    pollster::block_on(Test::run());
=======
#![allow(dead_code)]
mod test;

use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1920, 1080))
        .build(&event_loop)
        .unwrap();
    let mut state = test::State::new(&window).await;
    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window_id => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(e) => eprintln!("{:?}", e),
            }
        }

        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        }

        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => state.resize(*physical_size),

            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // new_inner_size is &&mut so we have to dereference it twice
                state.resize(**new_inner_size);
            }
            _ => {}
        },
        _ => {}
    });
}

fn main() {
    println!("Hello, world!");
    pollster::block_on(run());

    enum Test {
        Number(u64),
        String(String),
        Nothing,
    }

    let test = Test::String("hey".to_string());

    match test {
        Test::Number(a) if a == 20 => print!("{a}"),
        Test::Number(a) => print!("{a}"),
        Test::String(a) => print!("{a}"),
        Test::Nothing => print!("Print Nothing!!!!"),
    }
>>>>>>> a91d4e42d152be297baef08850ca0a27f9a6c3ab
}
