//! This file  defines constants representing Kafka error codes.
//! Each constant corresponds to a specific error scenario within the Kafka protocol, indicating
//! what went wrong during a request or response. These codes map directly to the standard Kafka
//! error codes specified by the official protocol documentation.
//!
//! # Usage
//!
//! ```rust
//! // Using an error code to handle a Kafka protocol error
//! let err_code = UNKNOWN_SERVER_ERROR;
//! if err_code == UNKNOWN_SERVER_ERROR {
//!     println!("Kafka encountered an unknown server error.");
//! }
//! ```
//!
//! For more information on Kafka error codes, see:
//! <https://kafka.apache.org/protocol#protocol_error_codes>
//!
//! Below, you'll find each known Kafka error code with a brief description of its meaning and
//! whether or not the error is considered retriable by a client.

/* ---------------------------------------------------------------------------------------------
-1 to 9
--------------------------------------------------------------------------------------------- */

/// UNKNOWN_SERVER_ERROR (-1)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The server experienced an unexpected error when processing the request.
pub const UNKNOWN_SERVER_ERROR: i16 = -1;

/// NONE (0)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// No error.
pub const NONE: i16 = 0;

/// OFFSET_OUT_OF_RANGE (1)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The requested offset is not within the range of offsets maintained by the server.
pub const OFFSET_OUT_OF_RANGE: i16 = 1;

/// CORRUPT_MESSAGE (2)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// This message has failed its CRC checksum, exceeds the valid size, has a null key for a
/// compacted topic, or is otherwise corrupt.
pub const CORRUPT_MESSAGE: i16 = 2;

/// UNKNOWN_TOPIC_OR_PARTITION (3)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// This server does not host this topic-partition.
pub const UNKNOWN_TOPIC_OR_PARTITION: i16 = 3;

/// INVALID_FETCH_SIZE (4)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The requested fetch size is invalid.
pub const INVALID_FETCH_SIZE: i16 = 4;

/// LEADER_NOT_AVAILABLE (5)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// There is no leader for this topic-partition as we are in the middle of a leadership election.
pub const LEADER_NOT_AVAILABLE: i16 = 5;

/// NOT_LEADER_OR_FOLLOWER (6)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// For requests intended only for the leader, this error indicates that the broker is not
/// the current leader. For requests intended for any replica, this error indicates that
/// the broker is not a replica of the topic partition.
pub const NOT_LEADER_OR_FOLLOWER: i16 = 6;

/// REQUEST_TIMED_OUT (7)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The request timed out.
pub const REQUEST_TIMED_OUT: i16 = 7;

/// BROKER_NOT_AVAILABLE (8)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The broker is not available.
pub const BROKER_NOT_AVAILABLE: i16 = 8;

/* ---------------------------------------------------------------------------------------------
9 to 19
--------------------------------------------------------------------------------------------- */

/// REPLICA_NOT_AVAILABLE (9)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The replica is not available for the requested topic-partition. Produce/Fetch requests and
/// other requests intended only for the leader or follower return NOT_LEADER_OR_FOLLOWER
/// if the broker is not a replica of the topic-partition.
pub const REPLICA_NOT_AVAILABLE: i16 = 9;

/// MESSAGE_TOO_LARGE (10)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The request included a message larger than the max message size the server will accept.
pub const MESSAGE_TOO_LARGE: i16 = 10;

/// STALE_CONTROLLER_EPOCH (11)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The controller moved to another broker.
pub const STALE_CONTROLLER_EPOCH: i16 = 11;

/// OFFSET_METADATA_TOO_LARGE (12)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The metadata field of the offset request was too large.
pub const OFFSET_METADATA_TOO_LARGE: i16 = 12;

/// NETWORK_EXCEPTION (13)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The server disconnected before a response was received.
pub const NETWORK_EXCEPTION: i16 = 13;

/// COORDINATOR_LOAD_IN_PROGRESS (14)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The coordinator is loading and hence can't process requests.
pub const COORDINATOR_LOAD_IN_PROGRESS: i16 = 14;

/// COORDINATOR_NOT_AVAILABLE (15)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The coordinator is not available.
pub const COORDINATOR_NOT_AVAILABLE: i16 = 15;

/// NOT_COORDINATOR (16)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// This is not the correct coordinator for the requested group or transaction.
pub const NOT_COORDINATOR: i16 = 16;

/// INVALID_TOPIC_EXCEPTION (17)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The request attempted to perform an operation on an invalid topic.
pub const INVALID_TOPIC_EXCEPTION: i16 = 17;

/// RECORD_LIST_TOO_LARGE (18)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The request included a message batch larger than the configured segment size on the server.
pub const RECORD_LIST_TOO_LARGE: i16 = 18;

/// NOT_ENOUGH_REPLICAS (19)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// Messages are rejected since there are fewer in-sync replicas than required.
pub const NOT_ENOUGH_REPLICAS: i16 = 19;

/* ---------------------------------------------------------------------------------------------
20 to 30
--------------------------------------------------------------------------------------------- */

/// NOT_ENOUGH_REPLICAS_AFTER_APPEND (20)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// Messages are written to the log, but to fewer in-sync replicas than required.
pub const NOT_ENOUGH_REPLICAS_AFTER_APPEND: i16 = 20;

/// INVALID_REQUIRED_ACKS (21)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// Produce request specified an invalid value for required acks.
pub const INVALID_REQUIRED_ACKS: i16 = 21;

/// ILLEGAL_GENERATION (22)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// Specified group generation ID is not valid.
pub const ILLEGAL_GENERATION: i16 = 22;

/// INCONSISTENT_GROUP_PROTOCOL (23)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The group member's supported protocols are incompatible with those of existing members,
/// or the first group member tried to join with empty protocol type or empty protocol list.
pub const INCONSISTENT_GROUP_PROTOCOL: i16 = 23;

/// INVALID_GROUP_ID (24)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The configured groupId is invalid.
pub const INVALID_GROUP_ID: i16 = 24;

/// UNKNOWN_MEMBER_ID (25)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The coordinator is not aware of this member.
pub const UNKNOWN_MEMBER_ID: i16 = 25;

/// INVALID_SESSION_TIMEOUT (26)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The session timeout is not within the range allowed by the broker (as configured by
/// group.min.session.timeout.ms and group.max.session.timeout.ms).
pub const INVALID_SESSION_TIMEOUT: i16 = 26;

/// REBALANCE_IN_PROGRESS (27)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The group is rebalancing, so a rejoin is needed.
pub const REBALANCE_IN_PROGRESS: i16 = 27;

/// INVALID_COMMIT_OFFSET_SIZE (28)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The committing offset data size is not valid.
pub const INVALID_COMMIT_OFFSET_SIZE: i16 = 28;

/// TOPIC_AUTHORIZATION_FAILED (29)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// Topic authorization failed.
pub const TOPIC_AUTHORIZATION_FAILED: i16 = 29;

/// GROUP_AUTHORIZATION_FAILED (30)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// Group authorization failed.
pub const GROUP_AUTHORIZATION_FAILED: i16 = 30;

/* ---------------------------------------------------------------------------------------------
31 to 40
--------------------------------------------------------------------------------------------- */

/// CLUSTER_AUTHORIZATION_FAILED (31)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// Cluster authorization failed.
pub const CLUSTER_AUTHORIZATION_FAILED: i16 = 31;

/// INVALID_TIMESTAMP (32)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The timestamp of the message is out of acceptable range.
pub const INVALID_TIMESTAMP: i16 = 32;

/// UNSUPPORTED_SASL_MECHANISM (33)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The broker does not support the requested SASL mechanism.
pub const UNSUPPORTED_SASL_MECHANISM: i16 = 33;

/// ILLEGAL_SASL_STATE (34)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// Request is not valid given the current SASL state.
pub const ILLEGAL_SASL_STATE: i16 = 34;

/// UNSUPPORTED_VERSION (35)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The version of API is not supported.
pub const UNSUPPORTED_VERSION: i16 = 35;

/// TOPIC_ALREADY_EXISTS (36)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// A topic with this name already exists.
pub const TOPIC_ALREADY_EXISTS: i16 = 36;

/// INVALID_PARTITIONS (37)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The number of partitions is below 1.
pub const INVALID_PARTITIONS: i16 = 37;

/// INVALID_REPLICATION_FACTOR (38)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The replication factor is below 1 or larger than the number of available brokers.
pub const INVALID_REPLICATION_FACTOR: i16 = 38;

/// INVALID_REPLICA_ASSIGNMENT (39)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// Replica assignment is invalid.
pub const INVALID_REPLICA_ASSIGNMENT: i16 = 39;

/// INVALID_CONFIG (40)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// A provided configuration is invalid.
pub const INVALID_CONFIG: i16 = 40;

/* ---------------------------------------------------------------------------------------------
41 to 50
--------------------------------------------------------------------------------------------- */

/// NOT_CONTROLLER (41)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// This is not the correct controller for this cluster.
pub const NOT_CONTROLLER: i16 = 41;

/// INVALID_REQUEST (42)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The request is malformed or incompatible with this broker's version. The broker logs may
/// have more details.
pub const INVALID_REQUEST: i16 = 42;

/// UNSUPPORTED_FOR_MESSAGE_FORMAT (43)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The broker's message format version does not support this request.
pub const UNSUPPORTED_FOR_MESSAGE_FORMAT: i16 = 43;

/// POLICY_VIOLATION (44)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The request parameters do not satisfy the configured policy.
pub const POLICY_VIOLATION: i16 = 44;

/// OUT_OF_ORDER_SEQUENCE_NUMBER (45)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The broker received an out-of-order sequence number.
pub const OUT_OF_ORDER_SEQUENCE_NUMBER: i16 = 45;

/// DUPLICATE_SEQUENCE_NUMBER (46)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The broker received a duplicate sequence number.
pub const DUPLICATE_SEQUENCE_NUMBER: i16 = 46;

/// INVALID_PRODUCER_EPOCH (47)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The producer attempted to produce with an old epoch.
pub const INVALID_PRODUCER_EPOCH: i16 = 47;

/// INVALID_TXN_STATE (48)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The producer attempted a transactional operation in an invalid state.
pub const INVALID_TXN_STATE: i16 = 48;

/// INVALID_PRODUCER_ID_MAPPING (49)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The producer attempted to use a producer ID which is not currently assigned to its transactional ID.
pub const INVALID_PRODUCER_ID_MAPPING: i16 = 49;

/// INVALID_TRANSACTION_TIMEOUT (50)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The transaction timeout is larger than the maximum value allowed by the broker
/// (transaction.max.timeout.ms).
pub const INVALID_TRANSACTION_TIMEOUT: i16 = 50;

/* ---------------------------------------------------------------------------------------------
51 to 60
--------------------------------------------------------------------------------------------- */

/// CONCURRENT_TRANSACTIONS (51)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The producer attempted to update a transaction while another concurrent operation on
/// the same transaction was ongoing.
pub const CONCURRENT_TRANSACTIONS: i16 = 51;

/// TRANSACTION_COORDINATOR_FENCED (52)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// Indicates that the transaction coordinator sending a WriteTxnMarker is no longer
/// the current coordinator for a given producer.
pub const TRANSACTION_COORDINATOR_FENCED: i16 = 52;

/// TRANSACTIONAL_ID_AUTHORIZATION_FAILED (53)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// Transactional ID authorization failed.
pub const TRANSACTIONAL_ID_AUTHORIZATION_FAILED: i16 = 53;

/// SECURITY_DISABLED (54)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// Security features are disabled.
pub const SECURITY_DISABLED: i16 = 54;

/// OPERATION_NOT_ATTEMPTED (55)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The broker did not attempt to execute this operation (e.g., in a batch of operations,
/// some failed earlier, causing the rest to be skipped).
pub const OPERATION_NOT_ATTEMPTED: i16 = 55;

/// KAFKA_STORAGE_ERROR (56)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// A disk error occurred when trying to access the log file on the disk.
pub const KAFKA_STORAGE_ERROR: i16 = 56;

/// LOG_DIR_NOT_FOUND (57)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The user-specified log directory was not found in the broker config.
pub const LOG_DIR_NOT_FOUND: i16 = 57;

/// SASL_AUTHENTICATION_FAILED (58)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// SASL Authentication failed.
pub const SASL_AUTHENTICATION_FAILED: i16 = 58;

/// UNKNOWN_PRODUCER_ID (59)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The broker could not locate producer metadata associated with the producerId. This can
/// happen if a producer's records were deleted due to retention time elapsing.
pub const UNKNOWN_PRODUCER_ID: i16 = 59;

/// REASSIGNMENT_IN_PROGRESS (60)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// A partition reassignment is in progress.
pub const REASSIGNMENT_IN_PROGRESS: i16 = 60;

/* ---------------------------------------------------------------------------------------------
61 to 70
--------------------------------------------------------------------------------------------- */

/// DELEGATION_TOKEN_AUTH_DISABLED (61)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The Delegation Token feature is not enabled.
pub const DELEGATION_TOKEN_AUTH_DISABLED: i16 = 61;

/// DELEGATION_TOKEN_NOT_FOUND (62)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The Delegation Token is not found on the server.
pub const DELEGATION_TOKEN_NOT_FOUND: i16 = 62;

/// DELEGATION_TOKEN_OWNER_MISMATCH (63)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The specified principal is not a valid owner/renewer for the token.
pub const DELEGATION_TOKEN_OWNER_MISMATCH: i16 = 63;

/// DELEGATION_TOKEN_REQUEST_NOT_ALLOWED (64)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// Delegation Token requests are not allowed on PLAINTEXT/1-way SSL channels or
/// on delegation token authenticated channels.
pub const DELEGATION_TOKEN_REQUEST_NOT_ALLOWED: i16 = 64;

/// DELEGATION_TOKEN_AUTHORIZATION_FAILED (65)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// Delegation Token authorization failed.
pub const DELEGATION_TOKEN_AUTHORIZATION_FAILED: i16 = 65;

/// DELEGATION_TOKEN_EXPIRED (66)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The Delegation Token is expired.
pub const DELEGATION_TOKEN_EXPIRED: i16 = 66;

/// INVALID_PRINCIPAL_TYPE (67)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The supplied principalType is not supported.
pub const INVALID_PRINCIPAL_TYPE: i16 = 67;

/// NON_EMPTY_GROUP (68)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The group is not empty.
pub const NON_EMPTY_GROUP: i16 = 68;

/// GROUP_ID_NOT_FOUND (69)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The group ID does not exist.
pub const GROUP_ID_NOT_FOUND: i16 = 69;

/// FETCH_SESSION_ID_NOT_FOUND (70)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The fetch session ID was not found.
pub const FETCH_SESSION_ID_NOT_FOUND: i16 = 70;

/* ---------------------------------------------------------------------------------------------
71 to 80
--------------------------------------------------------------------------------------------- */

/// INVALID_FETCH_SESSION_EPOCH (71)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The fetch session epoch is invalid.
pub const INVALID_FETCH_SESSION_EPOCH: i16 = 71;

/// LISTENER_NOT_FOUND (72)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// No listener on the leader broker matches the listener on which the metadata request was processed.
pub const LISTENER_NOT_FOUND: i16 = 72;

/// TOPIC_DELETION_DISABLED (73)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// Topic deletion is disabled.
pub const TOPIC_DELETION_DISABLED: i16 = 73;

/// FENCED_LEADER_EPOCH (74)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The leader epoch in the request is older than the epoch on the broker.
pub const FENCED_LEADER_EPOCH: i16 = 74;

/// UNKNOWN_LEADER_EPOCH (75)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The leader epoch in the request is newer than the epoch on the broker.
pub const UNKNOWN_LEADER_EPOCH: i16 = 75;

/// UNSUPPORTED_COMPRESSION_TYPE (76)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The requesting client does not support the compression type for the given partition.
pub const UNSUPPORTED_COMPRESSION_TYPE: i16 = 76;

/// STALE_BROKER_EPOCH (77)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The broker epoch has changed.
pub const STALE_BROKER_EPOCH: i16 = 77;

/// OFFSET_NOT_AVAILABLE (78)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The leader high watermark has not caught up after a recent leader election,
/// so offsets cannot be guaranteed to be monotonically increasing.
pub const OFFSET_NOT_AVAILABLE: i16 = 78;

/// MEMBER_ID_REQUIRED (79)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The group member needs to have a valid member ID before entering a consumer group.
pub const MEMBER_ID_REQUIRED: i16 = 79;

/// PREFERRED_LEADER_NOT_AVAILABLE (80)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The preferred leader was not available.
pub const PREFERRED_LEADER_NOT_AVAILABLE: i16 = 80;

/* ---------------------------------------------------------------------------------------------
81 to 90
--------------------------------------------------------------------------------------------- */

/// GROUP_MAX_SIZE_REACHED (81)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The consumer group has reached its maximum size.
pub const GROUP_MAX_SIZE_REACHED: i16 = 81;

/// FENCED_INSTANCE_ID (82)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// A static consumer with the same group.instance.id has registered with a different member.id,
/// thereby fencing this member.
pub const FENCED_INSTANCE_ID: i16 = 82;

/// ELIGIBLE_LEADERS_NOT_AVAILABLE (83)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// Eligible topic partition leaders are not available.
pub const ELIGIBLE_LEADERS_NOT_AVAILABLE: i16 = 83;

/// ELECTION_NOT_NEEDED (84)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// A leader election is not needed for the specified partition.
pub const ELECTION_NOT_NEEDED: i16 = 84;

/// NO_REASSIGNMENT_IN_PROGRESS (85)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// No partition reassignment is currently in progress.
pub const NO_REASSIGNMENT_IN_PROGRESS: i16 = 85;

/// GROUP_SUBSCRIBED_TO_TOPIC (86)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// Deleting offsets of a topic is forbidden while a consumer group is actively subscribed to it.
pub const GROUP_SUBSCRIBED_TO_TOPIC: i16 = 86;

/// INVALID_RECORD (87)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The record failed broker validation and will be rejected.
pub const INVALID_RECORD: i16 = 87;

/// UNSTABLE_OFFSET_COMMIT (88)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// There are unstable offsets that must be cleared.
pub const UNSTABLE_OFFSET_COMMIT: i16 = 88;

/// THROTTLING_QUOTA_EXCEEDED (89)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The throttling quota has been exceeded.
pub const THROTTLING_QUOTA_EXCEEDED: i16 = 89;

/// PRODUCER_FENCED (90)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// A newer producer with the same transactional ID has fenced the current producer.
pub const PRODUCER_FENCED: i16 = 90;

/* ---------------------------------------------------------------------------------------------
91 to 100
--------------------------------------------------------------------------------------------- */

/// RESOURCE_NOT_FOUND (91)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// A request illegally referred to a resource that does not exist.
pub const RESOURCE_NOT_FOUND: i16 = 91;

/// DUPLICATE_RESOURCE (92)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// A request illegally referred to the same resource more than once.
pub const DUPLICATE_RESOURCE: i16 = 92;

/// UNACCEPTABLE_CREDENTIAL (93)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The requested credential does not meet criteria for acceptability.
pub const UNACCEPTABLE_CREDENTIAL: i16 = 93;

/// INCONSISTENT_VOTER_SET (94)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// Either the sender or recipient of a voter-only request is not one of the expected voters.
pub const INCONSISTENT_VOTER_SET: i16 = 94;

/// INVALID_UPDATE_VERSION (95)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The update version provided was invalid.
pub const INVALID_UPDATE_VERSION: i16 = 95;

/// FEATURE_UPDATE_FAILED (96)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// Unable to update finalized features due to an unexpected server error.
pub const FEATURE_UPDATE_FAILED: i16 = 96;

/// PRINCIPAL_DESERIALIZATION_FAILURE (97)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// Request principal deserialization failed during forwarding, indicating an
/// internal broker cluster security setup issue.
pub const PRINCIPAL_DESERIALIZATION_FAILURE: i16 = 97;

/// SNAPSHOT_NOT_FOUND (98)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The requested snapshot was not found on the server.
pub const SNAPSHOT_NOT_FOUND: i16 = 98;

/// POSITION_OUT_OF_RANGE (99)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The requested position is out of valid bounds for the snapshot.
pub const POSITION_OUT_OF_RANGE: i16 = 99;

/// UNKNOWN_TOPIC_ID (100)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The server does not host this topic ID.
pub const UNKNOWN_TOPIC_ID: i16 = 100;

/* ---------------------------------------------------------------------------------------------
101 to 110
--------------------------------------------------------------------------------------------- */

/// DUPLICATE_BROKER_REGISTRATION (101)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// This broker ID is already in use.
pub const DUPLICATE_BROKER_REGISTRATION: i16 = 101;

/// BROKER_ID_NOT_REGISTERED (102)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The given broker ID was not registered.
pub const BROKER_ID_NOT_REGISTERED: i16 = 102;

/// INCONSISTENT_TOPIC_ID (103)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The log's topic ID did not match the topic ID in the request.
pub const INCONSISTENT_TOPIC_ID: i16 = 103;

/// INCONSISTENT_CLUSTER_ID (104)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The cluster ID in the request does not match that found on the server.
pub const INCONSISTENT_CLUSTER_ID: i16 = 104;

/// TRANSACTIONAL_ID_NOT_FOUND (105)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The transactionalId could not be found on the broker.
pub const TRANSACTIONAL_ID_NOT_FOUND: i16 = 105;

/// FETCH_SESSION_TOPIC_ID_ERROR (106)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The fetch session encountered inconsistent topic ID usage.
pub const FETCH_SESSION_TOPIC_ID_ERROR: i16 = 106;

/// INELIGIBLE_REPLICA (107)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The new ISR contains at least one ineligible replica.
pub const INELIGIBLE_REPLICA: i16 = 107;

/// NEW_LEADER_ELECTED (108)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The AlterPartition request updated the partition state, but the leader has changed.
pub const NEW_LEADER_ELECTED: i16 = 108;

/// OFFSET_MOVED_TO_TIERED_STORAGE (109)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The requested offset is moved to tiered storage.
pub const OFFSET_MOVED_TO_TIERED_STORAGE: i16 = 109;

/// FENCED_MEMBER_EPOCH (110)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The group coordinator fenced the member epoch. The member must abandon all its partitions
/// and rejoin.
pub const FENCED_MEMBER_EPOCH: i16 = 110;

/* ---------------------------------------------------------------------------------------------
111 to 120
--------------------------------------------------------------------------------------------- */

/// UNRELEASED_INSTANCE_ID (111)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The instance ID is still used by another member in the consumer group. That member must
/// leave first.
pub const UNRELEASED_INSTANCE_ID: i16 = 111;

/// UNSUPPORTED_ASSIGNOR (112)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The assignor or its version range is not supported by the consumer group.
pub const UNSUPPORTED_ASSIGNOR: i16 = 112;

/// STALE_MEMBER_EPOCH (113)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The member epoch is stale. The member must retry after receiving its updated member epoch
/// via the ConsumerGroupHeartbeat API.
pub const STALE_MEMBER_EPOCH: i16 = 113;

/// MISMATCHED_ENDPOINT_TYPE (114)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The request was sent to an endpoint of the wrong type.
pub const MISMATCHED_ENDPOINT_TYPE: i16 = 114;

/// UNSUPPORTED_ENDPOINT_TYPE (115)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// This endpoint type is not supported.
pub const UNSUPPORTED_ENDPOINT_TYPE: i16 = 115;

/// UNKNOWN_CONTROLLER_ID (116)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// This controller ID is not known.
pub const UNKNOWN_CONTROLLER_ID: i16 = 116;

/// UNKNOWN_SUBSCRIPTION_ID (117)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The client sent a push telemetry request with an invalid or outdated subscription ID.
pub const UNKNOWN_SUBSCRIPTION_ID: i16 = 117;

/// TELEMETRY_TOO_LARGE (118)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The client sent a push telemetry request larger than the maximum size the broker will accept.
pub const TELEMETRY_TOO_LARGE: i16 = 118;

/// INVALID_REGISTRATION (119)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The controller has determined that the broker registration is invalid.
pub const INVALID_REGISTRATION: i16 = 119;

/// TRANSACTION_ABORTABLE (120)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The server encountered an error with the transaction. The client can abort the transaction
/// to continue using this transactional ID.
pub const TRANSACTION_ABORTABLE: i16 = 120;

/* ---------------------------------------------------------------------------------------------
121 to 130
--------------------------------------------------------------------------------------------- */

/// INVALID_RECORD_STATE (121)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The record state is invalid; the acknowledgement of delivery could not be completed.
pub const INVALID_RECORD_STATE: i16 = 121;

/// SHARE_SESSION_NOT_FOUND (122)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The share session was not found.
pub const SHARE_SESSION_NOT_FOUND: i16 = 122;

/// INVALID_SHARE_SESSION_EPOCH (123)
///
/// **RETRIABLE**: True
///
/// **DESCRIPTION**:
/// The share session epoch is invalid.
pub const INVALID_SHARE_SESSION_EPOCH: i16 = 123;

/// FENCED_STATE_EPOCH (124)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The share coordinator rejected the request because the share-group state epoch did not match.
pub const FENCED_STATE_EPOCH: i16 = 124;

/// INVALID_VOTER_KEY (125)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The voter key does not match the receiving replica's key.
pub const INVALID_VOTER_KEY: i16 = 125;

/// DUPLICATE_VOTER (126)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The voter is already part of the set of voters.
pub const DUPLICATE_VOTER: i16 = 126;

/// VOTER_NOT_FOUND (127)
///
/// **RETRIABLE**: False
///
/// **DESCRIPTION**:
/// The voter is not part of the set of voters.
pub const VOTER_NOT_FOUND: i16 = 127;
