use std::fmt;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataStoreErrorResponse {
    pub error: String,
    pub message: String,
    pub error_details: Vec<DataStoreErrorDetail>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataStoreErrorDetail {
    pub error_detail_type: String,
    pub datastore_error_code: DataStoreErrorCode,
}

#[derive(Deserialize, Debug)]
pub enum DataStoreErrorCode {
    ContentLengthRequired,
    InvalidUniverseId,
    InvalidCursor,
    InvalidVersionId,
    ExistingValueNotNumeric,
    IncrementValueTooLarge,
    IncrementValueTooSmall,
    InvalidDataStoreScope,
    InvalidEntryKey,
    InvalidDataStoreName,
    InvalidStartTime,
    InvalidEndTime,
    InvalidAttributes,
    InvalidUserIds,
    ExclusiveCreateAndMatchVersionCannotBeSet,
    ContentTooBig,
    ChecksumMismatch,
    ContentNotJson,
    InvalidSortOrder,
    Forbidden,
    InsufficientScope,
    DatastoreNotFound,
    EntryNotFound,
    VersionNotFound,
    TooManyRequests,
    Unknown,
}

impl fmt::Display for DataStoreErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let details = self
            .error_details
            .iter()
            .map(|item| format!("{:?}", item.datastore_error_code))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{}] - {}", details, self.message)
    }
}
