use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillManifestApis {
    pub flash_briefing: Option<flash_briefing::FlashBriefingApis>,
    pub custom: Option<custom::CustomApis>,
    pub knowledge: Option<knowledge::KnowledgeApis>,
    pub smart_home: Option<smart_home::SmartHomeApis>,
    // pub video: Option<VideoApis>,
    // pub alexa_for_business: Option<AlexaForBusinessApis>,
    // pub household_list: Option<HouseholdList>,
    // pub music: Option<MusicApis>,
    // pub demand_response: Option<DemandResponseApis>,
}

pub mod flash_briefing {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};
    use strum::{Display, EnumString};

    use crate::models::shared::locale::Locale;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FlashBriefingApis {
        pub locales: HashMap<Locale, LocalizedFlashBriefingInfo>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LocalizedFlashBriefingInfo {
        pub feeds: Vec<LocalizedFlashBriefingInfoItems>,
        pub custom_error_message: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LocalizedFlashBriefingInfoItems {
        pub logical_name: Option<String>,
        pub name: Option<String>,
        pub url: String,
        pub image_uri: Option<String>,
        pub content_type: FlashBriefingContentType,
        pub genre: FlashBriefingGenre,
        pub update_frequency: FlashBriefingUpdateFrequency,
        pub vui_preamble: Option<String>,
        pub is_default: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum FlashBriefingContentType {
        Text,
        Audio,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum FlashBriefingGenre {
        HeadlineNews,
        Business,
        Politics,
        Entertainment,
        Technology,
        Humor,
        Lifestyle,
        Sports,
        Science,
        HealthAndFitness,
        ArtsAndCulture,
        ProductivityAndUtilities,
        Other,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum FlashBriefingUpdateFrequency {
        Hourly,
        Daily,
        Weekly,
    }
}

pub mod custom {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};
    use strum::{Display, EnumString};

    use crate::models::shared::interface::Interface;
    use crate::models::{shared::locale::Locale, skill_manifest::events::SkillManifestEndpoint};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CustomApis {
        pub _target_runtimes: Vec<TargetRuntime>,
        pub locales: HashMap<Locale, CustomLocalizedInformation>,
        pub regions: String, // TODO: NOT STRING, but need more clarification
        pub endpoint: SkillManifestEndpoint,
        pub interfaces: Vec<Interface>,
        pub tasks: Vec<CustomTask>,
        pub conncetions: CustomConnections,
        pub dialog_management: DialogManagement,
        pub app_link: AppLink,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TargetRuntime {
        #[serde(rename = "type")]
        pub runtime_type: TargetRuntimeType,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum TargetRuntimeType {
        Device,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CustomLocalizedInformation {
        pub dialog_management: CustomLocalizedInformationDialogManagement,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CustomLocalizedInformationDialogManagement {
        pub session_start_delegation_strategy: SessionStartDelegationStrategy,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SessionStartDelegationStrategy {
        pub target: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CustomTask {
        pub name: String,
        pub version: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CustomConnections {
        pub requires: Vec<Connection>,
        pub provides: Vec<Connection>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Connection {
        pub name: String,
        pub payload: ConnectionsPayload,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ConnectionsPayload {
        #[serde(rename = "type")]
        pub payload_type: String,
        pub version: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DialogManagement {
        pub dialog_managers: Vec<DialogManager>,
        pub session_start_delegation_strategy: DialogDelegationStrategy,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DialogManager {
        #[serde(rename = "type")]
        pub dialog_manager_type: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DialogDelegationStrategy {
        pub target: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AppLink {
        pub linked_applications: Vec<LinkedApplication>,
        pub linked_web_domains: Vec<String>,
        pub linked_android_common_intents: Vec<LinkedAndroidCommonIntent>,
        pub linked_common_schemes: LinkedCommonSchemes,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LinkedApplication {
        pub catalog_info: CatalogInfo,
        pub custom_schemes: Vec<String>,
        pub domains: Vec<String>,
        pub friendly_name: FriendlyName,
        pub android_custom_intents: AndroidCustomIntent,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CatalogInfo {
        #[serde(rename = "type")]
        pub catalog_type: CatalogName,
        pub identifier: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum CatalogName {
        IosAppStore,
        GooglePlayStore,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FriendlyName {
        pub default: String,
        pub localized_names: Vec<LocalizedName>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LocalizedName {
        pub locale: Locale,
        pub name: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AndroidCustomIntent {
        pub component: String,
        pub action: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LinkedAndroidCommonIntent {
        pub intent_name: AndroidCommonIntentName,
        pub catalog_type: CatalogName,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum AndroidCommonIntentName {
        ShowInMap,
        AddCalendarEvent,
        PlayMedia,
        StartPhoneCall,
        OpenSettings,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub struct LinkedCommonSchemes {
        pub ios_app_store: Vec<IOSAppStoreCommonSchemeName>,
        pub google_play_store: Vec<PlayStoreCommonSchemeName>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum IOSAppStoreCommonSchemeName {
        Maps,
        Tel,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum PlayStoreCommonSchemeName {
        Maps,
        Tel,
    }
}

pub mod knowledge {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};
    use strum::{Display, EnumString};

    use crate::models::shared::locale::Locale;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct KnowledgeApis {
        pub enablement_channel: KnowledgeApisEnablementChannel,
        pub locales: HashMap<Locale, LocalizedKnowledgeInformation>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
    pub enum KnowledgeApisEnablementChannel {
        #[strum(serialize = "PUBLIC")]
        Public,
        #[strum(serialize = "ASP")]
        ASP,
        #[strum(serialize = "A4B")]
        A4B,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LocalizedKnowledgeInformation {
        pub answer_attribution: String,
    }
}

pub mod smart_home {
    use serde::{Deserialize, Serialize};
    use strum::{Display, EnumString};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SmartHomeApis {
        pub regions: String, // TODO: not string!!!
        pub endpoint: LambdaEndpoint,
        pub protocol_version: SmartHomeProtocol,
        pub supported_controls: SupportedControls,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LambdaEndpoint {
        pub uri: String,
        pub ssl_certificate_type: Option<LambdaSSLCertificateType>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
    pub enum LambdaSSLCertificateType {
        SelfSigned,
        Wildcard,
        Trusted,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
    pub enum SmartHomeProtocol {
        #[strum(serialize = "2")]
        V2,

        #[strum(serialize = "2.0")]
        V2_0,

        #[strum(serialize = "3")]
        V3,

        #[strum(serialize = "3.0")]
        V3_0,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SupportedControls {
        #[serde(rename = "type")]
        pub supported_controls_type: SupportedControlsType,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum SupportedControlsType {
        RemoveVehicleControl,
    }
}
