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
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("trace"),
    )
    .init();

    let net_stack = RouterStack::new();

    loop {
        for device in find_new_devices(&HashSet::new()).await {
            let ident = register_router_interface(
                &net_stack,
                device,
                MAX_ERGOT_BUFFER_SIZE,
                OUTGOING_BUFFER_SIZE,
            )
            .await
            .map_err(|err| anyhow!("{err:?}"))?;

            eprintln!("Found new device (ident: `{ident}`).");
        }

        let networks = net_stack.manage_profile(|profile| profile.get_nets());
        let Some(network_id) = networks.into_iter().next() else {
            eprintln!("No network found.");
            continue;
        };

        let result = net_stack
            .endpoints()
            .request::<LedEndpoint>(
                Address {
                    network_id,
                    node_id: 0,
                    port_id: 0,
                },
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
                sleep(Duration::from_millis(2000)).await;
                continue;
            }
        }
    }

    Ok(())
}
