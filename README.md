# SSB-RPC
The remote procedure calls used by ssb.

Scuttlebutt has a plugin system that allows adding support for more rpcs. This module uses features to control which rpcs are exposed. By default, this includes the rpcs of the core [scuttlebot](https://github.com/ssbc/scuttlebot/blob/master/api.md) module, and the rpcs of the plugins which are [bundled with scuttlebot](https://github.com/ssbc/scuttlebot/tree/master/plugins).

The rpcs for other plugins will be implemented in different crates, but can be exposed (behind a non-default feature flag) in this crate as well.
