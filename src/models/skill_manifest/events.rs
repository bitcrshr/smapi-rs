use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::models::shared::event_name::{EventName, EventNameType};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillManifestEvents {
    pub subscriptions: Option<Vec<EventName>>,
    pub publications: Option<Vec<EventPublications>>,
    pub regions: Option<Vec<Region>>,
    pub endpoint: SkillManifestEndpoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventPublications {
    pub event_name: EventNameType, // TODO: spec says string, but i think it's actually the enum?
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    pub endpoint: SkillManifestEndpoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillManifestEndpoint {
    pub uri: String,
    pub ssl_certificate_type: Option<SslCertificateType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum SslCertificateType {
    Wildcard,
    SelfSigned,
    Trusted,
}
