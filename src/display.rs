use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use winit::dpi::LogicalSize;

use pixels::{Error, Pixels, SurfaceTexture};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 320;

fn main() {
    println!("CHIP8 EMULATOR");

    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(WIDTH, HEIGHT);
        WindowBuilder::new()
            .with_title("CHIP8 EMULATOR")
            .with_resizable(false)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)
            .unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("WindowEvent::CloseRequested");
                control_flow.set_exit();
            },
            Event::MainEventsCleared => {
                
            },
            Event::RedrawRequested(_) => {
                println!("Event::RedrawRequested");

                let mut frame = pixels.get_frame_mut();
                let colour_one = [ 0x66, 0x00, 0x66, 0xFF ];
                let colour_two = [ 0x66, 0x66, 0x00, 0xFF ];

                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let x = (i % WIDTH as usize) as i16;
                    let y = (i / WIDTH as usize) as i16;

                    let x_grid = x % 10;
                    let y_grid = y % 10;

                    if x_grid == 0 || y_grid == 0 {
                        pixel.copy_from_slice(&colour_one);
                    }
                }

                pixels.render();
            },
            _ => ()
        }
    });
}
