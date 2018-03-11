use muxrpc::Rpc;
use ssb_common::links::FeedId;
use ssb_common::messages::{SequenceNumber, Timestamp};

/// A source request for getting the seq numbers and timestamps of the latest messages of all
/// users in the database.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Latest([(); 0]);

impl Latest {
    /// Create a new `Latest` rpc.
    pub fn new() -> Latest {
        Latest([])
    }
}

const LATEST: [&'static str; 1] = ["latest"];

impl Rpc for Latest {
    fn names() -> &'static [&'static str] {
        &LATEST
    }
}

/// The responses to a `Latest` request.
///
/// Each item holds the id of the feed it describes, the feed's latest sequence number, and the
/// timestamp of the feed's latest message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LatestItem {
    id: FeedId,
    sequence: SequenceNumber,
    ts: Timestamp,
}

impl LatestItem {
    /// Get the `FeedId` of the item.
    pub fn feed_id(&self) -> FeedId {
        self.id
    }

    /// Get a reference to the `FeedId` item.
    pub fn feed_id_ref(&self) -> &FeedId {
        &self.id
    }

    /// Get a reference to the sequence number of the item.
    pub fn sequence_ref(&self) -> &SequenceNumber {
        &self.sequence
    }

    /// Get a reference to the timestamp of the item.
    pub fn timestamp_ref(&self) -> &Timestamp {
        &self.ts
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use test_helpers::*;
    use super::*;

    #[test]
    fn latest() {
        let req = Latest::new();
        test_source::<Latest, LatestItem, Value>(req);
    }
}
