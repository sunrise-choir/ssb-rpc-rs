//! The remote procedure calls used by ssb.
//!
//! Scuttlebutt has a plugin system that allows adding support for more rpcs. This module uses
//! features to control which rpcs are exposed. By default, this includes the rpcs of the core
//! [scuttlebot](https://github.com/ssbc/scuttlebot/blob/master/api.md) module, and the rpcs of
//! the plugins which are
//! [bundled with scuttlebot](https://github.com/ssbc/scuttlebot/tree/master/plugins).

#![deny(missing_docs)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
