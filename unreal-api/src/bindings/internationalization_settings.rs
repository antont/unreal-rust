#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UInternationalizationSettingsModel {
    __padding_end: [u8; 56],
}
impl UInternationalizationSettingsModel {}
#[repr(transparent)]
pub struct ETimezoneSetting(pub u8);
impl ETimezoneSetting {
    pub const INTERNATIONAL_DATE_LINE_WEST: ETimezoneSetting = ETimezoneSetting(0);
    pub const COORDINATED_UNIVERSAL_TIME_NEG11: ETimezoneSetting = ETimezoneSetting(1);
    pub const SAMOA: ETimezoneSetting = ETimezoneSetting(2);
    pub const HAWAII: ETimezoneSetting = ETimezoneSetting(3);
    pub const ALASKA: ETimezoneSetting = ETimezoneSetting(4);
    pub const PACIFIC_TIME_USCAN: ETimezoneSetting = ETimezoneSetting(5);
    pub const BAJA_CALIFORNIA: ETimezoneSetting = ETimezoneSetting(6);
    pub const MOUNTAIN_TIME_USCAN: ETimezoneSetting = ETimezoneSetting(7);
    pub const CHIHUAHUA_LA_PAZ_MAZATLAN: ETimezoneSetting = ETimezoneSetting(8);
    pub const ARIZONA: ETimezoneSetting = ETimezoneSetting(9);
    pub const SASKATCHEWAN: ETimezoneSetting = ETimezoneSetting(10);
    pub const CENTRAL_AMERICA: ETimezoneSetting = ETimezoneSetting(11);
    pub const CENTRAL_TIME_USCAN: ETimezoneSetting = ETimezoneSetting(12);
    pub const GUADALAJARA_MEXICO_CITY_MONTERREY: ETimezoneSetting = ETimezoneSetting(13);
    pub const EASTERN_TIME_USCAN: ETimezoneSetting = ETimezoneSetting(14);
    pub const BOGOTA_LIMA_QUITO: ETimezoneSetting = ETimezoneSetting(15);
    pub const INDIANA_US: ETimezoneSetting = ETimezoneSetting(16);
    pub const CARACAS: ETimezoneSetting = ETimezoneSetting(17);
    pub const ATLANTIC_TIME_CANADA: ETimezoneSetting = ETimezoneSetting(18);
    pub const CUIABA: ETimezoneSetting = ETimezoneSetting(19);
    pub const SANTIAGO: ETimezoneSetting = ETimezoneSetting(20);
    pub const GEORGETOWN_LA_PAZ_MANAUS_SAN_JUAN: ETimezoneSetting = ETimezoneSetting(21);
    pub const ASUNCION: ETimezoneSetting = ETimezoneSetting(22);
    pub const NEWFOUNDLAND: ETimezoneSetting = ETimezoneSetting(23);
    pub const BRASILIA: ETimezoneSetting = ETimezoneSetting(24);
    pub const GREENLAND: ETimezoneSetting = ETimezoneSetting(25);
    pub const MONTEVIDEO: ETimezoneSetting = ETimezoneSetting(26);
    pub const CAYENNE_FORTALEZA: ETimezoneSetting = ETimezoneSetting(27);
    pub const BUENOS_AIRES: ETimezoneSetting = ETimezoneSetting(28);
    pub const MID_ATLANTIC: ETimezoneSetting = ETimezoneSetting(29);
    pub const COORDINATED_UNIVERSAL_TIME_NEG02: ETimezoneSetting = ETimezoneSetting(30);
    pub const AZORES: ETimezoneSetting = ETimezoneSetting(31);
    pub const CABO_VERDE_IS: ETimezoneSetting = ETimezoneSetting(32);
    pub const DUBLIN_EDINBURGH_LISBON_LONDON: ETimezoneSetting = ETimezoneSetting(33);
    pub const MONROVIA_REYKJAVIK: ETimezoneSetting = ETimezoneSetting(34);
    pub const CASABLANCA: ETimezoneSetting = ETimezoneSetting(35);
    pub const UTC: ETimezoneSetting = ETimezoneSetting(36);
    pub const BELGRADE_BRATISLAVA_BUDAPEST_LJUBLJANA_PRAGUE: ETimezoneSetting = ETimezoneSetting(
        37,
    );
    pub const SARAJEVO_SKOPJE_WARSAW_ZAGREB: ETimezoneSetting = ETimezoneSetting(38);
    pub const BRUSSELS_COPENHAGEN_MADRID_PARIS: ETimezoneSetting = ETimezoneSetting(39);
    pub const WEST_CENTRAL_AFRICA: ETimezoneSetting = ETimezoneSetting(40);
    pub const AMSTERDAM_BERLIN_BERN_ROME_STOCKHOLM_VIENNA: ETimezoneSetting = ETimezoneSetting(
        41,
    );
    pub const WINDHOEK: ETimezoneSetting = ETimezoneSetting(42);
    pub const MINSK: ETimezoneSetting = ETimezoneSetting(43);
    pub const CAIRO: ETimezoneSetting = ETimezoneSetting(44);
    pub const HELSINKI_KYIV_RIGA_SOFIA_TALLINN_VILNIUS: ETimezoneSetting = ETimezoneSetting(
        45,
    );
    pub const ATHENS_BUCHAREST: ETimezoneSetting = ETimezoneSetting(46);
    pub const JERUSALEM: ETimezoneSetting = ETimezoneSetting(47);
    pub const AMMAN: ETimezoneSetting = ETimezoneSetting(48);
    pub const BEIRUT: ETimezoneSetting = ETimezoneSetting(49);
    pub const HARARE_PRETORIA: ETimezoneSetting = ETimezoneSetting(50);
    pub const DAMASCUS: ETimezoneSetting = ETimezoneSetting(51);
    pub const ISTANBUL: ETimezoneSetting = ETimezoneSetting(52);
    pub const KUWAIT_RIYADH: ETimezoneSetting = ETimezoneSetting(53);
    pub const BAGHDAD: ETimezoneSetting = ETimezoneSetting(54);
    pub const NAIROBI: ETimezoneSetting = ETimezoneSetting(55);
    pub const KALININGRAD: ETimezoneSetting = ETimezoneSetting(56);
    pub const TEHRAN: ETimezoneSetting = ETimezoneSetting(57);
    pub const MOSCOW_ST_PETERSBURG_VOLGOGRAD: ETimezoneSetting = ETimezoneSetting(58);
    pub const ABU_DHABI_MUSCAT: ETimezoneSetting = ETimezoneSetting(59);
    pub const BAKU: ETimezoneSetting = ETimezoneSetting(60);
    pub const YEREVAN: ETimezoneSetting = ETimezoneSetting(61);
    pub const TBILISI: ETimezoneSetting = ETimezoneSetting(62);
    pub const PORT_LOUIS: ETimezoneSetting = ETimezoneSetting(63);
    pub const KABUL: ETimezoneSetting = ETimezoneSetting(64);
    pub const TASHKENT: ETimezoneSetting = ETimezoneSetting(65);
    pub const ISLAMABAD_KARACHI: ETimezoneSetting = ETimezoneSetting(66);
    pub const CHENNAI_KOLKATA_MUMBAI_NEW_DELHI: ETimezoneSetting = ETimezoneSetting(67);
    pub const SRI_JAYAWARDENEPURA: ETimezoneSetting = ETimezoneSetting(68);
    pub const KATHMANDU: ETimezoneSetting = ETimezoneSetting(69);
    pub const EKATERINBURG: ETimezoneSetting = ETimezoneSetting(70);
    pub const ASTANA: ETimezoneSetting = ETimezoneSetting(71);
    pub const DHAKA: ETimezoneSetting = ETimezoneSetting(72);
    pub const YANGON_RANGOON: ETimezoneSetting = ETimezoneSetting(73);
    pub const BANGKOK_HANOI_JAKARTA: ETimezoneSetting = ETimezoneSetting(74);
    pub const KRASNOYARSK: ETimezoneSetting = ETimezoneSetting(75);
    pub const NOVOSIBIRSK: ETimezoneSetting = ETimezoneSetting(76);
    pub const BEIJING_CHONGQING_HONG_KONG_URUMQI: ETimezoneSetting = ETimezoneSetting(
        77,
    );
    pub const KUALA_LUMPUR_SINGAPORE: ETimezoneSetting = ETimezoneSetting(78);
    pub const TAIPEI: ETimezoneSetting = ETimezoneSetting(79);
    pub const PERTH: ETimezoneSetting = ETimezoneSetting(80);
    pub const ULAANBAATAR: ETimezoneSetting = ETimezoneSetting(81);
    pub const IRKUTSK: ETimezoneSetting = ETimezoneSetting(82);
    pub const SEOUL: ETimezoneSetting = ETimezoneSetting(83);
    pub const OSAKA_SAPPORO_TOKYO: ETimezoneSetting = ETimezoneSetting(84);
    pub const DARWIN: ETimezoneSetting = ETimezoneSetting(85);
    pub const ADELAIDE: ETimezoneSetting = ETimezoneSetting(86);
    pub const YAKUTSK: ETimezoneSetting = ETimezoneSetting(87);
    pub const CANBERRA_MELBOURNE_SYDNEY: ETimezoneSetting = ETimezoneSetting(88);
    pub const BRISBANE: ETimezoneSetting = ETimezoneSetting(89);
    pub const HOBART: ETimezoneSetting = ETimezoneSetting(90);
    pub const GUAM_PORT_MORESBY: ETimezoneSetting = ETimezoneSetting(91);
    pub const VLADIVOSTOK: ETimezoneSetting = ETimezoneSetting(92);
    pub const SOLOMON_IS_NEW_CALEDONIA: ETimezoneSetting = ETimezoneSetting(93);
    pub const MAGADAN: ETimezoneSetting = ETimezoneSetting(94);
    pub const FIJI: ETimezoneSetting = ETimezoneSetting(95);
    pub const AUCKLAND_WELLINGTON: ETimezoneSetting = ETimezoneSetting(96);
    pub const COORDINATED_UNIVERSAL_TIME12: ETimezoneSetting = ETimezoneSetting(97);
    pub const NUKUALOFA: ETimezoneSetting = ETimezoneSetting(98);
    pub const LOCAL_TIME: ETimezoneSetting = ETimezoneSetting(99);
}
