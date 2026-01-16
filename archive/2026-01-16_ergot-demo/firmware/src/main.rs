#![no_std]
#![no_main]

use core::pin::pin;

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    peripherals::{PIO0, USB},
    pio::{self, Pio},
    pio_programs::ws2812::{PioWs2812, PioWs2812Program},
    usb,
};
use embassy_usb::{UsbDevice, driver::Driver};
use ergot::{
    exports::bbq2::traits::coordination::cs::CsCoord,
    toolkits::embassy_usb_v0_5::{
        self, DEFAULT_TIMEOUT_MS_PER_FRAME, RxWorker, Stack,
        USB_FS_MAX_PACKET_SIZE, WireStorage, new_target_stack, tx_worker,
    },
};
use mutex::raw_impls::cs::CriticalSectionRawMutex;
use panic_halt as _;
use smart_leds::RGB8;
use static_cell::ConstStaticCell;

use shared::LedEndpoint;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<USB>;
    PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
});

type Queue = embassy_usb_v0_5::Queue<4096, CsCoord>;
type UsbDriver = usb::Driver<'static, USB>;

const MAX_PACKET_SIZE: usize = 1024;

static OUT_QUEUE: Queue = Queue::new();

static STACK: Stack<&'static Queue, CriticalSectionRawMutex> =
    new_target_stack(OUT_QUEUE.framed_producer(), MAX_PACKET_SIZE as u16);
static STORAGE: WireStorage = WireStorage::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let driver = usb::Driver::new(p.USB, Irqs);
    // USB VID/PID for testing only: https://pid.codes/1209/0001/
    let config = embassy_usb::Config::new(0x1209, 0x0001);
    let (usb_device, endpoint_in, endpoint_out) =
        STORAGE.init_ergot(driver, config);

    spawner.must_spawn(run_usb(usb_device));
    spawner.must_spawn(run_tx(endpoint_in));
    spawner.must_spawn(run_rx(endpoint_out));

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    let program = PioWs2812Program::new(&mut common);
    let mut ws2812 =
        PioWs2812::new(&mut common, sm0, p.DMA_CH0, p.PIN_23, &program);

    let server = STACK.endpoints().bounded_server::<LedEndpoint, 1>(None);
    let server = pin!(server);
    let mut handle = server.attach();

    loop {
        // This can return a send error, presumably because the response to the
        // request can fail. But we can't do anything about the client not
        // receiving the response, so let's just restart the server if that
        // happens.
        let _ = handle
            .serve(async |&[[r1, g1, b1], [r2, g2, b2]]| {
                ws2812
                    .write(&[RGB8::new(r1, g1, b1), RGB8::new(r2, g2, b2)])
                    .await;
            })
            .await;
    }
}

#[embassy_executor::task]
async fn run_usb(mut device: UsbDevice<'static, UsbDriver>) {
    device.run().await
}

#[embassy_executor::task]
async fn run_tx(mut endpoint_in: <UsbDriver as Driver<'static>>::EndpointIn) {
    tx_worker::<UsbDriver, _, _>(
        &mut endpoint_in,
        OUT_QUEUE.framed_consumer(),
        DEFAULT_TIMEOUT_MS_PER_FRAME,
        USB_FS_MAX_PACKET_SIZE,
    )
    .await;
}

#[embassy_executor::task]
async fn run_rx(endpoint_out: <UsbDriver as Driver<'static>>::EndpointOut) {
    static BUF: ConstStaticCell<[u8; MAX_PACKET_SIZE]> =
        ConstStaticCell::new([0u8; MAX_PACKET_SIZE]);

    RxWorker::<&Queue, CriticalSectionRawMutex, UsbDriver>::new(
        &STACK,
        endpoint_out,
    )
    .run(BUF.take(), USB_FS_MAX_PACKET_SIZE)
    .await;
}
