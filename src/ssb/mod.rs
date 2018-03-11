//! The [core ssb rpcs](https://github.com/ssbc/scuttlebot/blob/master/api.md)

mod get;
mod latest;
mod whoami;

pub use self::get::*;
pub use self::latest::*;
pub use self::whoami::*;
