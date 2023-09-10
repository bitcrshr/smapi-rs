use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, EnumString, Display)]
pub enum PermissionName {
    #[strum(serialize = "alexa::device_id:read")]
    AlexaDeviceIdRead,
    #[strum(serialize = "alexa::personality:explicit:read")]
    AlexaPersonalityExplicitRead,
    #[strum(serialize = "alexa::authenticate:2:mandatory")]
    AlexaAuthenticate2Mandatory,
    #[strum(serialize = "alexa:devices:all:address:country_and_postal_code:read")]
    AlexaDevicesAllAddressCountryAndPostalCodeRead,
    #[strum(serialize = "alexa::profile:mobile_number:read")]
    AlexaProfileMobileNumberRead,
    #[strum(serialize = "alexa::async_event:write")]
    AlexaAsyncEventWrite,
    #[strum(serialize = "alexa::device_type:read")]
    AlexaDeviceTypeRead,
    #[strum(serialize = "alexa::skill:proactive_enablement")]
    AlexaSkillProactiveEnablement,
    #[strum(serialize = "alexa::personality:explicit:write")]
    AlexaPersonalityExplicitWrite,
    #[strum(serialize = "alexa::household:lists:read")]
    AlexaHouseholdListsRead,
    #[strum(serialize = "alexa::utterance_id:read")]
    AlexaUtteranceIdRead,
    #[strum(serialize = "alexa::user_experience_guidance:read")]
    AlexaUserExperienceGuidanceRead,
    #[strum(serialize = "alexa::devices:all:notifications:write")]
    AlexaDevicesAllNotificationsWrite,
    #[strum(serialize = "avs::distributed_audio")]
    AvsDistributedAudio,
    #[strum(serialize = "alexa::devices:all:address:full:read")]
    AlexaDevicesAllAddressFullRead,
    #[strum(serialize = "alexa::devices:all:notifications:urgent:write")]
    AlexaDevicesAllNotificationsUrgentWrite,
    #[strum(serialize = "payments:autopay_consent")]
    PaymentsAutopayConsent,
    #[strum(serialize = "alexa::alerts:timers:skill:readwrite")]
    AlexaAlertsTimersSkillReadwrite,
    #[strum(serialize = "alexa::customer_id:read")]
    AlexaCustomerIdRead,
    #[strum(serialize = "alexa::skill:cds:monetization")]
    AlexaSkillCdsMonetization,
    #[strum(serialize = "alexa::music:cast")]
    AlexaMusicCast,
    #[strum(serialize = "alexa::profile:given_name:read")]
    AlexaProfileGivenNameRead,
    #[strum(serialize = "alexa::alerts:reminders:skill:readwrite")]
    AlexaAlertsRemindersSkillReadwrite,
    #[strum(serialize = "alexa::household:lists:write")]
    AlexaHouseholdListsWrite,
    #[strum(serialize = "alexa::profile:email:read")]
    AlexaProfileEmailRead,
    #[strum(serialize = "alexa::profile:name:read")]
    AlexaProfileNameRead,
    #[strum(serialize = "alexa::devices:all:geolocation:read")]
    AlexaDevicesAllGeolocationRead,
    #[strum(serialize = "alexa::raw_person_id:read")]
    AlexaRawPersonIdRead,
    #[strum(serialize = "alexa::authenticate:2:optional")]
    AlexaAuthenticate2Optional,
    #[strum(serialize = "alexa::health:profile:write")]
    AlexaHealthProfileWrite,
    #[strum(serialize = "alexa::person_id:read")]
    AlexaPersonIdRead,
    #[strum(serialize = "alexa::skill:products:entitlements")]
    AlexaSkillProductsEntitlements,
    #[strum(serialize = "alexa::energy:devices:state:read")]
    AlexaEnergyDevicesStateRead,
    #[strum(serialize = "alexa::origin_ip_address:read")]
    AlexaOriginIpAddressRead,
    #[strum(serialize = "alexa::devices:all:coarse_location:read")]
    AlexaDevicesAllCoarseLocationRead,
    #[strum(serialize = "alexa::devices:all:tokenized_geolocation:read")]
    AlexaDevicesAllTokenizedGeolocationRead,
    #[strum(serialize = "alexa::devices:all:intent_tokens:read")]
    AlexaDevicesAllIntentTokensRead,
    #[strum(serialize = "alexa::measurement_system::readwrite")]
    AlexaMeasurementSystemReadwrite,
    #[strum(serialize = "dash::vendor:read:endpoints")]
    DashVendorReadEndpoints,
    #[strum(serialize = "dash::read:endpoints:sensors")]
    DashReadEndpointsSensors,
}
