//! The [core ssb rpcs](https://github.com/ssbc/scuttlebot/blob/master/api.md)

use muxrpc::Rpc;
use ssb_common::links::FeedId;

/// A sync request for getting information about the current user.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Whoami([(); 0]);

impl Whoami {
    /// Create a new `Whoami` rpc.
    pub fn new() -> Whoami {
        Whoami([])
    }
}

impl Rpc for Whoami {
    fn names() -> Box<[&'static str]> {
        Box::new(["whoami"])
    }
}

/// The response to a `Whoami` request.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WhoamiResponse {
    id: FeedId,
}

impl WhoamiResponse {
    /// The `FeedId` of the current user.
    pub fn feed_id(&self) -> &FeedId {
        &self.id
    }

    /// Consume the `WhoamiResponse` and return the user's `FeedId`.
    pub fn into_feed_id(self) -> FeedId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use test_helpers::*;
    use super::*;

    #[test]
    fn whoami() {
        let req = Whoami::new();
        test_sync::<Whoami, WhoamiResponse, Value>(req);
    }
}
