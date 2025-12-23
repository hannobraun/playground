use anyhow::anyhow;
use ergot::{Address, toolkits::nusb_v0_1::RouterStack};

use shared::LedEndpoint;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let net_stack = RouterStack::new();

    let () = net_stack
        .endpoints()
        .request::<LedEndpoint>(
            Address::unknown(),
            &[[255, 0, 0], [0, 255, 0]],
            None,
        )
        .await
        .map_err(|err| anyhow!("{err:?}"))?;

    Ok(())
}
