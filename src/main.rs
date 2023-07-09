use pixels::Error;
use winit::{event_loop::EventLoop, event::{Event, WindowEvent}};

mod window;

pub static mut HEIGHT: u32 = 400;
pub static mut WIDTH: u32 = 640;

fn main() -> Result<(), Error>{
    let event_loop = EventLoop::new();
    let mut gw = window::GameWindow::new("snake", &event_loop)?;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                println!("Redraw requested");
                verline(gw.pixels.frame_mut(), 0, 2, 203, &[255, 0, 0, 255], 10.);
                gw.pixels.render().unwrap();
            }

            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                println!("Window closed");
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }

            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                println!("Window resized to {:?}", size);
                gw.resize((size.width, size.height));
            }

            // Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
            //     println!("Keyboard input detected");
            //     match input.virtual_keycode {
            //         Some(VirtualKeyCode::Up) if input.state == ElementState::Pressed => {
            //             snake.change_direction(snake::Direction::Up);
            //         }
            //         Some(VirtualKeyCode::Down) if input.state == ElementState::Pressed => {
            //             snake.change_direction(snake::Direction::Down);
            //         }
            //         Some(VirtualKeyCode::Left) if input.state == ElementState::Pressed => {
            //             snake.change_direction(snake::Direction::Left);
            //         }
            //         Some(VirtualKeyCode::Right) if input.state == ElementState::Pressed => {
            //             snake.change_direction(snake::Direction::Right);
            //         }
            //         _ => {}
            //     }
            // }

            _ => {}
        }

        gw.window.request_redraw();
    });
}

fn verline(frame: &mut [u8], x: usize, y1: usize, y2: usize, rgba: &[u8; 4], thickness: f64) {
    let width;
    unsafe { width = WIDTH as usize; }

    let half_thickness = (thickness / 2.0).ceil() as i64;

    for t in -half_thickness..=half_thickness {
        let x = if ((x as i64 + t) as usize) < width {
            (x as i64 + t) as usize
        } else {
            x
        };

        for y in y1..=y2 {
            let index = (y * width + x) * 4;
            if index < frame.len() && index + 3 < frame.len() {
                frame[index] = rgba[0];
                frame[index + 1] = rgba[1];
                frame[index + 2] = rgba[2];
                frame[index + 3] = rgba[3];
            }
        }
    }
}
