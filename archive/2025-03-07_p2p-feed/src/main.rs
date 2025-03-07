mod item;
mod main_;
mod railway;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    self::main_::main().await
}
