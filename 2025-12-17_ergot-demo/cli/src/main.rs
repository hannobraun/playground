use std::time::Duration;

use anyhow::anyhow;
use ergot::{Address, toolkits::nusb_v0_1::RouterStack};

use shared::LedEndpoint;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let net_stack = RouterStack::new();

    loop {
        let result = net_stack
            .endpoints()
            .request::<LedEndpoint>(
                Address::unknown(),
                &[[255, 0, 0], [0, 255, 0]],
                None,
            )
            .await
            .map_err(|err| anyhow!("Failed to set LED color: {err:?}"));

        match result {
            Ok(()) => {
                break;
            }
            Err(err) => {
                eprintln!("{err:?}");
                sleep(Duration::from_millis(500)).await;
                continue;
            }
        }
    }

    Ok(())
}
