use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
pub enum Country {
    #[strum(serialize = "AF")]
    Afghanistan,
    #[strum(serialize = "AX")]
    AlandIslands,
    #[strum(serialize = "AL")]
    Albania,
    #[strum(serialize = "DZ")]
    Algeria,
    #[strum(serialize = "AS")]
    AmericanSamoa,
    #[strum(serialize = "AD")]
    Andorra,
    #[strum(serialize = "AO")]
    Angola,
    #[strum(serialize = "AI")]
    Anguilla,
    #[strum(serialize = "AQ")]
    Antarctica,
    #[strum(serialize = "AG")]
    AntiguaAndBarbuda,
    #[strum(serialize = "AR")]
    Argentina,
    #[strum(serialize = "AM")]
    Armenia,
    #[strum(serialize = "AW")]
    Aruba,
    #[strum(serialize = "AU")]
    Australia,
    #[strum(serialize = "AT")]
    Austria,
    #[strum(serialize = "AZ")]
    Azerbaijan,
    #[strum(serialize = "BS")]
    Bahamas,
    #[strum(serialize = "BH")]
    Bahrain,
    #[strum(serialize = "BD")]
    Bangladesh,
    #[strum(serialize = "BB")]
    Barbados,
    #[strum(serialize = "BY")]
    Belarus,
    #[strum(serialize = "BE")]
    Belgium,
    #[strum(serialize = "BZ")]
    Belize,
    #[strum(serialize = "BJ")]
    Benin,
    #[strum(serialize = "BM")]
    Bermuda,
    #[strum(serialize = "BT")]
    Bhutan,
    #[strum(serialize = "BO")]
    Bolivia,
    #[strum(serialize = "BA")]
    BosniaAndHerzegovina,
    #[strum(serialize = "BW")]
    Botswana,
    #[strum(serialize = "BV")]
    BouvetIsland,
    #[strum(serialize = "BR")]
    Brazil,
    #[strum(serialize = "IO")]
    BritishIndianOceanTerritory,
    #[strum(serialize = "BN")]
    BruneiDarussalam,
    #[strum(serialize = "BG")]
    Bulgaria,
    #[strum(serialize = "BF")]
    BurkinaFaso,
    #[strum(serialize = "BI")]
    Burundi,
    #[strum(serialize = "KH")]
    Cambodia,
    #[strum(serialize = "CM")]
    Cameroon,
    #[strum(serialize = "CA")]
    Canada,
    #[strum(serialize = "CV")]
    CapeVerde,
    #[strum(serialize = "KY")]
    CaymanIslands,
    #[strum(serialize = "CF")]
    CentralAfricanRepublic,
    #[strum(serialize = "TD")]
    Chad,
    #[strum(serialize = "CL")]
    Chile,
    #[strum(serialize = "CN")]
    China,
    #[strum(serialize = "CX")]
    ChristmasIsland,
    #[strum(serialize = "CC")]
    CocosIslands,
    #[strum(serialize = "CO")]
    Colombia,
    #[strum(serialize = "KM")]
    Comoros,
    #[strum(serialize = "CD")]
    DemocraticRepublicOfTheCongo,
    #[strum(serialize = "CK")]
    CookIslands,
    #[strum(serialize = "CR")]
    CostaRica,
    #[strum(serialize = "HR")]
    Croatia,
    #[strum(serialize = "CU")]
    Cuba,
    #[strum(serialize = "CY")]
    Cyprus,
    #[strum(serialize = "CZ")]
    CzechRepublic,
    #[strum(serialize = "DK")]
    Denmark,
    #[strum(serialize = "DJ")]
    Djibouti,
    #[strum(serialize = "DM")]
    Dominica,
    #[strum(serialize = "DO")]
    DominicanRepublic,
    #[strum(serialize = "EC")]
    Ecuador,
    #[strum(serialize = "EG")]
    Egypt,
    #[strum(serialize = "SV")]
    ElSalvador,
    #[strum(serialize = "GQ")]
    EquatorialGuinea,
    #[strum(serialize = "ER")]
    Eritrea,
    #[strum(serialize = "EE")]
    Estonia,
    #[strum(serialize = "ET")]
    Ethiopia,
    #[strum(serialize = "FK")]
    FalklandIslands,
    #[strum(serialize = "FO")]
    FaroeIslands,
    #[strum(serialize = "FJ")]
    Fiji,
    #[strum(serialize = "FI")]
    Finland,
    #[strum(serialize = "FR")]
    France,
    #[strum(serialize = "GF")]
    FrenchGuiana,
    #[strum(serialize = "PF")]
    FrenchPolynesia,
    #[strum(serialize = "TF")]
    FrenchSouthernTerritories,
    #[strum(serialize = "GA")]
    Gabon,
    #[strum(serialize = "GM")]
    Gambia,
    #[strum(serialize = "GE")]
    Georgia,
    #[strum(serialize = "DE")]
    Germany,
    #[strum(serialize = "GH")]
    Ghana,
    #[strum(serialize = "GI")]
    Gibraltar,
    #[strum(serialize = "GR")]
    Greece,
    #[strum(serialize = "GL")]
    Greenland,
    #[strum(serialize = "GD")]
    Grenada,
    #[strum(serialize = "GP")]
    Guadeloupe,
    #[strum(serialize = "GU")]
    Guam,
    #[strum(serialize = "GT")]
    Guatemala,
    #[strum(serialize = "GG")]
    Guernsey,
    #[strum(serialize = "GN")]
    Guinea,
    #[strum(serialize = "GW")]
    GuineaBissau,
    #[strum(serialize = "GY")]
    Guyana,
    #[strum(serialize = "HT")]
    Haiti,
    #[strum(serialize = "HM")]
    HeardIslandAndMcdonaldIslands,
    #[strum(serialize = "VA")]
    HolySee,
    #[strum(serialize = "HN")]
    Honduras,
    #[strum(serialize = "HK")]
    HongKong,
    #[strum(serialize = "HU")]
    Hungary,
    #[strum(serialize = "IS")]
    Iceland,
    #[strum(serialize = "IN")]
    India,
    #[strum(serialize = "ID")]
    Indonesia,
    #[strum(serialize = "IR")]
    Iran,
    #[strum(serialize = "IQ")]
    Iraq,
    #[strum(serialize = "IE")]
    Ireland,
    #[strum(serialize = "IM")]
    IsleOfMan,
    #[strum(serialize = "IL")]
    Israel,
    #[strum(serialize = "IT")]
    Italy,
    #[strum(serialize = "CI")]
    IvoryCoast,
    #[strum(serialize = "JM")]
    Jamaica,
    #[strum(serialize = "JP")]
    Japan,
    #[strum(serialize = "JE")]
    Jersey,
    #[strum(serialize = "JO")]
    Jordan,
    #[strum(serialize = "KZ")]
    Kazakhstan,
    #[strum(serialize = "KE")]
    Kenya,
    #[strum(serialize = "KI")]
    Kiribati,
    #[strum(serialize = "KP")]
    NorthKorea,
    #[strum(serialize = "KR")]
    SouthKorea,
    #[strum(serialize = "KW")]
    Kuwait,
    #[strum(serialize = "KG")]
    Kyrgyzstan,
    #[strum(serialize = "LA")]
    Laos,
    #[strum(serialize = "LV")]
    Latvia,
    #[strum(serialize = "LB")]
    Lebanon,
    #[strum(serialize = "LS")]
    Lesotho,
    #[strum(serialize = "LR")]
    Liberia,
    #[strum(serialize = "LY")]
    Libya,
    #[strum(serialize = "LI")]
    Liechtenstein,
    #[strum(serialize = "LT")]
    Lithuania,
    #[strum(serialize = "LU")]
    Luxembourg,
    #[strum(serialize = "MO")]
    Macao,
    #[strum(serialize = "MK")]
    Macedonia,
    #[strum(serialize = "MG")]
    Madagascar,
    #[strum(serialize = "MW")]
    Malawi,
    #[strum(serialize = "MY")]
    Malaysia,
    #[strum(serialize = "MV")]
    Maldives,
    #[strum(serialize = "ML")]
    Mali,
    #[strum(serialize = "MT")]
    Malta,
    #[strum(serialize = "MH")]
    MarshallIslands,
    #[strum(serialize = "MQ")]
    Martinique,
    #[strum(serialize = "MR")]
    Mauritania,
    #[strum(serialize = "MU")]
    Mauritius,
    #[strum(serialize = "YT")]
    Mayotte,
    #[strum(serialize = "MX")]
    Mexico,
    #[strum(serialize = "FM")]
    Micronesia,
    #[strum(serialize = "MD")]
    Moldova,
    #[strum(serialize = "MC")]
    Monaco,
    #[strum(serialize = "MN")]
    Mongolia,
    #[strum(serialize = "ME")]
    Montenegro,
    #[strum(serialize = "MS")]
    Montserrat,
    #[strum(serialize = "MA")]
    Morocco,
    #[strum(serialize = "MZ")]
    Mozambique,
    #[strum(serialize = "MM")]
    Myanmar,
    #[strum(serialize = "NA")]
    Namibia,
    #[strum(serialize = "NR")]
    Nauru,
    #[strum(serialize = "NP")]
    Nepal,
    #[strum(serialize = "NL")]
    Netherlands,
    #[strum(serialize = "AN")]
    NetherlandsAntilles,
    #[strum(serialize = "NC")]
    NewCaledonia,
    #[strum(serialize = "NZ")]
    NewZealand,
    #[strum(serialize = "NI")]
    Nicaragua,
    #[strum(serialize = "NE")]
    Niger,
    #[strum(serialize = "NG")]
    Nigeria,
    #[strum(serialize = "NU")]
    Niue,
    #[strum(serialize = "NF")]
    NorfolkIsland,
    #[strum(serialize = "MP")]
    NorthernMarianaIslands,
    #[strum(serialize = "NO")]
    Norway,
    #[strum(serialize = "OM")]
    Oman,
    #[strum(serialize = "PK")]
    Pakistan,
    #[strum(serialize = "PW")]
    Palau,
    #[strum(serialize = "PS")]
    Palestine,
    #[strum(serialize = "PA")]
    Panama,
    #[strum(serialize = "PG")]
    PapuaNewGuinea,
    #[strum(serialize = "PY")]
    Paraguay,
    #[strum(serialize = "PE")]
    Peru,
    #[strum(serialize = "PH")]
    Philippines,
    #[strum(serialize = "PN")]
    Pitcairn,
    #[strum(serialize = "PL")]
    Poland,
    #[strum(serialize = "PT")]
    Portugal,
    #[strum(serialize = "PR")]
    PuertoRico,
    #[strum(serialize = "QA")]
    Qatar,
    #[strum(serialize = "RE")]
    Reunion,
    #[strum(serialize = "RO")]
    Romania,
    #[strum(serialize = "RU")]
    RussianFederation,
    #[strum(serialize = "RW")]
    Rwanda,
    #[strum(serialize = "BL")]
    SaintBarthelemy,
    #[strum(serialize = "SH")]
    SaintHelena,
    #[strum(serialize = "KN")]
    SaintKittsAndNevis,
    #[strum(serialize = "LC")]
    SaintLucia,
    #[strum(serialize = "MF")]
    SaintMartin,
    #[strum(serialize = "PM")]
    SaintPierreAndMiquelon,
    #[strum(serialize = "VC")]
    SaintVincentAndTheGrenadines,
    #[strum(serialize = "WS")]
    Samoa,
    #[strum(serialize = "SM")]
    SanMarino,
    #[strum(serialize = "ST")]
    SaoTomeAndPrincipe,
    #[strum(serialize = "SA")]
    SaudiArabia,
    #[strum(serialize = "SN")]
    Senegal,
    #[strum(serialize = "RS")]
    Serbia,
    #[strum(serialize = "SC")]
    Seychelles,
    #[strum(serialize = "SL")]
    SierraLeone,
    #[strum(serialize = "SG")]
    Singapore,
    #[strum(serialize = "SK")]
    Slovakia,
    #[strum(serialize = "SI")]
    Slovenia,
    #[strum(serialize = "SB")]
    SolomonIslands,
    #[strum(serialize = "SO")]
    Somalia,
    #[strum(serialize = "ZA")]
    SouthAfrica,
    #[strum(serialize = "GS")]
    SouthGeorgiaAndTheSouthSandwichIslands,
    #[strum(serialize = "ES")]
    Spain,
    #[strum(serialize = "LK")]
    SriLanka,
    #[strum(serialize = "SD")]
    Sudan,
    #[strum(serialize = "SR")]
    Suriname,
    #[strum(serialize = "SJ")]
    SvalbardAndJanMayen,
    #[strum(serialize = "SZ")]
    Swaziland,
    #[strum(serialize = "SE")]
    Sweden,
    #[strum(serialize = "CH")]
    Switzerland,
    #[strum(serialize = "SY")]
    Syria,
    #[strum(serialize = "TW")]
    Taiwan,
    #[strum(serialize = "TJ")]
    Tajikistan,
    #[strum(serialize = "TZ")]
    Tanzania,
    #[strum(serialize = "TH")]
    Thailand,
    #[strum(serialize = "TL")]
    TimorLeste,
    #[strum(serialize = "TG")]
    Togo,
    #[strum(serialize = "TK")]
    Tokelau,
    #[strum(serialize = "TO")]
    Tonga,
    #[strum(serialize = "TT")]
    TrinidadAndTobago,
    #[strum(serialize = "TN")]
    Tunisia,
    #[strum(serialize = "TR")]
    Turkey,
    #[strum(serialize = "TM")]
    Turkmenistan,
    #[strum(serialize = "TC")]
    TurksAndCaicosIslands,
    #[strum(serialize = "TV")]
    Tuvalu,
    #[strum(serialize = "UG")]
    Uganda,
    #[strum(serialize = "UA")]
    Ukraine,
    #[strum(serialize = "AE")]
    UnitedArabEmirates,
    #[strum(serialize = "GB")]
    UnitedKingdom,
    #[strum(serialize = "US")]
    UnitedStates,
    #[strum(serialize = "UM")]
    UnitedStatesMinorOutlyingIslands,
    #[strum(serialize = "UY")]
    Uruguay,
    #[strum(serialize = "UZ")]
    Uzbekistan,
    #[strum(serialize = "VU")]
    Vanuatu,
    #[strum(serialize = "VE")]
    Venezuela,
    #[strum(serialize = "VN")]
    Vietnam,
    #[strum(serialize = "VG")]
    VirginIslandsBritish,
    #[strum(serialize = "VI")]
    VirginIslandsUS,
    #[strum(serialize = "WF")]
    WallisAndFutuna,
    #[strum(serialize = "EH")]
    WesternSahara,
    #[strum(serialize = "YE")]
    Yemen,
    #[strum(serialize = "ZM")]
    Zambia,
    #[strum(serialize = "ZW")]
    Zimbabwe,
}
