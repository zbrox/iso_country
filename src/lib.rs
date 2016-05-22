// What is ISO 3166-1?
//
// | ISO 3166-1 is part of the ISO 3166 standard published by the International
// | Organization for Standardization (ISO), and defines codes for the names of
// | countries, dependent territories, and special areas of geographical
// | interest.
// |
// | - [Wikipedia](http://en.wikipedia.org/wiki/ISO_3166-1)

use std::{ fmt, str };
use std::error::Error;
pub mod data;

#[derive(Debug)]
pub enum CountryParseError {
    InvalidCountryCode(String)
}

impl Error for CountryParseError {
    fn description(&self) -> &str { "error parsing country code" }
}

impl fmt::Display for CountryParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl str::FromStr for Country {
    type Err = CountryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match COUNTRY_CODE_SEARCH_TABLE.binary_search_by(|&(o, _)| o.cmp(s)) {
            Ok(pos) => Ok(COUNTRY_CODE_SEARCH_TABLE[pos].1),
            Err(_)  => Err(CountryParseError::InvalidCountryCode(s.to_string()))
        }
    }
}

impl fmt::Display for Country  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Country::Unspecified => Ok(()),
            _ => fmt::Debug::fmt(self, f)
        }
    }
}

impl Country {
    pub fn name(&self) -> &'static str {
        use Country::*;
        match *self {
            Unspecified => "",
            AD => "Andorra",
            AE => "United Arab Emirates",
            AF => "Afghanistan",
            AG => "Antigua and Barbuda",
            AI => "Anguilla",
            AL => "Albania",
            AM => "Armenia",
            AO => "Angola",
            AQ => "Antarctica",
            AR => "Argentina",
            AS => "American Samoa",
            AT => "Austria",
            AU => "Australia",
            AW => "Aruba",
            AX => "Åland Islands",
            AZ => "Azerbaijan",
            BA => "Bosnia and Herzegovina",
            BB => "Barbados",
            BD => "Bangladesh",
            BE => "Belgium",
            BF => "Burkina Faso",
            BG => "Bulgaria",
            BH => "Bahrain",
            BI => "Burundi",
            BJ => "Benin",
            BL => "Saint Barthélemy",
            BM => "Bermuda",
            BN => "Brunei Darussalam",
            BO => "Bolivia (Plurinational State of)",
            BQ => "Bonaire, Sint Eustatius and Saba",
            BR => "Brazil",
            BS => "Bahamas",
            BT => "Bhutan",
            BV => "Bouvet Island",
            BW => "Botswana",
            BY => "Belarus",
            BZ => "Belize",
            CA => "Canada",
            CC => "Cocos (Keeling) Islands",
            CD => "Congo (Democratic Republic of the)",
            CF => "Central African Republic",
            CG => "Congo",
            CH => "Switzerland",
            CI => "Côte d'Ivoire",
            CK => "Cook Islands",
            CL => "Chile",
            CM => "Cameroon",
            CN => "China",
            CO => "Colombia",
            CR => "Costa Rica",
            CU => "Cuba",
            CV => "Cabo Verde",
            CW => "Curaçao",
            CX => "Christmas Island",
            CY => "Cyprus",
            CZ => "Czech Republic",
            DE => "Germany",
            DJ => "Djibouti",
            DK => "Denmark",
            DM => "Dominica",
            DO => "Dominican Republic",
            DZ => "Algeria",
            EC => "Ecuador",
            EE => "Estonia",
            EG => "Egypt",
            EH => "Western Sahara",
            ER => "Eritrea",
            ES => "Spain",
            ET => "Ethiopia",
            FI => "Finland",
            FJ => "Fiji",
            FK => "Falkland Islands",
            FM => "Micronesia (Federated States of)",
            FO => "Faroe Islands",
            FR => "France",
            GA => "Gabon",
            GB => "United Kingdom of Great Britain and Northern Ireland",
            GD => "Grenada",
            GE => "Georgia",
            GF => "French Guiana",
            GG => "Guernsey",
            GH => "Ghana",
            GI => "Gibraltar",
            GL => "Greenland",
            GM => "Gambia",
            GN => "Guinea",
            GP => "Guadeloupe",
            GQ => "Equatorial Guinea",
            GR => "Greece",
            GS => "South Georgia and the South Sandwich Islands",
            GT => "Guatemala",
            GU => "Guam",
            GW => "Guinea-Bissau",
            GY => "Guyana",
            HK => "Hong Kong",
            HM => "Heard Island and McDonald Islands",
            HN => "Honduras",
            HR => "Croatia",
            HT => "Haiti",
            HU => "Hungary",
            ID => "Indonesia",
            IE => "Ireland",
            IL => "Israel",
            IM => "Isle of Man",
            IN => "India",
            IO => "British Indian Ocean Territory",
            IQ => "Iraq",
            IR => "Iran (Islamic Republic of)",
            IS => "Iceland",
            IT => "Italy",
            JE => "Jersey",
            JM => "Jamaica",
            JO => "Jordan",
            JP => "Japan",
            KE => "Kenya",
            KG => "Kyrgyzstan",
            KH => "Cambodia",
            KI => "Kiribati",
            KM => "Comoros",
            KN => "Saint Kitts and Nevis",
            KP => "Korea (Democratic People's Republic of)",
            KR => "Korea (Republic of)",
            KW => "Kuwait",
            KY => "Cayman Islands",
            KZ => "Kazakhstan",
            LA => "Lao People's Democratic Republic",
            LB => "Lebanon",
            LC => "Saint Lucia",
            LI => "Liechtenstein",
            LK => "Sri Lanka",
            LR => "Liberia",
            LS => "Lesotho",
            LT => "Lithuania",
            LU => "Luxembourg",
            LV => "Latvia",
            LY => "Libya",
            MA => "Morocco",
            MC => "Monaco",
            MD => "Moldova (Republic of)",
            ME => "Montenegro",
            MF => "Saint Martin (French part)",
            MG => "Madagascar",
            MH => "Marshall Islands",
            MK => "Macedonia (the former Yugoslav Republic of)",
            ML => "Mali",
            MM => "Myanmar",
            MN => "Mongolia",
            MO => "Macao",
            MP => "Northern Mariana Islands",
            MQ => "Martinique",
            MR => "Mauritania",
            MS => "Montserrat",
            MT => "Malta",
            MU => "Mauritius",
            MV => "Maldives",
            MW => "Malawi",
            MX => "Mexico",
            MY => "Malaysia",
            MZ => "Mozambique",
            NA => "Namibia",
            NC => "New Caledonia",
            NE => "Niger",
            NF => "Norfolk Island",
            NG => "Nigeria",
            NI => "Nicaragua",
            NL => "Netherlands",
            NO => "Norway",
            NP => "Nepal",
            NR => "Nauru",
            NU => "Niue",
            NZ => "New Zealand",
            OM => "Oman",
            PA => "Panama",
            PE => "Peru",
            PF => "French Polynesia",
            PG => "Papua New Guinea",
            PH => "Philippines",
            PK => "Pakistan",
            PL => "Poland",
            PM => "Saint Pierre and Miquelon",
            PN => "Pitcairn",
            PR => "Puerto Rico",
            PS => "Palestine, State of",
            PT => "Portugal",
            PW => "Palau",
            PY => "Paraguay",
            QA => "Qatar",
            RE => "Réunion",
            RO => "Romania",
            RS => "Serbia",
            RU => "Russian Federation",
            RW => "Rwanda",
            SA => "Saudi Arabia",
            SB => "Solomon Islands",
            SC => "Seychelles",
            SD => "Sudan",
            SE => "Sweden",
            SG => "Singapore",
            SH => "Saint Helena, Ascension and Tristan da Cunha",
            SI => "Slovenia",
            SJ => "Svalbard and Jan Mayen",
            SK => "Slovakia",
            SL => "Sierra Leone",
            SM => "San Marino",
            SN => "Senegal",
            SO => "Somalia",
            SR => "Suriname",
            SS => "South Sudan",
            ST => "Sao Tome and Principe",
            SV => "El Salvador",
            SX => "Sint Maarten (Dutch part)",
            SY => "Syrian Arab Republic",
            SZ => "Swaziland",
            TC => "Turks and Caicos Islands",
            TD => "Chad",
            TF => "French Southern Territories",
            TG => "Togo",
            TH => "Thailand",
            TJ => "Tajikistan",
            TK => "Tokelau",
            TL => "Timor-Leste",
            TM => "Turkmenistan",
            TN => "Tunisia",
            TO => "Tonga",
            TR => "Turkey",
            TT => "Trinidad and Tobago",
            TV => "Tuvalu",
            TW => "Taiwan, Province of China[a]",
            TZ => "Tanzania, United Republic of",
            UA => "Ukraine",
            UG => "Uganda",
            UM => "United States Minor Outlying Islands",
            US => "United States of America",
            UY => "Uruguay",
            UZ => "Uzbekistan",
            VA => "Holy See",
            VC => "Saint Vincent and the Grenadines",
            VE => "Venezuela (Bolivarian Republic of)",
            VG => "Virgin Islands (British)",
            VI => "Virgin Islands (U.S.)",
            VN => "Viet Nam",
            VU => "Vanuatu",
            WF => "Wallis and Futuna",
            WS => "Samoa",
            YE => "Yemen",
            YT => "Mayotte",
            ZA => "South Africa",
            ZM => "Zambia",
            ZW => "Zimbabwe",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Country {
    Unspecified = 0,
    AD = 20,
    AE = 784,
    AF = 4,
    AG = 28,
    AI = 660,
    AL = 8,
    AM = 51,
    AO = 24,
    AQ = 10,
    AR = 32,
    AS = 16,
    AT = 40,
    AU = 36,
    AW = 533,
    AX = 248,
    AZ = 31,
    BA = 70,
    BB = 52,
    BD = 50,
    BE = 56,
    BF = 854,
    BG = 100,
    BH = 48,
    BI = 108,
    BJ = 204,
    BL = 652,
    BM = 60,
    BN = 96,
    BO = 68,
    BQ = 535,
    BR = 76,
    BS = 44,
    BT = 64,
    BV = 74,
    BW = 72,
    BY = 112,
    BZ = 84,
    CA = 124,
    CC = 166,
    CD = 180,
    CF = 140,
    CG = 178,
    CH = 756,
    CI = 384,
    CK = 184,
    CL = 152,
    CM = 120,
    CN = 156,
    CO = 170,
    CR = 188,
    CU = 192,
    CV = 132,
    CW = 531,
    CX = 162,
    CY = 196,
    CZ = 203,
    DE = 276,
    DJ = 262,
    DK = 208,
    DM = 212,
    DO = 214,
    DZ = 12,
    EC = 218,
    EE = 233,
    EG = 818,
    EH = 732,
    ER = 232,
    ES = 724,
    ET = 231,
    FI = 246,
    FJ = 242,
    FK = 238,
    FM = 583,
    FO = 234,
    FR = 250,
    GA = 266,
    GB = 826,
    GD = 308,
    GE = 268,
    GF = 254,
    GG = 831,
    GH = 288,
    GI = 292,
    GL = 304,
    GM = 270,
    GN = 324,
    GP = 312,
    GQ = 226,
    GR = 300,
    GS = 239,
    GT = 320,
    GU = 316,
    GW = 624,
    GY = 328,
    HK = 344,
    HM = 334,
    HN = 340,
    HR = 191,
    HT = 332,
    HU = 348,
    ID = 360,
    IE = 372,
    IL = 376,
    IM = 833,
    IN = 356,
    IO = 86,
    IQ = 368,
    IR = 364,
    IS = 352,
    IT = 380,
    JE = 832,
    JM = 388,
    JO = 400,
    JP = 392,
    KE = 404,
    KG = 417,
    KH = 116,
    KI = 296,
    KM = 174,
    KN = 659,
    KP = 408,
    KR = 410,
    KW = 414,
    KY = 136,
    KZ = 398,
    LA = 418,
    LB = 422,
    LC = 662,
    LI = 438,
    LK = 144,
    LR = 430,
    LS = 426,
    LT = 440,
    LU = 442,
    LV = 428,
    LY = 434,
    MA = 504,
    MC = 492,
    MD = 498,
    ME = 499,
    MF = 663,
    MG = 450,
    MH = 584,
    MK = 807,
    ML = 466,
    MM = 104,
    MN = 496,
    MO = 446,
    MP = 580,
    MQ = 474,
    MR = 478,
    MS = 500,
    MT = 470,
    MU = 480,
    MV = 462,
    MW = 454,
    MX = 484,
    MY = 458,
    MZ = 508,
    NA = 516,
    NC = 540,
    NE = 562,
    NF = 574,
    NG = 566,
    NI = 558,
    NL = 528,
    NO = 578,
    NP = 524,
    NR = 520,
    NU = 570,
    NZ = 554,
    OM = 512,
    PA = 591,
    PE = 604,
    PF = 258,
    PG = 598,
    PH = 608,
    PK = 586,
    PL = 616,
    PM = 666,
    PN = 612,
    PR = 630,
    PS = 275,
    PT = 620,
    PW = 585,
    PY = 600,
    QA = 634,
    RE = 638,
    RO = 642,
    RS = 688,
    RU = 643,
    RW = 646,
    SA = 682,
    SB = 90,
    SC = 690,
    SD = 729,
    SE = 752,
    SG = 702,
    SH = 654,
    SI = 705,
    SJ = 744,
    SK = 703,
    SL = 694,
    SM = 674,
    SN = 686,
    SO = 706,
    SR = 740,
    SS = 728,
    ST = 678,
    SV = 222,
    SX = 534,
    SY = 760,
    SZ = 748,
    TC = 796,
    TD = 148,
    TF = 260,
    TG = 768,
    TH = 764,
    TJ = 762,
    TK = 772,
    TL = 626,
    TM = 795,
    TN = 788,
    TO = 776,
    TR = 792,
    TT = 780,
    TV = 798,
    TW = 158,
    TZ = 834,
    UA = 804,
    UG = 800,
    UM = 581,
    US = 840,
    UY = 858,
    UZ = 860,
    VA = 336,
    VC = 670,
    VE = 862,
    VG = 92,
    VI = 850,
    VN = 704,
    VU = 548,
    WF = 876,
    WS = 882,
    YE = 887,
    YT = 175,
    ZA = 710,
    ZM = 894,
    ZW = 716,
}

const COUNTRY_CODE_SEARCH_TABLE : &'static [(&'static str, Country)] = &[
    ("",    Country::Unspecified),
    ("AD",  Country::AD),
    ("AE",  Country::AE),
    ("AF",  Country::AF),
    ("AG",  Country::AG),
    ("AI",  Country::AI),
    ("AL",  Country::AL),
    ("AM",  Country::AM),
    ("AO",  Country::AO),
    ("AQ",  Country::AQ),
    ("AR",  Country::AR),
    ("AS",  Country::AS),
    ("AT",  Country::AT),
    ("AU",  Country::AU),
    ("AW",  Country::AW),
    ("AX",  Country::AX),
    ("AZ",  Country::AZ),
    ("BA",  Country::BA),
    ("BB",  Country::BB),
    ("BD",  Country::BD),
    ("BE",  Country::BE),
    ("BF",  Country::BF),
    ("BG",  Country::BG),
    ("BH",  Country::BH),
    ("BI",  Country::BI),
    ("BJ",  Country::BJ),
    ("BL",  Country::BL),
    ("BM",  Country::BM),
    ("BN",  Country::BN),
    ("BO",  Country::BO),
    ("BQ",  Country::BQ),
    ("BR",  Country::BR),
    ("BS",  Country::BS),
    ("BT",  Country::BT),
    ("BV",  Country::BV),
    ("BW",  Country::BW),
    ("BY",  Country::BY),
    ("BZ",  Country::BZ),
    ("CA",  Country::CA),
    ("CC",  Country::CC),
    ("CD",  Country::CD),
    ("CF",  Country::CF),
    ("CG",  Country::CG),
    ("CH",  Country::CH),
    ("CI",  Country::CI),
    ("CK",  Country::CK),
    ("CL",  Country::CL),
    ("CM",  Country::CM),
    ("CN",  Country::CN),
    ("CO",  Country::CO),
    ("CR",  Country::CR),
    ("CU",  Country::CU),
    ("CV",  Country::CV),
    ("CW",  Country::CW),
    ("CX",  Country::CX),
    ("CY",  Country::CY),
    ("CZ",  Country::CZ),
    ("DE",  Country::DE),
    ("DJ",  Country::DJ),
    ("DK",  Country::DK),
    ("DM",  Country::DM),
    ("DO",  Country::DO),
    ("DZ",  Country::DZ),
    ("EC",  Country::EC),
    ("EE",  Country::EE),
    ("EG",  Country::EG),
    ("EH",  Country::EH),
    ("ER",  Country::ER),
    ("ES",  Country::ES),
    ("ET",  Country::ET),
    ("FI",  Country::FI),
    ("FJ",  Country::FJ),
    ("FK",  Country::FK),
    ("FM",  Country::FM),
    ("FO",  Country::FO),
    ("FR",  Country::FR),
    ("GA",  Country::GA),
    ("GB",  Country::GB),
    ("GD",  Country::GD),
    ("GE",  Country::GE),
    ("GF",  Country::GF),
    ("GG",  Country::GG),
    ("GH",  Country::GH),
    ("GI",  Country::GI),
    ("GL",  Country::GL),
    ("GM",  Country::GM),
    ("GN",  Country::GN),
    ("GP",  Country::GP),
    ("GQ",  Country::GQ),
    ("GR",  Country::GR),
    ("GS",  Country::GS),
    ("GT",  Country::GT),
    ("GU",  Country::GU),
    ("GW",  Country::GW),
    ("GY",  Country::GY),
    ("HK",  Country::HK),
    ("HM",  Country::HM),
    ("HN",  Country::HN),
    ("HR",  Country::HR),
    ("HT",  Country::HT),
    ("HU",  Country::HU),
    ("ID",  Country::ID),
    ("IE",  Country::IE),
    ("IL",  Country::IL),
    ("IM",  Country::IM),
    ("IN",  Country::IN),
    ("IO",  Country::IO),
    ("IQ",  Country::IQ),
    ("IR",  Country::IR),
    ("IS",  Country::IS),
    ("IT",  Country::IT),
    ("JE",  Country::JE),
    ("JM",  Country::JM),
    ("JO",  Country::JO),
    ("JP",  Country::JP),
    ("KE",  Country::KE),
    ("KG",  Country::KG),
    ("KH",  Country::KH),
    ("KI",  Country::KI),
    ("KM",  Country::KM),
    ("KN",  Country::KN),
    ("KP",  Country::KP),
    ("KR",  Country::KR),
    ("KW",  Country::KW),
    ("KY",  Country::KY),
    ("KZ",  Country::KZ),
    ("LA",  Country::LA),
    ("LB",  Country::LB),
    ("LC",  Country::LC),
    ("LI",  Country::LI),
    ("LK",  Country::LK),
    ("LR",  Country::LR),
    ("LS",  Country::LS),
    ("LT",  Country::LT),
    ("LU",  Country::LU),
    ("LV",  Country::LV),
    ("LY",  Country::LY),
    ("MA",  Country::MA),
    ("MC",  Country::MC),
    ("MD",  Country::MD),
    ("ME",  Country::ME),
    ("MF",  Country::MF),
    ("MG",  Country::MG),
    ("MH",  Country::MH),
    ("MK",  Country::MK),
    ("ML",  Country::ML),
    ("MM",  Country::MM),
    ("MN",  Country::MN),
    ("MO",  Country::MO),
    ("MP",  Country::MP),
    ("MQ",  Country::MQ),
    ("MR",  Country::MR),
    ("MS",  Country::MS),
    ("MT",  Country::MT),
    ("MU",  Country::MU),
    ("MV",  Country::MV),
    ("MW",  Country::MW),
    ("MX",  Country::MX),
    ("MY",  Country::MY),
    ("MZ",  Country::MZ),
    ("NA",  Country::NA),
    ("NC",  Country::NC),
    ("NE",  Country::NE),
    ("NF",  Country::NF),
    ("NG",  Country::NG),
    ("NI",  Country::NI),
    ("NL",  Country::NL),
    ("NO",  Country::NO),
    ("NP",  Country::NP),
    ("NR",  Country::NR),
    ("NU",  Country::NU),
    ("NZ",  Country::NZ),
    ("OM",  Country::OM),
    ("PA",  Country::PA),
    ("PE",  Country::PE),
    ("PF",  Country::PF),
    ("PG",  Country::PG),
    ("PH",  Country::PH),
    ("PK",  Country::PK),
    ("PL",  Country::PL),
    ("PM",  Country::PM),
    ("PN",  Country::PN),
    ("PR",  Country::PR),
    ("PS",  Country::PS),
    ("PT",  Country::PT),
    ("PW",  Country::PW),
    ("PY",  Country::PY),
    ("QA",  Country::QA),
    ("RE",  Country::RE),
    ("RO",  Country::RO),
    ("RS",  Country::RS),
    ("RU",  Country::RU),
    ("RW",  Country::RW),
    ("SA",  Country::SA),
    ("SB",  Country::SB),
    ("SC",  Country::SC),
    ("SD",  Country::SD),
    ("SE",  Country::SE),
    ("SG",  Country::SG),
    ("SH",  Country::SH),
    ("SI",  Country::SI),
    ("SJ",  Country::SJ),
    ("SK",  Country::SK),
    ("SL",  Country::SL),
    ("SM",  Country::SM),
    ("SN",  Country::SN),
    ("SO",  Country::SO),
    ("SR",  Country::SR),
    ("SS",  Country::SS),
    ("ST",  Country::ST),
    ("SV",  Country::SV),
    ("SX",  Country::SX),
    ("SY",  Country::SY),
    ("SZ",  Country::SZ),
    ("TC",  Country::TC),
    ("TD",  Country::TD),
    ("TF",  Country::TF),
    ("TG",  Country::TG),
    ("TH",  Country::TH),
    ("TJ",  Country::TJ),
    ("TK",  Country::TK),
    ("TL",  Country::TL),
    ("TM",  Country::TM),
    ("TN",  Country::TN),
    ("TO",  Country::TO),
    ("TR",  Country::TR),
    ("TT",  Country::TT),
    ("TV",  Country::TV),
    ("TW",  Country::TW),
    ("TZ",  Country::TZ),
    ("UA",  Country::UA),
    ("UG",  Country::UG),
    ("UM",  Country::UM),
    ("US",  Country::US),
    ("UY",  Country::UY),
    ("UZ",  Country::UZ),
    ("VA",  Country::VA),
    ("VC",  Country::VC),
    ("VE",  Country::VE),
    ("VG",  Country::VG),
    ("VI",  Country::VI),
    ("VN",  Country::VN),
    ("VU",  Country::VU),
    ("WF",  Country::WF),
    ("WS",  Country::WS),
    ("YE",  Country::YE),
    ("YT",  Country::YT),
    ("ZA",  Country::ZA),
    ("ZM",  Country::ZM),
    ("ZW",  Country::ZW),
];


#[cfg(test)]
mod tests {
    use super::Country;

    macro_rules! assert_s {
        ($expr:expr) => ({
            let c : Country = $expr.parse().unwrap();
            assert_eq!($expr, c.to_string());
        })
    }

    #[test]
    fn from_to_str() {
        assert_s!("ZW");
        assert_s!("PL");
        assert_s!("");
    }

    #[test]
    fn name() {
        assert_eq!("Poland", Country::PL.name());
        assert_eq!("", Country::Unspecified.name());
    }
}