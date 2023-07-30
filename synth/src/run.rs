use anyhow::anyhow;
use tinyaudio::{run_output_device, OutputDeviceParameters};

use crate::{clock, osc::Osc, signal, wave};

pub fn run() -> anyhow::Result<()> {
    let params = OutputDeviceParameters {
        sample_rate: 48000,
        channels_count: 1,
        channel_sample_count: 48000,
    };

    let mut clock = clock::Clock {
        time: 0,
        sample_rate: params.sample_rate as u64,
    };

    let freq_osc = Osc {
        frequency: signal::Signal::constant(1.),
        amplitude: signal::Signal::constant(220.),
        offset: signal::Signal::constant(440.),
        wave: wave::triangle,
    };

    let osc = Osc {
        frequency: signal::Signal::new(freq_osc),
        amplitude: signal::Signal::constant(0.1),
        offset: signal::Signal::constant(0.),
        wave: wave::square,
    };

    let _device = run_output_device(params, move |data| {
        for value in data {
            use signal::IsSignal;

            clock.advance();
            *value = osc.value(&clock);
        }
    })
    .map_err(|err| anyhow!("{}", err))?;

    loop {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            dbg!(key);
        }
    }
}
