#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    peripherals::PIO0,
    pio::{self, Pio},
    pio_programs::ws2812::{PioWs2812, PioWs2812Program},
};
use panic_halt as _;
use smart_leds::RGB8;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    let program = PioWs2812Program::new(&mut common);
    let mut ws2812 =
        PioWs2812::new(&mut common, sm0, p.DMA_CH0, p.PIN_23, &program);

    ws2812
        .write(&[RGB8::new(255, 0, 0), RGB8::new(0, 255, 0)])
        .await;
}
