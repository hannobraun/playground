use std::{collections::HashSet, time::Duration};

use anyhow::anyhow;
use ergot::{
    Address,
    toolkits::nusb_v0_1::{
        RouterStack, find_new_devices, register_router_interface,
    },
};

use shared::LedEndpoint;
use tokio::time::sleep;

const MAX_ERGOT_BUFFER_SIZE: u16 = 1024;
const OUTGOING_BUFFER_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env("debug,nusb=info").init();

    let net_stack = RouterStack::new();

    loop {
        for device in find_new_devices(&HashSet::new()).await {
            eprintln!("Found new device.");

            register_router_interface(
                &net_stack,
                device,
                MAX_ERGOT_BUFFER_SIZE,
                OUTGOING_BUFFER_SIZE,
            )
            .await
            .map_err(|err| anyhow!("{err:?}"))?;
        }

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
