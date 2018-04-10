# SSB-RPC
The remote procedure calls used by ssb.

Scuttlebutt has a plugin system that allows adding support for more rpcs. This module uses features to control which rpcs are exposed. By default, this includes the rpcs of the core [scuttlebot](https://github.com/ssbc/scuttlebot/blob/master/api.md) module, and the rpcs of the plugins which are [bundled with scuttlebot](https://github.com/ssbc/scuttlebot/tree/master/plugins).

The rpcs for other plugins will be implemented in different crates, but can be exposed (behind a non-default feature flag) in this crate as well.

## note on future crate
some of the related repositories use futures 0.2.0, whether others are still stuck at 0.1.x. That's because there are a few blockers with 0.2.0, most importantly [#857](https://github.com/rust-lang-nursery/futures-rs/issues/857) [#858](https://github.com/rust-lang-nursery/futures-rs/issues/858) has been resolved since. And there's also [#623](https://github.com/rust-lang-nursery/futures-rs/issues/623).

These issues prevent a better (API) design of packet-stream, which in turn blocks muxrpc and ssb-client. So the stack is currently in limbo between two APIs, completely at mercy of the futures crate.