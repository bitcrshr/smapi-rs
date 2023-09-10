use serde::{Deserialize, Serialize};
use strum::EnumString;
use thiserror::Error;

use serde_json::Value as JsonValue;

pub type SmapiResult<T> = Result<T, SmapiError>;

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Error)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[error("Message: {message}\nValidation Details:\n{validation_details:?}")]
pub enum SmapiError {
    ConflictingInstances {
        message: String,
        validation_details: JsonValue,
    },

    ContentParseFailure {
        message: String,
        validation_details: JsonValue,
    },

    DeniedFeatureAccess {
        message: String,
        validation_details: JsonValue,
    },

    DuplicateArrayItems {
        message: String,
        validation_details: JsonValue,
    },

    ExpectedComplianceAgreement {
        message: String,
        validation_details: JsonValue,
    },

    ExpectedRelatedInstance {
        message: String,
        validation_details: JsonValue,
    },

    ExpectedRelatedInstances {
        message: String,
        validation_details: JsonValue,
    },

    InconsistentEndpoints {
        message: String,
        validation_details: JsonValue,
    },

    InvalidArraySize {
        message: String,
        validation_details: JsonValue,
    },

    InvalidContentType {
        message: String,
        validation_details: JsonValue,
    },

    InvalidDataType {
        message: String,
        validation_details: JsonValue,
    },

    InvalidEnumValue {
        message: String,
        validation_details: JsonValue,
    },

    InvalidImageAttributes {
        message: String,
        validation_details: JsonValue,
    },

    InvalidIntegerValue {
        message: String,
        validation_details: JsonValue,
    },

    InvalidRequestParameter {
        message: String,
        validation_details: JsonValue,
    },

    InvalidStringLength {
        message: String,
        validation_details: JsonValue,
    },

    InvalidStringPattern {
        message: String,
        validation_details: JsonValue,
    },

    InvalidUrlDomain {
        message: String,
        validation_details: JsonValue,
    },

    InvalidUrlFormat {
        message: String,
        validation_details: JsonValue,
    },

    MissingRequiredProperty {
        message: String,
        validation_details: JsonValue,
    },

    MutuallyExclusiveArrayItems {
        message: String,
        validation_details: JsonValue,
    },

    ParameterExpired {
        message: String,
        validation_details: JsonValue,
    },

    PreconditionNotMet {
        message: String,
        validation_details: JsonValue,
    },

    OperationNotAllowed {
        message: String,
        validation_details: JsonValue,
    },

    ResourceNotFound {
        message: String,
        validation_details: JsonValue,
    },

    UnexpectedEmptyObject {
        message: String,
        validation_details: JsonValue,
    },

    UnexpectedProperty {
        message: String,
        validation_details: JsonValue,
    },
}
