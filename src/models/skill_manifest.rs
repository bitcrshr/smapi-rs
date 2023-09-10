use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use apis::SkillManifestApis;
use authorized_client::AuthorizedClient;
use events::SkillManifestEvents;
use permission_items::PermissionItems;
use privacy_and_compliance::SkillManifestPrivacyAndCompliance;
use publishing_information::SkillManifestPublishingInformation;

pub mod apis;
pub mod authorized_client;
pub mod events;
pub mod permission_items;
pub mod privacy_and_compliance;
pub mod publishing_information;

/// Defines the structure for a skill's metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillManifest {
    pub manifest_version: ManifestVersion,
    pub publishing_information: SkillManifestPublishingInformation,
    pub privacy_and_compliance: SkillManifestPrivacyAndCompliance,
    pub events: SkillManifestEvents,
    pub permissions: Vec<PermissionItems>,
    pub authorized_clients: Vec<AuthorizedClient>,
    pub apis: SkillManifestApis,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
pub enum ManifestVersion {
    #[strum(serialize = "1.0")]
    V1,
}
