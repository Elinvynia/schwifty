//! Countries supported by IBAN and helper methods.

use regex::Regex;
use std::fmt::{self, Display};
use std::str::FromStr;

/// These are the IBAN-supported countries.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Country {
    Albania,
    Andorra,
    Austria,
    Azerbaijan,
    Bahrain,
    Belarus,
    Belgium,
    BosniaHerzegovina,
    Brazil,
    Bulgaria,
    CostaRica,
    Croatia,
    Cyprus,
    CzechRepublic,
    Denmark,
    DominicanRepublic,
    EastTimor,
    Egypt,
    ElSalvador,
    Estonia,
    FaroeIslands,
    Finland,
    France,
    Georgia,
    Germany,
    Gibraltar,
    Greece,
    Greenland,
    Guatemala,
    Hungary,
    Iceland,
    Internet,
    Iraq,
    Ireland,
    Israel,
    Italy,
    Jordan,
    Kazakhstan,
    Kosovo,
    Kuwait,
    Latvia,
    Lebanon,
    Libya,
    Liechenstein,
    Lithuania,
    Luxembourg,
    NorthMacedonia,
    Malta,
    Mauritania,
    Mauritius,
    Monaco,
    Moldova,
    Montenegro,
    Netherlands,
    Norway,
    Pakistan,
    PalestinianTerritories,
    Poland,
    Portugal,
    Qatar,
    Romania,
    SaintLucia,
    SanMarino,
    SaoTomePrincipe,
    SaudiArabia,
    Serbia,
    Seychelles,
    Slovakia,
    Slovenia,
    Spain,
    Sweden,
    Switzerland,
    Tunisia,
    Turkey,
    Ukraine,
    UnitedArabEmirates,
    UnitedKingdom,
    VaticanCity,
    VirginIslands,
}

// Helper functions for the regex groups.
fn n(num: usize) -> String {
    format!("[0-9]{{{}}}", num)
}

fn c(num: usize) -> String {
    format!("[a-zA-Z0-9]{{{}}}", num)
}

fn a(num: usize) -> String {
    format!("[A-Z]{{{}}}", num)
}

// Almost-DSL macro for comfortable calling of above functions.
macro_rules! f {
    ($($arg:expr),+) => {
        {
            let mut s = String::with_capacity(34);
            $(s.push_str(&$arg);)*
            s
        }
    }
}

impl Country {
    pub(crate) fn length(&self) -> usize {
        use Country::*;
        match self {
            Albania => 28,
            Andorra => 24,
            Austria => 20,
            Azerbaijan => 28,
            Bahrain => 22,
            Belarus => 28,
            Belgium => 16,
            BosniaHerzegovina => 20,
            Brazil => 29,
            Bulgaria => 22,
            CostaRica => 22,
            Croatia => 21,
            Cyprus => 28,
            CzechRepublic => 24,
            Denmark => 18,
            DominicanRepublic => 28,
            EastTimor => 23,
            Egypt => 29,
            ElSalvador => 28,
            Estonia => 20,
            FaroeIslands => 18,
            Finland => 18,
            France => 27,
            Georgia => 22,
            Germany => 22,
            Gibraltar => 23,
            Greece => 27,
            Greenland => 18,
            Guatemala => 28,
            Hungary => 28,
            Iceland => 26,
            Internet => 16,
            Iraq => 23,
            Ireland => 22,
            Israel => 23,
            Italy => 27,
            Jordan => 30,
            Kazakhstan => 20,
            Kosovo => 20,
            Kuwait => 30,
            Latvia => 21,
            Lebanon => 28,
            Libya => 25,
            Liechenstein => 21,
            Lithuania => 20,
            Luxembourg => 20,
            NorthMacedonia => 19,
            Malta => 31,
            Mauritania => 27,
            Mauritius => 30,
            Monaco => 27,
            Moldova => 24,
            Montenegro => 22,
            Netherlands => 18,
            Norway => 15,
            Pakistan => 24,
            PalestinianTerritories => 29,
            Poland => 28,
            Portugal => 25,
            Qatar => 29,
            Romania => 24,
            SaintLucia => 32,
            SanMarino => 27,
            SaoTomePrincipe => 25,
            SaudiArabia => 24,
            Serbia => 22,
            Seychelles => 31,
            Slovakia => 24,
            Slovenia => 19,
            Spain => 24,
            Sweden => 24,
            Switzerland => 21,
            Tunisia => 24,
            Turkey => 26,
            Ukraine => 29,
            UnitedArabEmirates => 23,
            UnitedKingdom => 22,
            VaticanCity => 22,
            VirginIslands => 24,
        }
    }

    // To better understand this formatting, check here
    // https://en.wikipedia.org/wiki/International_Bank_Account_Number#IBAN_formats_by_country
    pub(crate) fn format(&self) -> Regex {
        use Country::*;
        let format = match self {
            Albania => f!(n(8), c(16)),
            Andorra => f!(n(8), c(12)),
            Austria => f!(n(16)),
            Azerbaijan => f!(c(4), n(20)),
            Bahrain => f!(a(4), c(14)),
            Belarus => f!(c(4), n(4), c(16)),
            Belgium => f!(n(12)),
            BosniaHerzegovina => f!(n(16)),
            Brazil => f!(n(23), a(1), c(1)),
            Bulgaria => f!(a(4), n(6), c(8)),
            CostaRica => f!(n(18)),
            Croatia => f!(n(17)),
            Cyprus => f!(n(8), c(16)),
            CzechRepublic => f!(n(20)),
            Denmark => f!(n(14)),
            DominicanRepublic => f!(a(4), n(20)),
            EastTimor => f!(n(19)),
            Egypt => f!(n(25)),
            ElSalvador => f!(a(4), n(20)),
            Estonia => f!(n(16)),
            FaroeIslands => f!(n(14)),
            Finland => f!(n(14)),
            France => f!(n(10), c(11), n(2)),
            Georgia => f!(c(2), n(16)),
            Germany => f!(n(18)),
            Gibraltar => f!(a(4), c(15)),
            Greece => f!(n(7), c(16)),
            Greenland => f!(n(14)),
            Guatemala => f!(c(4), c(20)),
            Hungary => f!(n(24)),
            Iceland => f!(n(22)),
            Internet => f!(n(4), c(8)),
            Iraq => f!(a(4), n(15)),
            Ireland => f!(c(4), n(14)),
            Israel => f!(n(19)),
            Italy => f!(a(1), n(10), c(12)),
            Jordan => f!(a(4), n(22)),
            Kazakhstan => f!(n(3), c(13)),
            Kosovo => f!(n(4), n(10), n(2)),
            Kuwait => f!(a(4), c(22)),
            Latvia => f!(a(4), c(13)),
            Lebanon => f!(n(4), c(20)),
            Libya => f!(n(21)),
            Liechenstein => f!(n(5), c(12)),
            Lithuania => f!(n(16)),
            Luxembourg => f!(n(3), c(13)),
            NorthMacedonia => f!(n(3), c(10), n(2)),
            Malta => f!(a(4), n(5), c(18)),
            Mauritania => f!(n(23)),
            Mauritius => f!(a(4), n(19), a(3)),
            Monaco => f!(n(10), c(11), n(2)),
            Moldova => f!(c(2), c(18)),
            Montenegro => f!(n(18)),
            Netherlands => f!(a(4), n(10)),
            Norway => f!(n(11)),
            Pakistan => f!(c(4), n(16)),
            PalestinianTerritories => f!(c(4), n(21)),
            Poland => f!(n(24)),
            Portugal => f!(n(21)),
            Qatar => f!(a(4), c(21)),
            Romania => f!(a(4), c(16)),
            SaintLucia => f!(a(4), c(24)),
            SanMarino => f!(a(1), n(10), c(12)),
            SaoTomePrincipe => f!(n(21)),
            SaudiArabia => f!(n(2), c(18)),
            Serbia => f!(n(18)),
            Seychelles => f!(a(4), n(20), a(3)),
            Slovakia => f!(n(20)),
            Slovenia => f!(n(15)),
            Spain => f!(n(20)),
            Sweden => f!(n(20)),
            Switzerland => f!(n(5), c(12)),
            Tunisia => f!(n(20)),
            Turkey => f!(n(5), c(17)),
            Ukraine => f!(n(6), c(19)),
            UnitedArabEmirates => f!(n(3), n(16)),
            UnitedKingdom => f!(a(4), n(14)),
            VaticanCity => f!(n(3), n(15)),
            VirginIslands => f!(c(4), n(16)),
        };

        Regex::new(&format).unwrap()
    }

    // Same reference as above.
    pub(crate) fn account_number(&self, input: &str) -> u128 {
        use Country::*;
        let end = input.len() - 1;
        let (start, stop) = match self {
            Albania | Andorra | Belarus | Cyprus | Egypt | Germany | Guatemala | Jordan
            | Poland | SaoTomePrincipe => (12, end),
            Azerbaijan
            | Bahrain
            | CostaRica
            | Denmark
            | DominicanRepublic
            | ElSalvador
            | Gibraltar
            | Greenland
            | Internet
            | Kosovo
            | Kuwait
            | Latvia
            | Lebanon
            | Netherlands
            | Pakistan
            | PalestinianTerritories
            | Qatar
            | Romania
            | SaintLucia
            | VirginIslands => (8, end),
            Austria | Liechenstein | Lithuania | Switzerland | UnitedArabEmirates => (9, end),
            Belgium | EastTimor | NorthMacedonia | Montenegro | Serbia => (7, end - 2),
            BosniaHerzegovina => (10, end - 2),
            Brazil => (17, end - 2),
            Bulgaria | CzechRepublic | Slovakia | Spain => (14, end),
            Croatia | Greece | Iraq => (11, end),
            FaroeIslands | Norway | Estonia => (8, end - 1),
            Finland => (10, end - 1),
            France | Monaco => (14, end - 2),
            Georgia | Moldova | SaudiArabia => (6, end),
            Hungary => (12, end - 1),
            Iceland => (10, 15),
            Ireland | Mauritania | UnitedKingdom => (14, end),
            Israel | Libya => (10, end),
            Italy | SanMarino => (15, end),
            Kazakhstan | Luxembourg | Sweden | VaticanCity => (7, end),
            Malta => (13, end),
            Mauritius => (12, end - 6),
            Portugal => (12, end - 2),
            Seychelles => (13, end - 3),
            Slovenia => (8, end - 2),
            Tunisia => (9, end - 2),
            Turkey | Ukraine => (10, end),
        };

        input[start..=stop].parse().unwrap()
    }

    // Same reference as above.
    pub(crate) fn bank_code(&self, input: &str) -> String {
        use Country::*;
        // b = part of bank code, x = other
        let (start, stop) = match self {
            // bbxx xxxx xxxx
            Estonia | Georgia | Iceland | Moldova | SaudiArabia | Slovenia | Tunisia => (4, 5),

            // bbbx xxxx xxxx
            Albania | Belgium | BosniaHerzegovina | Cyprus | EastTimor | Greece | Hungary
            | Israel | Kazakhstan | Libya | Luxembourg | NorthMacedonia | Montenegro | Poland
            | Serbia | Sweden | UnitedArabEmirates | VaticanCity => (4, 6),

            // bbbb xxxx xxxx
            Andorra
            | Azerbaijan
            | Bahrain
            | Belarus
            | Bulgaria
            | CzechRepublic
            | Denmark
            | DominicanRepublic
            | Egypt
            | ElSalvador
            | FaroeIslands
            | Gibraltar
            | Greenland
            | Guatemala
            | Internet
            | Iraq
            | Jordan
            | Kosovo
            | Kuwait
            | Latvia
            | Lebanon
            | Malta
            | Netherlands
            | Norway
            | Pakistan
            | PalestinianTerritories
            | Portugal
            | Qatar
            | Romania
            | SaintLucia
            | SaoTomePrincipe
            | Slovakia
            | Spain
            | UnitedKingdom
            | VirginIslands => (4, 7),

            // bbbb bxxx xxxx
            Austria | France | Liechenstein | Lithuania | Mauritania | Monaco | Switzerland
            | Turkey => (4, 8),

            // bbbb bbxx xxxx
            Finland | Mauritius | Seychelles | Ukraine => (4, 9),

            // bbbb bbbx xxxx
            Croatia => (4, 10),

            // bbbb bbbb xxxx
            Brazil | Germany => (4, 11),

            // xbbb bbbb bbbb
            CostaRica => (5, 7),

            // xxxx bbbb bbxx
            Ireland => (8, 13),

            // xbbb bbxx xxxx
            Italy | SanMarino => (5, 9),
        };

        input[start..=stop].to_string()
    }
}

impl FromStr for Country {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AL" => Ok(Country::Albania),
            "AD" => Ok(Country::Andorra),
            "AT" => Ok(Country::Austria),
            "AZ" => Ok(Country::Azerbaijan),
            "BH" => Ok(Country::Bahrain),
            "BY" => Ok(Country::Belarus),
            "BE" => Ok(Country::Belgium),
            "BA" => Ok(Country::BosniaHerzegovina),
            "BR" => Ok(Country::Brazil),
            "BG" => Ok(Country::Bulgaria),
            "CR" => Ok(Country::CostaRica),
            "HR" => Ok(Country::Croatia),
            "CY" => Ok(Country::Cyprus),
            "CZ" => Ok(Country::CzechRepublic),
            "DK" => Ok(Country::Denmark),
            "DO" => Ok(Country::DominicanRepublic),
            "TL" => Ok(Country::EastTimor),
            "EG" => Ok(Country::Egypt),
            "SV" => Ok(Country::ElSalvador),
            "EE" => Ok(Country::Estonia),
            "FO" => Ok(Country::FaroeIslands),
            "FI" => Ok(Country::Finland),
            "FR" => Ok(Country::France),
            "GE" => Ok(Country::Georgia),
            "DE" => Ok(Country::Germany),
            "GI" => Ok(Country::Gibraltar),
            "GR" => Ok(Country::Greece),
            "GL" => Ok(Country::Greenland),
            "GT" => Ok(Country::Guatemala),
            "HU" => Ok(Country::Hungary),
            "IS" => Ok(Country::Iceland),
            "IQ" => Ok(Country::Iraq),
            "AA" => Ok(Country::Internet),
            "IE" => Ok(Country::Ireland),
            "IL" => Ok(Country::Israel),
            "IT" => Ok(Country::Italy),
            "JO" => Ok(Country::Jordan),
            "KZ" => Ok(Country::Kazakhstan),
            "XK" => Ok(Country::Kosovo),
            "KW" => Ok(Country::Kuwait),
            "LV" => Ok(Country::Latvia),
            "LB" => Ok(Country::Lebanon),
            "LY" => Ok(Country::Libya),
            "LI" => Ok(Country::Liechenstein),
            "LT" => Ok(Country::Lithuania),
            "LU" => Ok(Country::Luxembourg),
            "MK" => Ok(Country::NorthMacedonia),
            "MT" => Ok(Country::Malta),
            "MR" => Ok(Country::Mauritania),
            "MU" => Ok(Country::Mauritius),
            "MC" => Ok(Country::Monaco),
            "MD" => Ok(Country::Moldova),
            "ME" => Ok(Country::Montenegro),
            "NL" => Ok(Country::Netherlands),
            "NO" => Ok(Country::Norway),
            "PK" => Ok(Country::Pakistan),
            "PS" => Ok(Country::PalestinianTerritories),
            "PL" => Ok(Country::Poland),
            "PT" => Ok(Country::Portugal),
            "QA" => Ok(Country::Qatar),
            "RO" => Ok(Country::Romania),
            "LC" => Ok(Country::SaintLucia),
            "SM" => Ok(Country::SanMarino),
            "ST" => Ok(Country::SaoTomePrincipe),
            "SA" => Ok(Country::SaudiArabia),
            "RS" => Ok(Country::Serbia),
            "SC" => Ok(Country::Seychelles),
            "SK" => Ok(Country::Slovakia),
            "SI" => Ok(Country::Slovenia),
            "ES" => Ok(Country::Spain),
            "SE" => Ok(Country::Sweden),
            "CH" => Ok(Country::Switzerland),
            "TN" => Ok(Country::Tunisia),
            "TR" => Ok(Country::Turkey),
            "UA" => Ok(Country::Ukraine),
            "AE" => Ok(Country::UnitedArabEmirates),
            "GB" => Ok(Country::UnitedKingdom),
            "VA" => Ok(Country::VaticanCity),
            "VG" => Ok(Country::VirginIslands),
            _ => Err(()),
        }
    }
}

impl Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Country::*;
        let code = match self {
            Albania => "AL",
            Andorra => "AD",
            Austria => "AT",
            Azerbaijan => "AZ",
            Bahrain => "BH",
            Belarus => "BY",
            Belgium => "BE",
            BosniaHerzegovina => "BA",
            Brazil => "BR",
            Bulgaria => "BG",
            CostaRica => "CR",
            Croatia => "HR",
            Cyprus => "CY",
            CzechRepublic => "CZ",
            Denmark => "DK",
            DominicanRepublic => "DO",
            EastTimor => "TL",
            Egypt => "EG",
            ElSalvador => "SV",
            Estonia => "EE",
            FaroeIslands => "FO",
            Finland => "FI",
            France => "FR",
            Georgia => "GE",
            Germany => "DE",
            Gibraltar => "GI",
            Greece => "GR",
            Greenland => "GL",
            Guatemala => "GT",
            Hungary => "HU",
            Iceland => "IS",
            Iraq => "IQ",
            Internet => "AA",
            Ireland => "IE",
            Israel => "IL",
            Italy => "IT",
            Jordan => "JO",
            Kazakhstan => "KZ",
            Kosovo => "XK",
            Kuwait => "KW",
            Latvia => "LV",
            Lebanon => "LB",
            Libya => "LY",
            Liechenstein => "LI",
            Lithuania => "LT",
            Luxembourg => "LU",
            NorthMacedonia => "MK",
            Malta => "MT",
            Mauritania => "MR",
            Mauritius => "MU",
            Monaco => "MC",
            Moldova => "MD",
            Montenegro => "ME",
            Netherlands => "NL",
            Norway => "NO",
            Pakistan => "PK",
            PalestinianTerritories => "PS",
            Poland => "PL",
            Portugal => "PT",
            Qatar => "QA",
            Romania => "RO",
            SaintLucia => "LC",
            SanMarino => "SM",
            SaoTomePrincipe => "ST",
            SaudiArabia => "SA",
            Serbia => "RS",
            Seychelles => "SC",
            Slovakia => "SK",
            Slovenia => "SI",
            Spain => "ES",
            Sweden => "SE",
            Switzerland => "CH",
            Tunisia => "TN",
            Turkey => "TR",
            Ukraine => "UA",
            UnitedArabEmirates => "AE",
            UnitedKingdom => "GB",
            VaticanCity => "VA",
            VirginIslands => "VG",
        };
        write!(f, "{}", code)
    }
}
