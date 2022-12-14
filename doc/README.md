<!-- EDIT /Users/z/rmw/rmw_upnp/doc/README.md -->

[English](#english-readme) | [中文说明](#中文说明)

---

## English Readme

<!-- EDIT /Users/z/rmw/rmw_upnp/doc/en/readme.md -->

rmw_upnp : upnp port map daemon

### Use

[→ examples/main.rs](../examples/main.rs)

```rust
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
```


---

## 中文说明

<!-- EDIT /Users/z/rmw/rmw_upnp/doc/zh/readme.md -->

rmw_upnp : upnp 守护进程

### 使用

[→ examples/main.rs](../examples/main.rs)

```rust
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
```

