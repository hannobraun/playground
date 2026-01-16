use std::sync::Arc;

use crossbeam_channel::{Receiver, Sender, TryRecvError};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{GRID_SIZE, PIXELS_SIZE_BYTES};

pub fn run(
    lifeline_tx: Sender<()>,
    pixels_rx: Receiver<[u8; 4096]>,
) -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;

    let mut app = WindowApp {
        window: None,
        pixels: None,
        pixels_rx,
    };
    event_loop.run_app(&mut app)?;

    drop(lifeline_tx);

    Ok(())
}

struct WindowApp {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    pixels_rx: Receiver<[u8; PIXELS_SIZE_BYTES]>,
}

impl WindowApp {
    pub fn init(&mut self, event_loop: &ActiveEventLoop) -> anyhow::Result<()> {
        let window = {
            let window = event_loop.create_window(
                WindowAttributes::default().with_title("Snake | StackAssembly"),
            )?;

            Arc::new(window)
        };

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(
                window_size.width,
                window_size.height,
                window.clone(),
            );

            let Ok(grid_size) = GRID_SIZE.try_into() else {
                unreachable!(
                    "Can represent `GRID_SIZE` (`{GRID_SIZE}`) as `u32`."
                );
            };

            Pixels::new(grid_size, grid_size, surface_texture)?
        };

        self.window = Some(window);
        self.pixels = Some(pixels);

        Ok(())
    }
}

impl ApplicationHandler for WindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(err) = self.init(event_loop) {
            eprintln!("Error creating window: {err:?}");
            event_loop.exit();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        let Some(pixels) = &mut self.pixels else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let mut pixels_data = None;

                loop {
                    match self.pixels_rx.try_recv() {
                        Ok(pixels) => {
                            pixels_data = Some(pixels);
                        }
                        Err(TryRecvError::Empty) => {
                            break;
                        }
                        Err(TryRecvError::Disconnected) => {
                            event_loop.exit();
                        }
                    }
                }

                if let Some(pixels_data) = pixels_data {
                    pixels.frame_mut().copy_from_slice(&pixels_data);
                }

                if let Err(err) = pixels.render() {
                    eprintln!("Failed to draw pixels: {err:?}");
                    event_loop.exit();
                }
            }

            _ => {}
        }
    }
}
