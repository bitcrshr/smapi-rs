use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::shared::locale::Locale;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillManifestPrivacyAndCompliance {
    pub locales: HashMap<Locale, SkillManifestLocalizedPrivacyAndCompliance>,
    pub allows_purchases: bool,
    pub uses_personal_info: bool,
    pub is_child_directed: bool,
    pub is_export_compliant: bool,
    pub contains_ads: bool,
    pub uses_health_info: bool,
    pub shopping_kit: ShoppingKit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillManifestLocalizedPrivacyAndCompliance {
    pub privacy_policy_url: String,
    pub terms_of_use_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingKit {
    pub is_shopping_actions_enabled: bool,
    pub is_amazon_associates_on_alexa_enabled: bool,
}
