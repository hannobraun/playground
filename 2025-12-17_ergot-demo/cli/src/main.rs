use std::{collections::HashSet, time::Duration};

use anyhow::anyhow;
use clap::Parser;
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
        env_logger::Env::default().default_filter_or("error"),
    )
    .init();

    #[derive(clap::Parser)]
    struct Args {
        #[arg(long)]
        r1: Option<u8>,

        #[arg(long)]
        g1: Option<u8>,

        #[arg(long)]
        b1: Option<u8>,

        #[arg(long)]
        r2: Option<u8>,

        #[arg(long)]
        g2: Option<u8>,

        #[arg(long)]
        b2: Option<u8>,
    }
    let args = Args::parse();

    let r1 = args.r1.unwrap_or_default();
    let g1 = args.g1.unwrap_or_default();
    let b1 = args.b1.unwrap_or_default();
    let r2 = args.r2.unwrap_or_default();
    let g2 = args.g2.unwrap_or_default();
    let b2 = args.b2.unwrap_or_default();

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
            eprintln!("No network found. Retrying in 2 seconds...");
            sleep(Duration::from_millis(2000)).await;
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
                &[[r1, g1, b1], [r2, g2, b2]],
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
