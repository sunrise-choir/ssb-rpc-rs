//! The remote procedure calls used by ssb.
//!
//! Scuttlebutt has a plugin system that allows adding support for more rpcs. This module uses
//! features to control which rpcs are exposed. By default, this includes the rpcs of the core
//! [scuttlebot](https://github.com/ssbc/scuttlebot/blob/master/api.md) module, and the rpcs of
//! the plugins which are
//! [bundled with scuttlebot](https://github.com/ssbc/scuttlebot/tree/master/plugins).

#![deny(missing_docs)]
#![feature(try_from)] // tests only
#![feature(ip_constructors)] // tests only

extern crate muxrpc;
extern crate ssb_common;
extern crate serde;
#[macro_use(Serialize, Deserialize)]
extern crate serde_derive;

#[cfg(test)]
extern crate box_stream;
#[cfg(test)]
extern crate futures;
#[cfg(test)]
extern crate sodiumoxide;
#[cfg(test)]
extern crate secret_stream;
#[cfg(test)]
extern crate serde_json;
#[cfg(test)]
extern crate ssb_keyfile;
#[cfg(test)]
extern crate tokio;
#[cfg(test)]
extern crate tokio_io;

#[cfg(feature = "ssb")]
pub mod ssb;
#[cfg(test)]
mod test_helpers;
