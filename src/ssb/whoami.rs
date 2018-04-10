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

const WHOAMI: [&'static str; 1] = ["whoami"];

impl Rpc for Whoami {
    fn names() -> &'static [&'static str] {
        &WHOAMI
    }
}

/// The response to a `Whoami` request.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WhoamiResponse {
    id: FeedId,
}

impl WhoamiResponse {
    /// Get the `FeedId` of the current user.
    pub fn feed_id(&self) -> FeedId {
        self.id
    }

    /// Get a reference to the `FeedId` of the current user.
    pub fn feed_id_ref(&self) -> &FeedId {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;
    use test_helpers::*;

    #[test]
    fn whoami() {
        let req = Whoami::new();
        test_sync::<Whoami, WhoamiResponse, Value>(req);
    }
}
