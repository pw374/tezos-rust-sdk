use tezos_core::types::encoded::BlockPayloadHash;

use super::{OperationContentTag, TraitOperationConsensusContent, TraitOperationContent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endorsement {
    slot: u16,
    level: i32,
    round: i32,
    block_payload_hash: BlockPayloadHash,
}

impl Endorsement {
    pub fn new(slot: u16, level: i32, round: i32, block_payload_hash: BlockPayloadHash) -> Self {
        Self {
            slot,
            level,
            round,
            block_payload_hash,
        }
    }
}

impl TraitOperationContent for Endorsement {
    fn tag() -> OperationContentTag {
        OperationContentTag::Endorsement
    }
}

impl TraitOperationConsensusContent for Endorsement {
    fn slot(&self) -> u16 {
        self.slot
    }

    fn level(&self) -> i32 {
        self.level
    }

    fn round(&self) -> i32 {
        self.round
    }

    fn block_payload_hash(&self) -> &BlockPayloadHash {
        &self.block_payload_hash
    }
}