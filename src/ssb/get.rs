use muxrpc::Rpc;
use ssb_common::links::MessageId;

/// An async request for getting a `Message` by its `MessageId`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Get([MessageId; 1]);

impl Get {
    /// Create a new `Get` rpc.
    pub fn new(id: MessageId) -> Get {
        Get([id])
    }
}

/// An async request for getting a `Message` by its `MessageId`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub struct GetRef<'id>([&'id MessageId; 1]);

impl<'id> GetRef<'id> {
    /// Create a new `GetRef` rpc.
    pub fn new(id: &'id MessageId) -> GetRef<'id> {
        GetRef([id])
    }
}

const GET: [&'static str; 1] = ["get"];

impl Rpc for Get {
    fn names() -> &'static [&'static str] {
        &GET
    }
}

impl<'id> Rpc for GetRef<'id> {
    fn names() -> &'static [&'static str] {
        &GET
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;
    use test_helpers::*;

    #[test]
    fn get() {
        let req = Get::new(
            "%Igm25FZEje8LeruZ0MnCajFz9e1LoMO3EHB5C0fRMmw=.sha256"
                .parse::<MessageId>()
                .unwrap(),
        );
        test_async::<Get, Value, Value>(req);
    }
}
