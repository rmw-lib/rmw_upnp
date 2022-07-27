use std::{
  net::{IpAddr, Ipv4Addr, SocketAddrV4, TcpStream},
  time::Duration,
};

use anyhow::{Error, Result};
use async_std::task::sleep;
use igd::{aio::search_gateway, AddPortError::PortInUse};
use log::info;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UpnpError {
  #[error("upnp not support ipv6")]
  Ipv6,
}

pub async fn upnp(name: &str, port: u16, duration: u32) -> Result<(SocketAddrV4, Ipv4Addr)> {
  let gateway = search_gateway(Default::default()).await?;
  let gateway_addr = gateway.addr;
  let stream = TcpStream::connect(gateway_addr)?;
  let addr = stream.local_addr()?;
  let ip = addr.ip();
  drop(stream);

  if let IpAddr::V4(ip) = ip {
    let mut retry = true;
    loop {
      match gateway
        .add_port(
          igd::PortMappingProtocol::UDP,
          port,
          SocketAddrV4::new(ip, port),
          duration,
          name,
        )
        .await
      {
        Err(err) => {
          if let PortInUse = err {
            if retry {
              retry = false;
              match gateway
                .remove_port(igd::PortMappingProtocol::UDP, port)
                .await
              {
                Err(err) => {
                  info!("upnp remove port {} error {}", port, err);
                }
                Ok(_) => {
                  continue;
                }
              }
            }
          }
          //info!("upnp {} > {}", gateway_addr, err);
          return Err(err.into());
        }
        Ok(_) => {
          return Ok((gateway_addr, ip));
        }
      }
    }
  } else {
    return Err(UpnpError::Ipv6.into());
  }
}

pub trait Watch {
  fn ok(&self, ip: Ipv4Addr, port: u16, gateway: SocketAddrV4);
  fn err(&self, err: Error);
}

pub struct Log;

impl Watch for Log {
  fn ok(&self, ip: Ipv4Addr, port: u16, gateway: SocketAddrV4) {
    info!("upnp success ( addr {}:{} gateway {})", ip, port, gateway);
  }
  fn err(&self, err: Error) {
    info!("upnp error err : {}", err);
  }
}

pub async fn rmw_upnp(name: &str, port: u16, sleep_seconds: u32) {
  rmw_upnp_watch(name, port, sleep_seconds, Log).await;
}

pub async fn rmw_upnp_watch(name: &str, port: u16, sleep_seconds: u32, watch: impl Watch) {
  let mut local_ip = Ipv4Addr::UNSPECIFIED;
  let mut pre_gateway = SocketAddrV4::new(local_ip, 0);
  let seconds = Duration::from_secs(sleep_seconds.into());
  let duration = sleep_seconds + 60;
  loop {
    match upnp(name, port, duration).await {
      Ok((gateway, ip)) => {
        if ip != local_ip || gateway != pre_gateway {
          local_ip = ip;
          pre_gateway = gateway;
          watch.ok(ip, port, gateway);
        }
      }
      Err(err) => {
        dbg!(err);
      }
    }
    sleep(seconds).await;
  }
}
