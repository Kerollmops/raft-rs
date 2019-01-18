
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntryType {
    EntryNormal,
    EntryConfChange,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Entry {
    pub entry_type: EntryType,
    pub term: u64,
    pub index: u64,
    pub data: Vec<u8>,
    pub context: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SnapshotMetadata {
    pub conf_state: ConfState,
    pub index: u64,
    pub term: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Snapshot {
    pub data: Vec<u8>,
    pub metadata: SnapshotMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HardState {
    pub term: u64,
    pub vote: u64,
    pub commit: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConfState {
    pub nodes: Vec<u64>,
    pub learners: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConfChangeType {
    AddNode,
    RemoveNode,
    AddLearnerNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConfChange {
    pub id: u64,
    pub change_type: ConfChangeType,
    pub node_id: u64,
    pub context: Vec<u8>,
}
