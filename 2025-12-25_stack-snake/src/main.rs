use std::{fs::File, io::Read, panic, path::Path, sync::Arc, thread};

use crossbeam_channel::{Receiver, RecvError, select, unbounded};
use notify::{RecursiveMode, Watcher};
use pixels::{Pixels, SurfaceTexture};
use stack_assembly::Eval;
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes, WindowId},
};

fn main() -> anyhow::Result<()> {
    let (lifeline_tx, lifeline_rx) = unbounded();

    let handle = thread::spawn(|| run_script(lifeline_rx));

    let event_loop = EventLoop::new()?;

    let mut app = WindowApp {
        window: None,
        pixels: None,
    };
    event_loop.run_app(&mut app)?;

    drop(lifeline_tx);

    match handle.join() {
        Ok(result) => result?,
        Err(err) => {
            panic::resume_unwind(err);
        }
    }

    Ok(())
}

fn run_script(lifeline: Receiver<()>) -> anyhow::Result<()> {
    let (notify_tx, notify_rx) = unbounded();

    let mut watcher = notify::recommended_watcher(notify_tx)?;
    watcher.watch(Path::new("snake.stack"), RecursiveMode::NonRecursive)?;

    let mut run = 0;

    'outer: loop {
        let mut script = String::new();
        File::open("snake.stack")?.read_to_string(&mut script)?;

        let mut eval = Eval::start(&script);

        let effect = eval.run();
        eprintln!("{run}: Script triggered effect: {effect:?}");

        'inner: loop {
            let event = select! {
                recv(notify_rx) -> event => {
                    event??
                }
                recv(lifeline) -> message => {
                    let Err(RecvError) = message else {
                        unreachable!(
                            "Lifeline channel only exists to get dropped."
                        );
                    };

                    // Channel has been dropped. We're done.
                    return Ok(());
                }
            };

            match event.kind {
                notify::EventKind::Modify(_) => {
                    run += 1;
                    continue 'outer;
                }
                _ => {
                    continue 'inner;
                }
            }
        }
    }
}

struct WindowApp {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
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

            let grid_size = 32;
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
        let Some(pixels) = &self.pixels else {
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
                if let Err(err) = pixels.render() {
                    eprintln!("Failed to draw pixels: {err:?}");
                    event_loop.exit();
                }
            }

            _ => {}
        }
    }
}
