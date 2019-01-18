use std::mem;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntryType {
    EntryNormal,
    EntryConfChange,
}

impl Default for EntryType {
    fn default() -> EntryType {
        EntryType::EntryNormal
    }
}

/// The entry is a type of change that needs to be applied. It contains two data fields.
/// While the fields are built into the model; their usage is determined by the entry_type.
///
/// For normal entries, the data field should contain the data change that should be applied.
/// The context field can be used for any contextual data that might be relevant to the
/// application of the data.
///
/// For configuration changes, the data will contain the ConfChange message and the
/// context will provide anything needed to assist the configuration change. The context
/// if for the user to set and use in this case.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Entry {
    pub entry_type: EntryType,
    pub term: u64,
    pub index: u64,
    pub data: Vec<u8>,
    pub context: Vec<u8>,
}

impl Entry {
    pub fn new() -> Entry {
        Entry::default()
    }

    pub fn take_data(&mut self) -> Vec<u8> {
        mem::replace(&mut self.data, Vec::new())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SnapshotMetadata {
    pub conf_state: ConfState,
    pub index: u64,
    pub term: u64,
}

impl SnapshotMetadata {
    pub fn new() -> SnapshotMetadata {
        SnapshotMetadata::default()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Snapshot {
    pub data: Vec<u8>,
    pub metadata: SnapshotMetadata,
}

impl Snapshot {
    pub fn new() -> Snapshot {
        Snapshot::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageType {
    MsgHup,
    MsgBeat,
    MsgPropose,
    MsgAppend,
    MsgAppendResponse,
    MsgRequestVote,
    MsgRequestVoteResponse,
    MsgSnapshot,
    MsgHeartbeat,
    MsgHeartbeatResponse,
    MsgUnreachable,
    MsgSnapStatus,
    MsgCheckQuorum,
    MsgTransferLeader,
    MsgTimeoutNow,
    MsgReadIndex,
    MsgReadIndexResp,
    MsgRequestPreVote,
    MsgRequestPreVoteResponse,
}

impl Default for MessageType {
    fn default() -> MessageType {
        MessageType::MsgHup
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Message {
    pub msg_type: MessageType,
    pub to: u64,
    pub from: u64,
    pub term: u64,
    pub log_term: u64,
    pub index: u64,
    pub entries: Vec<Entry>,
    pub commit: u64,
    pub snapshot: Snapshot,
    pub reject: bool,
    pub reject_hint: u64,
    pub context: Vec<u8>,
}

impl Message {
    pub fn new() -> Message {
        Message::default()
    }

    pub fn take_entries(&mut self) -> Vec<Entry> {
        mem::replace(&mut self.entries, Vec::new())
    }

    pub fn take_context(&mut self) -> Vec<u8> {
        mem::replace(&mut self.context, Vec::new())
    }

    pub fn take_snapshot(&mut self) -> Snapshot {
        mem::replace(&mut self.snapshot, Snapshot::new())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HardState {
    pub term: u64,
    pub vote: u64,
    pub commit: u64,
}

impl HardState {
    pub fn new() -> HardState {
        HardState::default()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConfState {
    pub nodes: Vec<u64>,
    pub learners: Vec<u64>,
}

impl ConfState {
    pub fn new() -> ConfState {
        ConfState::default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConfChangeType {
    AddNode,
    RemoveNode,
    AddLearnerNode,
}

impl Default for ConfChangeType {
    fn default() -> ConfChangeType {
        ConfChangeType::AddNode
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConfChange {
    pub id: u64,
    pub change_type: ConfChangeType,
    pub node_id: u64,
    pub context: Vec<u8>,
}

impl ConfChange {
    pub fn new() -> ConfChange {
        ConfChange::default()
    }
}
