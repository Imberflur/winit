use pixels::{wgpu::Surface, Pixels, SurfaceTexture};
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{DeviceEvent, Event};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Mouse motion test")
        .with_inner_size(LogicalSize::new(600, 200))
        .with_min_inner_size(LogicalSize::new(600, 200))
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let surface = Surface::create(&window);
        let surface_texture = SurfaceTexture::new(600, 200, surface);
        Pixels::new(600, 200, surface_texture).unwrap()
    };

    let mut pos = 0.0;

    let mut start: Option<Instant> = None;
    let mut last_cleared = Instant::now();

    let mut poll_count = 0;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                if let Some(start) = &start {
                    println!("[MainEventsCleared] @ {:.3} us", start.elapsed().as_nanos() as f32 / 1000.0 );
                    println!("{:.3} ms since last MainEventsCleared", last_cleared.elapsed().as_nanos() as f32 / 1_000_000.0);
                    last_cleared = Instant::now();
                }

                if poll_count < 3 {
                    poll_count += 1;
                    return;
                }
                
                if start.is_some() {
                    println!("Rendering");
                }

                draw(pos, pixels.get_frame());
                pixels.render();

                if pos >= 600.0 {
                    *control_flow = ControlFlow::Exit;
                }

                // Sleep
                std::thread::sleep(std::time::Duration::from_millis((1000.0/60.0) as u64));
                poll_count = 0;
            },
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion{ delta: (x, y), recv},
                ..
            } => {
                if start.is_none() {
                    start = Some(Instant::now());
                    last_cleared = Instant::now();
                }
                println!(
                    "[DeviceEvent::MouseMotion({:.3}, {:.3})] @ {:.3} us, {:.3} us after winit created",
                    x,
                    y,
                    start.as_ref().unwrap().elapsed().as_nanos() as f32 / 1000.0,
                    recv.elapsed().as_nanos() as f32 / 1000.0,
                );

                pos += x as f32 / 2.0;
            }
            _ => {}
        }
    }
    );
}

fn draw(pos: f32, frame: &mut [u8]) {
    frame
        .chunks_exact_mut(4)
        .enumerate()
        .map(|(i, pixel)| (((i % 600) as f32) < pos, pixel))
        .for_each(|(colored, pixel)| {
            let rgba = if colored {
                [0xfe, 0x48, 0xe8, 0xff]
            } else {
                [0x08, 0x02, 0x08, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        });
}
