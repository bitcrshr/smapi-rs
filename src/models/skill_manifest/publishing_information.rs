use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::{Display, EnumString};

use crate::models::shared::category::Category;
use crate::models::shared::country::Country;
use crate::models::shared::locale::Locale;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillManifestPublishingInformation {
    /// Name of the skill that is displayed to customers in the Alexa app.
    pub name: String,

    /// Description of the skill's purpose and feature and how it works.
    pub description: String,

    pub locales: HashMap<Locale, SkillManifestLocalizedPublishingInformation>,

    pub is_available_worldwide: bool,

    pub distribution_mode: DistributionMode,

    pub gadget_support: ManifestGadgetSupport,

    pub testing_instructions: String,

    pub category: Category,

    pub distribution_countries: Vec<Country>,

    pub automatic_distribution: AutomaticDistribution,

    pub automatic_cloned_locale: AutomaticClonedLocale,

    pub paid_skill_information: PaidSkillInformation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillManifestLocalizedPublishingInformation {
    pub name: String,

    pub small_icon_uri: Option<String>,

    pub large_icon_uri: Option<String>,

    pub summary: Option<String>,

    pub description: Option<String>,

    pub updates_description: Option<String>,

    pub example_phrases: Option<Vec<String>>,

    pub keywords: Option<Vec<String>>,

    pub custom_product_prompts: Option<CustomProductPrompts>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomProductPrompts {
    pub purchase_prompt_description: String,
    pub purchase_confirmation_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum DistributionMode {
    Public,
    Private,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestGadgetSupport {
    pub requirement: GadgetSupportRequirement,
    pub min_gadget_buttons: Option<u8>,
    pub max_gadget_buttons: Option<u8>,
    pub num_players_max: Option<u8>,
    pub num_players_min: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum GadgetSupportRequirement {
    Required,
    Optional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomaticDistribution {
    pub is_active: bool,
    /// yes, this is actually how it's defined in the docs
    pub source_locale_for_languages: Option<Vec<SourceLanguageForLocales>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceLanguageForLocales {
    pub language: String,
    pub source_locale: Locale,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomaticClonedLocale {
    pub locales: Vec<LocalesByAutomaticClonedLocale>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalesByAutomaticClonedLocale {
    pub source_locale: Locale,
    pub locales: Option<Vec<Locale>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaidSkillInformation {
    pub pricing: MarketplacePricing,
    pub tax_information: TaxInformation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketplacePricing {
    pub offer_type: OfferType,
    pub price: u32,
    pub currency: Currency,
    pub free_trial_information: Option<FreeTrialInformation>,
    pub subscription_information: Option<SubscriptionInformation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum OfferType {
    Subscription,
    Entitlement,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
pub enum Currency {
    USD,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreeTrialInformation {
    pub free_trial_duration: crate::util::wrappers::SerDeDuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionInformation {
    pub subscription_payment_frequency: SubscriptionPaymentFrequency,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionPaymentFrequency {
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxInformation {
    pub category: TaxInformationCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TaxInformationCategory {
    Software,
    StreamingAudio,
    StreamingRadio,
    InformationServices,
    Video,
    Periodicals,
    Newspapers,
}
