use anyhow::Result;
use rmw_upnp::daemon;

#[async_std::main]
async fn main() -> Result<()> {
  fern::Dispatch::new()
    .level(log::LevelFilter::Info)
    .level_for("surf", log::LevelFilter::Warn)
    .chain(std::io::stdout())
    .apply()?;

  let sleep_seconds = 60;
  let port = 12345;
  daemon("test_rmw_upnp", port, sleep_seconds).await;
  Ok(())
}
