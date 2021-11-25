use schwifty::*;

#[test]
fn validate_iban_basic() {
    validate("AL47 2121 1009 0000 0002 3569 8741").unwrap(); // Albania
    validate("AD12 0001 2030 2003 5910 0100").unwrap(); // Andorra
    validate("AT61 1904 3002 3457 3201").unwrap(); // Austria
    validate("AZ21 NABZ 0000 0000 1370 1000 1944").unwrap(); // Republic of Azerbaijan
    validate("BH67 BMAG 0000 1299 1234 56").unwrap(); // Bahrain
    validate("BE68 5390 0754 7034").unwrap(); // Belgium
    validate("BA39 1290 0794 0102 8494").unwrap(); // Bosnia and Herzegovina
    validate("BR97 0036 0305 0000 1000 9795 493P 1").unwrap(); // Brazil
    validate("BR18 0000 0000 1414 5512 3924 100C 2").unwrap(); // Brazil
    validate("BG80 BNBG 9661 1020 3456 78").unwrap(); // Bulgaria
    validate("CR05 0152 0200 1026 2840 66").unwrap(); // Costa Rica
    validate("HR12 1001 0051 8630 0016 0").unwrap(); // Croatia
    validate("CY17 0020 0128 0000 0012 0052 7600").unwrap(); // Cyprus
    validate("CZ65 0800 0000 1920 0014 5399").unwrap(); // Czech Republic
    validate("CZ94 5500 0000 0010 1103 8930").unwrap(); // Czech Republic
    validate("DK50 0040 0440 1162 43").unwrap(); // Greenland
    validate("FO62 6460 0001 6316 34").unwrap(); // Faroer
    validate("GL89 6471 0001 0002 06").unwrap(); // Denmark
    validate("DO28 BAGR 0000 0001 2124 5361 1324").unwrap(); // Dominican Republic
    validate("EE38 2200 2210 2014 5685").unwrap(); // Estonia
    validate("FI21 1234 5600 0007 85").unwrap(); // Finland
    validate("FR14 2004 1010 0505 0001 3M02 606").unwrap(); // France
    validate("GE29 NB00 0000 0101 9049 17").unwrap(); // Georgia
    validate("DE89 3704 0044 0532 0130 00").unwrap(); // Germany
    validate("GI75 NWBK 0000 0000 7099 453").unwrap(); // Gibraltar
    validate("GR16 0110 1250 0000 0001 2300 695").unwrap(); // Greece
    validate("GT82 TRAJ 0102 0000 0012 1002 9690").unwrap(); // Guatemala
    validate("HU42 1177 3016 1111 1018 0000 0000").unwrap(); // Hungary
    validate("IS14 0159 2600 7654 5510 7303 39").unwrap(); // Iceland
    validate("IE29 AIBK 9311 5212 3456 78").unwrap(); // Ireland
    validate("IL62 0108 0000 0009 9999 999").unwrap(); // Israel
    validate("IT60 X054 2811 1010 0000 0123 456").unwrap(); // Italy
    validate("JO94 CBJO 0010 0000 0000 0131 0003 02").unwrap(); // Jordan
    validate("KZ86 125K ZT50 0410 0100").unwrap(); // Kazakhstan
    validate("XK05 1212 0123 4567 8906").unwrap(); // Republic of Kosovo
    validate("KW81 CBKU 0000 0000 0000 1234 5601 01").unwrap(); // Kuwait
    validate("LV80 BANK 0000 4351 9500 1").unwrap(); // Latvia
    validate("LB62 0999 0000 0001 0019 0122 9114").unwrap(); // Lebanon
    validate("LI21 0881 0000 2324 013A A").unwrap(); // Liechtenstein
    validate("LT12 1000 0111 0100 1000").unwrap(); // Lithuania
    validate("LU28 0019 4006 4475 0000").unwrap(); // Luxembourg
    validate("MK07 2501 2000 0058 984").unwrap(); // Macedonia
    validate("MT84 MALT 0110 0001 2345 MTLC AST0 01S").unwrap(); // Malta
    validate("MR13 0002 0001 0100 0012 3456 753").unwrap(); // Mauritania
    validate("MU17 BOMM 0101 1010 3030 0200 000M UR").unwrap(); // Mauritius
    validate("MD24 AG00 0225 1000 1310 4168").unwrap(); // Moldova
    validate("MC58 1122 2000 0101 2345 6789 030").unwrap(); // Monaco
    validate("ME25 5050 0001 2345 6789 51").unwrap(); // Montenegro
    validate("NL91 ABNA 0417 1643 00").unwrap(); // The Netherlands
    validate("NO93 8601 1117 947").unwrap(); // Norway
    validate("PK36 SCBL 0000 0011 2345 6702").unwrap(); // Pakistan
    validate("PS92 PALS 0000 0000 0400 1234 5670 2").unwrap(); // Palestine
    validate("PL61 1090 1014 0000 0712 1981 2874").unwrap(); // Poland
    validate("PT50 0002 0123 1234 5678 9015 4").unwrap(); // Portugal
    validate("QA58 DOHB 0000 1234 5678 90AB CDEF G").unwrap(); // Qatar
    validate("RO49 AAAA 1B31 0075 9384 0000").unwrap(); // Romania
    validate("LC55 HEMM 0001 0001 0012 0012 00023015").unwrap(); // Saint Lucia
    validate("SM86 U032 2509 8000 0000 0270 100").unwrap(); // San Marino
    validate("ST68 0001 0001 0051 8453 1011 2").unwrap(); // Sao Tome And Principe
    validate("SA03 8000 0000 6080 1016 7519").unwrap(); // Saudi Arabia
    validate("RS35 2600 0560 1001 6113 79").unwrap(); // Serbia
    validate("SC18 SSCB 1101 0000 0000 0000 1497 USD").unwrap(); // Seychelles
    validate("SK31 1200 0000 1987 4263 7541").unwrap(); // Slovak Republic
    validate("SI56 1910 0000 0123 438").unwrap(); // Slovenia
    validate("ES91 2100 0418 4502 0005 1332").unwrap(); // Spain
    validate("SE45 5000 0000 0583 9825 7466").unwrap(); // Sweden
    validate("CH93 0076 2011 6238 5295 7").unwrap(); // Switzerland
    validate("TL38 0080 0123 4567 8910 157").unwrap(); // Timor-Leste
    validate("TN59 1000 6035 1835 9847 8831").unwrap(); // Tunisia
    validate("TR33 0006 1005 1978 6457 8413 26").unwrap(); // Turkey
    validate("UA21 3996 2200 0002 6007 2335 6600 1").unwrap(); // Ukraine
    validate("AE07 0331 2345 6789 0123 456").unwrap(); // United Arab Emirates
    validate("GB29 NWBK 6016 1331 9268 19").unwrap(); // United Kingdom
    validate("VG96 VPVG 0000 0123 4567 8901").unwrap(); // Virgin Islands, British
    validate("BY13 NBRB 3600 9000 0000 2Z00 AB00").unwrap(); // Republic of Belarus
    validate("SV62 CENR 0000 0000 0000 0070 0025").unwrap(); // El Salvador
    validate("FO62 6460 0001 6316 34").unwrap(); // Faroe Islands
    validate("GL89 6471 0001 0002 06").unwrap(); // Grenland
    validate("IQ98 NBIQ 8501 2345 6789 012").unwrap(); // Iraq
    validate("AA11 0011 123Z 5678").unwrap(); // Internet
}

#[test]
fn validate_iban_empty() {
    assert!(matches!(validate(""), Err(ValidationError::InvalidCountryCode)));
    assert!(matches!(validate("F"), Err(ValidationError::InvalidCountryCode)));
    assert!(matches!(validate("FR"), Err(ValidationError::InvalidLength)));
}