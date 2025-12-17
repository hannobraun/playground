#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as _;
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {}
