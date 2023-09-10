use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

// TODO: may need put somewhere common
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display, PartialEq, Eq, Hash)]
pub enum Locale {
    #[strum(serialize = "ar-SA")]
    ArabicSaudiArabia,

    #[strum(serialize = "de-DE")]
    GermanGermany,

    #[strum(serialize = "en-AU")]
    EnglishAustralia,

    #[strum(serialize = "en-CA")]
    EnglishCanada,

    #[strum(serialize = "en-GB")]
    EnglishGreatBritain,

    #[strum(serialize = "en-IN")]
    EnglishIndia,

    #[strum(serialize = "en-US")]
    EnglishUnitedStates,

    #[strum(serialize = "es-ES")]
    SpanishSpain,

    #[strum(serialize = "es-MX")]
    SpanishMexico,

    #[strum(serialize = "es-US")]
    SpanishUnitedStates,

    #[strum(serialize = "fr-CA")]
    FrenchCanada,

    #[strum(serialize = "fr-FR")]
    FrenchFrance,

    #[strum(serialize = "hi-IN")]
    HindiIndia,

    #[strum(serialize = "it-IT")]
    ItalianItaly,

    #[strum(serialize = "ja-JP")]
    JapaneseJapan,

    #[strum(serialize = "pt-BR")]
    PortugueseBrazil,
}
