use std::{num::NonZeroU32, sync::Arc};

use anyhow::anyhow;
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use softbuffer::{SoftBufferError, Surface};
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, OwnedDisplayHandle},
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{BYTES_PER_PIXEL, GRID_SIZE, PIXELS_SIZE_BYTES};

pub fn start_and_wait(
    lifeline_tx: Sender<()>,
    pixels_rx: Receiver<[u8; 4096]>,
) -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;

    let mut app = WindowApp {
        window: None,
        renderer: None,
        pixels: None,
        pixels_rx,
    };
    event_loop.run_app(&mut app)?;

    drop(lifeline_tx);

    Ok(())
}

struct WindowApp {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    pixels: Option<[u8; PIXELS_SIZE_BYTES]>,
    pixels_rx: Receiver<[u8; PIXELS_SIZE_BYTES]>,
}

impl WindowApp {
    fn init(&mut self, event_loop: &ActiveEventLoop) -> anyhow::Result<()> {
        let window = {
            let window = event_loop.create_window(
                WindowAttributes::default().with_title("Snake | StackAssembly"),
            )?;

            Arc::new(window)
        };

        let surface = {
            let context =
                softbuffer::Context::new(event_loop.owned_display_handle())
                    .map_err(|err| {
                        anyhow!("Failed to create context: {err:?}")
                    })?;

            Surface::new(&context, window.clone())
                .map_err(|err| anyhow!("Failed to create surface: {err:?}"))?
        };

        self.window = Some(window);
        self.renderer = Some(Renderer { surface });

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
        let (Some(window), Some(renderer)) = (&self.window, &mut self.renderer)
        else {
            return;
        };

        loop {
            match self.pixels_rx.try_recv() {
                Ok(pxs) => {
                    self.pixels = Some(pxs);
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(TryRecvError::Disconnected) => {
                    event_loop.exit();
                    return;
                }
            }
        }

        let Some(pixels) = self.pixels else {
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
                if let Err(err) = renderer.draw(window, pixels) {
                    eprintln!("Failed to draw pixels: {err:?}");
                    event_loop.exit();
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        let Some(window) = &self.window else {
            return;
        };

        // We want to redraw on every frame. Otherwise, the window will only
        // redraw in response to some events, like losing or gaining focus.
        window.request_redraw();
    }
}

struct Renderer {
    surface: Surface<OwnedDisplayHandle, Arc<Window>>,
}

impl Renderer {
    pub fn draw(
        &mut self,
        window: &Window,
        pixels: [u8; 4096],
    ) -> Result<(), SoftBufferError> {
        let size = window.inner_size();
        let [Some(width), Some(height)] =
            [size.width, size.height].map(NonZeroU32::new)
        else {
            return Ok(());
        };
        self.surface.resize(width, height)?;

        let [Ok(width_usize), Ok(height_usize)]: [Result<usize, _>; 2] =
            [width, height].map(NonZeroU32::get).map(TryInto::try_into)
        else {
            unreachable!("Surface dimensions can be represented as `usize`.");
        };

        let mut buffer = self.surface.buffer_mut()?;

        for (target_index, target) in buffer.iter_mut().enumerate() {
            let target_x = target_index % width_usize;
            let target_y = target_index / width_usize;

            let source_x = target_x * GRID_SIZE / width_usize;
            let source_y = target_y * GRID_SIZE / height_usize;

            let source_i = (source_y * GRID_SIZE + source_x) * BYTES_PER_PIXEL;

            let [r, g, b, a] = pixels[source_i..source_i + BYTES_PER_PIXEL]
            else {
                unreachable!(
                    "Four-element slice destructures into 4 elements."
                );
            };

            let source = u32::from_le_bytes([r, g, b, a]);

            *target = source;
        }

        buffer.present()?;

        Ok(())
    }
}
