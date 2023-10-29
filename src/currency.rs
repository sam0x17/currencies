use crate::amount::*;
use crate::u256::{U256, u64_to_u256};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum FormatStyle {
    /// Specifies that the symbol should prefix the amount with no space, like "$40.00". Common
    /// for major English-speaking locales.
    PrefixAttached,
    /// Specifies that the symbol should suffix the amount with no space, like  "40.00€".
    /// Commonly used in non-English locales.
    SuffixAttached,
    /// Specifies that the symbol should prefix the amount with a space, like "$ 40.00". Generally not used.
    PrefixSpaced,
    /// Specifies that the symbol should suffix the amount with a space, like "40.00 AUD".
    /// Commonly used by currencies that use a multi-character or alphabetic symbol.
    SuffixSpaced,
}

impl Default for FormatStyle {
    fn default() -> Self {
        Self::PrefixAttached
    }
}

/// Uniquely defines a particular currency, such as [`USD`], [`BTC`], or [`ETH`].
pub trait Currency: Copy + Clone + PartialEq + Eq + PartialOrd + Ord + core::hash::Hash {
    /// Represents the underlying (signed or un-signed) primitive integer type used to
    /// represent [`Amount`]s of this [`Currency`].
    type Backing: Backing;

    /// Determines the numerical base of an [`Amount`] of this [`Currency`].
    /// 
    /// For base ten currencies, this should be a `1` followed by a number of zeroes
    /// corresponding with the number of supported digits to the right of the decimal place.
    /// 
    /// Some very rare currencies use a base other than 10, such as Malagasy ariary. For these
    /// you should use an appropriate base.
    const BASE: Self::Backing;

    /// Specifies a 3-4 digit acronym or "code" that can be used as a short name for this
    /// [`Currency`]. For ISO-supported currencies, this will be equal to the ISO-4217
    /// alphabetic code, which can be found here: <https://en.wikipedia.org/wiki/ISO_4217>.
    ///
    /// For cryptocurrencies and other currencies not named in ISO-4217, this should be a short
    /// globally unique code that is specific to the currency, for example `BTC` for Bitcoin,
    /// `ETH` for Ethereum, etc..
    const CODE: &'static str;

    /// Specifies the monetary _symbol_, such as `$`, that is commonly associated with this
    /// [`Currency`].
    ///
    /// It is worth noting that such symbols can be multiple characters long, are not globally
    /// unique (actually many currencies use the `$` symbol and there are plenty examples of
    /// the same symbol being used for many currencies), and are not governed or defined by
    /// ISO-4217 or any other ISO. They also do not have to be symbols, they could be a word or
    /// several words long.
    ///
    /// The symbol is used when formatted values of an [`Amount`] using this [`Currency`] are
    /// displayed.
    const SYMBOL: &'static str;

    /// Specifies a long-hand / "proper" name for this [`Currency`], for example "United States
    /// Dollar".
    ///
    /// For currencies governed by ISO-4217, this corresponds with the "entity" field.
    const PROPER_NAME: &'static str;

    /// Specifies how an [`Amount`] of this [`Currency`] should be displayed when it is
    /// represented textually via [`core::fmt::Display`] and [`core::fmt::Debug`].
    const STYLE: FormatStyle;

    /// Set to `true` if this [`Currency`] is governed by ISO-4217. Otherwise `false`.
    const IS_ISO: bool;

    /// Set to `true` if this [`Currency`] is a cryptocurrency. Otherwise `false`.
    ///
    /// This is provided separately from `IS_ISO` to prepare for a future where one or more
    /// cryptocurrencies are included in ISO-4217.
    const IS_CRYPTO: bool;
}

macro_rules! define_currency {
    (
        $currency_name:ident, 
        $base_type:ty, 
        $base:expr, 
        $symbol:expr, 
        $proper_name:expr, 
        $style:ident,
        $is_iso:expr, 
        $is_crypto:expr
    ) => {
        #[doc = concat!($proper_name, " (", $symbol, ")")]
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $currency_name;

        impl $crate::currency::Currency for $currency_name {
            type Backing = $base_type;
            const BASE: Self::Backing = $base;
            const CODE: &'static str = stringify!($currency_name);
            const SYMBOL: &'static str = $symbol;
            const PROPER_NAME: &'static str = $proper_name;
            const STYLE: $crate::currency::FormatStyle = $crate::currency::FormatStyle::$style;
            const IS_ISO: bool = $is_iso;
            const IS_CRYPTO: bool = $is_crypto;
        }
    };
}


define_currency!(ETH, U256, u64_to_u256(1_000000000000000000), "ETH", "Ethereum", SuffixSpaced, false, true);
define_currency!(BTC, u64, 1_00000000, "BTC", "Bitcoin", SuffixSpaced, false, true);

define_currency!(USD, u64, 1_00, "$", "United States Dollar", PrefixAttached, true, false);
define_currency!(BAM, u64, 1_00, "KM", "Bosnia and Herzegovina Convertible Mark", SuffixSpaced, true, false);
define_currency!(AED, u64, 1_00, "Dh", "United Arab Emirates Dirham", SuffixSpaced, true, false);
define_currency!(AFN, u64, 1_00, "Af", "Afgan Afghani", SuffixSpaced, true, false);
define_currency!(ALL, u64, 1_00, "Lek", "Albanien Lek", SuffixSpaced, true, false);
define_currency!(AMD, u64, 1_00, "֏", "Armenian Dram", PrefixAttached, true, false);
define_currency!(ANG, u64, 1_00, "ƒ", "Netherlands Antillean Guilder", PrefixAttached, true, false);
define_currency!(AOA, u64, 1_00, "Kz", "Angolan Kwanza", SuffixSpaced, true, false);
define_currency!(ARS, u64, 1_00, "$", "Argentine Peso", SuffixSpaced, true, false);
define_currency!(AUD, u64, 1_00, "$", "Australian Dollar", SuffixSpaced, true, false);
define_currency!(AWG, u64, 1_00, "ƒ", "Aruban Florin", SuffixSpaced, true, false);
define_currency!(AZN, u64, 1_00, "₼", "Azerbaijani Manat", SuffixSpaced, true, false);
define_currency!(BBD, u64, 1_00, "$", "Barbados Dollar", SuffixSpaced, true, false);
define_currency!(BDT, u64, 1_00, "৳", "Bangladeshi Taka", SuffixSpaced, true, false);
define_currency!(BGN, u64, 1_00, "Lev", "Bulgarian Lev", SuffixSpaced, true, false);
define_currency!(BHD, u64, 1_000, "BD", "Bahraini Dinar", SuffixSpaced, true, false);
define_currency!(BIF, u64, 1_00, "Fr", "Burundian Franc", SuffixSpaced, true, false);
define_currency!(BMD, u64, 1_00, "$", "Berumdian Dollar", SuffixSpaced, true, false);
define_currency!(BND, u64, 1_00, "$", "Brunei Dollar", SuffixSpaced, true, false);
define_currency!(BOB, u64, 1_00, "Bs", "Boliviano", SuffixSpaced, true, false);
define_currency!(BOV, u64, 1_00, "BOV", "Bolivian Mvdol", SuffixSpaced, true, false);
define_currency!(BRL, u64, 1_00, "R$", "Brazilian Real", SuffixSpaced, true, false);
define_currency!(BSD, u64, 1_00, "$", "Bahamian Dollar", SuffixSpaced, true, false);
define_currency!(BTN, u64, 1_00, "Nu", "Bhutanese Ngultrum", SuffixSpaced, true, false);
define_currency!(BWP, u64, 1_00, "P", "Botswanna Pula", SuffixSpaced, true, false);
define_currency!(BYN, u64, 1_00, "Rbl", "Belarusian Ruble", SuffixSpaced, true, false);
define_currency!(BZD, u64, 1_00, "$", "Belize Dollar", SuffixSpaced, true, false);
define_currency!(CAD, u64, 1_00, "$", "Canadian Dollar", SuffixSpaced, true, false);
define_currency!(CDF, u64, 1_00, "Fr", "Congloese Franc", SuffixSpaced, true, false);
define_currency!(CHE, u64, 1_00, "CHE", "WIR Euro", SuffixSpaced, true, false);
define_currency!(CHF, u64, 1_00, "Fr", "Swiss Franc", SuffixSpaced, true, false);
define_currency!(CHW, u64, 1_00, "CHW", "WIR Franc", SuffixSpaced, true, false);
define_currency!(CLF, u64, 1_00, "CLF", "Unidad de Fomento", SuffixSpaced, true, false);
define_currency!(CLP, u64, 1_00, "$", "Chilean Peso", SuffixSpaced, true, false);
define_currency!(COP, u64, 1_00, "$", "Colombian Peso", SuffixSpaced, true, false);
define_currency!(COU, u64, 1_00, "COU", "Unidad de Valor Real (UVR)", SuffixSpaced, true, false);
define_currency!(CRC, u64, 1_00, "₡", "Costa Rican Colon", SuffixSpaced, true, false);
define_currency!(CUC, u64, 1_00, "CUC", "Cuban Convertible Peso", SuffixSpaced, true, false);
define_currency!(CUP, u64, 1_00, "$", "Cuban Peso", SuffixSpaced, true, false);
define_currency!(CVE, u64, 1_00, "$", "Cape Verdean Escudo", SuffixSpaced, true, false);
define_currency!(CZK, u64, 1_00, "Kč", "Czech Koruna", SuffixSpaced, true, false);
define_currency!(DJF, u64, 1_00, "Fr", "Dijiboutian Franc", SuffixSpaced, true, false);
define_currency!(DKK, u64, 1_00, "kr", "Danish Krone", SuffixSpaced, true, false);
define_currency!(DOP, u64, 1_00, "$", "Dominican Peso", SuffixSpaced, true, false);
define_currency!(DZD, u64, 1_00, "DA", "Algerian Dinar", SuffixSpaced, true, false);
define_currency!(EGP, u64, 1_00, "LE", "Egyptian Pound", SuffixSpaced, true, false);
define_currency!(ERN, u64, 1_00, "Nkf", "Eritrean Nakfa", SuffixSpaced, true, false);
define_currency!(ETB, u64, 1_00, "Br", "Ethiopian Birr", SuffixSpaced, true, false);
define_currency!(EUR, u64, 1_00, "€", "Euro", SuffixSpaced, true, false);
define_currency!(FJD, u64, 1_00, "$", "Fiji Dollar", SuffixSpaced, true, false);
define_currency!(FKP, u64, 1_00, "£", "Falkland Islands Pound", SuffixSpaced, true, false);
define_currency!(GBP, u64, 1_00, "£", "Pound Sterling", SuffixSpaced, true, false);
define_currency!(GEL, u64, 1_00, "₾", "Georgian Iari", SuffixSpaced, true, false);
define_currency!(GHS, u64, 1_00, "₵", "Ghanaian Cedi", SuffixSpaced, true, false);
define_currency!(GIP, u64, 1_00, "£", "Gibralter Pound", SuffixSpaced, true, false);
define_currency!(GMD, u64, 1_00, "D", "Gambian Dalasi", SuffixSpaced, true, false);
define_currency!(GNF, u64, 1_00, "Fr", "Guinean Franc", SuffixSpaced, true, false);
define_currency!(GTQ, u64, 1_00, "Q", "Guatemalan Quetzal", SuffixSpaced, true, false);
define_currency!(HKD, u64, 1_00, "$", "Hong Kong Dollar", SuffixSpaced, true, false);
define_currency!(HNL, u64, 1_00, "L", "Honduran Lempira", SuffixSpaced, true, false);
define_currency!(HTG, u64, 1_00, "G", "Haitian Gourde", SuffixSpaced, true, false);
define_currency!(HUF, u64, 1_00, "Ft", "Hungarian Forint", SuffixSpaced, true, false);
define_currency!(IDR, u64, 1_00, "Rp", "Indonesian Rupiah", SuffixSpaced, true, false);
define_currency!(ILS, u64, 1_00, "₪", "Israeli New Shekel", SuffixSpaced, true, false);
define_currency!(INR, u64, 1_00, "₹", "Indian Rupee", SuffixSpaced, true, false);
define_currency!(IQD, u64, 1_000, "ID", "Iraqi Dinar", SuffixSpaced, true, false);
define_currency!(IRR, u64, 1, "Rl", "Iranian Rial", SuffixSpaced, true, false);
define_currency!(ISK, u64, 1_00, "kr", "Icelandic Króna", SuffixSpaced, true, false);
define_currency!(JMD, u64, 1_00, "$", "Jamaican Dollar", SuffixSpaced, true, false);
define_currency!(JOD, u64, 1_00, "JD", "Jordanian Dinar", SuffixSpaced, true, false);
define_currency!(JPY, u64, 1_00, "¥", "Japanese Yen", SuffixSpaced, true, false);
define_currency!(KES, u64, 1_00, "Sh", "Kenyan Shilling", SuffixSpaced, true, false);
define_currency!(KGS, u64, 1_00, "som", "Kyrgyzstani Som", SuffixSpaced, true, false);
define_currency!(KHR, u64, 1_00, "CR", "Cambodian Riel", SuffixSpaced, true, false);
define_currency!(KMF, u64, 1_00, "Fr", "Comoro Franc", SuffixSpaced, true, false);
define_currency!(KPW, u64, 1_00, "₩", "North Korean Won", SuffixSpaced, true, false);
define_currency!(KRW, u64, 1_00, "₩", "South Korean Won", SuffixSpaced, true, false);
define_currency!(KWD, u64, 1_000, "KD", "Kuwaiti Dinar", SuffixSpaced, true, false);
define_currency!(KYD, u64, 1_00, "$", "Caymen Islands Dollar", SuffixSpaced, true, false);
define_currency!(KZT, u64, 1_00, "₸", "Kazakhstani Tenge", SuffixSpaced, true, false);
define_currency!(LAK, u64, 1_00, "₭", "Lao Kip", SuffixSpaced, true, false);
define_currency!(LBP, u64, 1_00, "LL", "Lebanese Pound", SuffixSpaced, true, false);
define_currency!(LKR, u64, 1_00, "Re", "Sri Lankan Rupee", SuffixSpaced, true, false);
define_currency!(LRD, u64, 1_00, "$", "Liberian Dollar", SuffixSpaced, true, false);
define_currency!(LSL, u64, 1_00, "L", "Lesotho Loti", SuffixSpaced, true, false);
define_currency!(LYD, u64, 1_000, "LD", "Libyan Dinar", SuffixSpaced, true, false);
define_currency!(MAD, u64, 1_00, "DH", "Moroccan Dirham", SuffixSpaced, true, false);
define_currency!(MDL, u64, 1_00, "Leu", "Moldovan Leu", SuffixSpaced, true, false);
define_currency!(MGA, u64, 5, "Ar", "Malagasy Ariary", SuffixSpaced, true, false);
define_currency!(MKD, u64, 1_00, "DEN", "Macedonian Denar", SuffixSpaced, true, false);
define_currency!(MMK, u64, 1_00, "K", "Myanmar Kyat", SuffixSpaced, true, false);
define_currency!(MNT, u64, 1_00, "₮", "Mongolian Tögrög", SuffixSpaced, true, false);
define_currency!(MOP, u64, 1_00, "MOP$", "Macanese Pataca", SuffixSpaced, true, false);
define_currency!(MRU, u64, 5, "UM", "Mauritanian Ouguiya", SuffixSpaced, true, false);
define_currency!(MUR, u64, 1_00, "Re", "Mauritian Rupee", SuffixSpaced, true, false);
define_currency!(MVR, u64, 1_00, "Rf", "Maldivian Rufiyaa", SuffixSpaced, true, false);
define_currency!(MWK, u64, 1_00, "K", "Malawian Kwacha", SuffixSpaced, true, false);
define_currency!(MXN, u64, 1_00, "$", "Mexican Peso", SuffixSpaced, true, false);
define_currency!(MXV, u64, 1_00, "MXV", "Mexican Unidad de Inversion (UDI)", SuffixSpaced, true, false);
define_currency!(MYR, u64, 1_00, "RM", "Malaysian Ringgit", SuffixSpaced, true, false);
define_currency!(MZN, u64, 1_00, "Mt", "Mozambican Metical", SuffixSpaced, true, false);
define_currency!(NAD, u64, 1_00, "$", "Namibian Dollar", SuffixSpaced, true, false);
define_currency!(NGN, u64, 1_00, "₦", "Nigerian Naira", SuffixSpaced, true, false);
define_currency!(NIO, u64, 1_00, "C$", "Nicaraguan Córdoba", SuffixSpaced, true, false);
define_currency!(NOK, u64, 1_00, "kr", "Norwegian Krone", SuffixSpaced, true, false);
define_currency!(NPR, u64, 1_00, "Re", "Nepalese Rupee", SuffixSpaced, true, false);
define_currency!(NZD, u64, 1_00, "$", "New Zealand Dollar", SuffixSpaced, true, false);
define_currency!(OMR, u64, 1_000, "RO", "Omani Rial", SuffixSpaced, true, false);
define_currency!(PAB, u64, 1_00, "B/", "Panamanian Balboa", SuffixSpaced, true, false);
define_currency!(PEN, u64, 1_00, "S/", "Peruvian Sol", SuffixSpaced, true, false);
define_currency!(PGK, u64, 1_00, "K", "Papua New Guinean Kina", SuffixSpaced, true, false);
define_currency!(PHP, u64, 1_00, "₱", "Philippine Peso", SuffixSpaced, true, false);
define_currency!(PKR, u64, 1_00, "Re", "Pakistani Rupee", SuffixSpaced, true, false);
define_currency!(PLN, u64, 1_00, "zł", "Polish Złoty", SuffixSpaced, true, false);
define_currency!(PYG, u64, 1_00, "₲", "Paraguayan Guarani", SuffixSpaced, true, false);
define_currency!(QAR, u64, 1_00, "QR", "Qatari Riyal", SuffixSpaced, true, false);
define_currency!(RON, u64, 1_00, "Leu", "Romanian Leu", SuffixSpaced, true, false);
define_currency!(RSD, u64, 1_00, "DIN", "Serbian Dinar", SuffixSpaced, true, false);
define_currency!(CNY, u64, 1_0, "¥", "Renminbi", SuffixSpaced, true, false);
define_currency!(RUB, u64, 1_00, "₽", "Russian Ruble", SuffixSpaced, true, false);
define_currency!(RWF, u64, 1_00, "Fr", "Rwandan Franc", SuffixSpaced, true, false);
define_currency!(SAR, u64, 1_00, "Rl", "Saudi Riyal", SuffixSpaced, true, false);
define_currency!(SBD, u64, 1_00, "$", "Solomon Islands Dollar", SuffixSpaced, true, false);
define_currency!(SCR, u64, 1_00, "Re", "Seychelles Rupee", SuffixSpaced, true, false);
define_currency!(SDG, u64, 1_00, "LS", "Sudanese Pound", SuffixSpaced, true, false);
define_currency!(SEK, u64, 1_00, "kr", "Swedish Krona", SuffixSpaced, true, false);
define_currency!(SGD, u64, 1_00, "$", "Singapore Dollar", SuffixSpaced, true, false);
define_currency!(SHP, u64, 1_00, "£", "Saint Helena Pound", SuffixSpaced, true, false);
define_currency!(SLE, u64, 1_00, "Le", "Sierra Leonean Leone", SuffixSpaced, true, false);
define_currency!(SOS, u64, 1_00, "Sh", "Somali Shilling", SuffixSpaced, true, false);
define_currency!(SRD, u64, 1_00, "$", "Surinamese Dollar", SuffixSpaced, true, false);
define_currency!(SSP, u64, 1_00, "SSP", "South Sudanese Pound", SuffixSpaced, true, false);
define_currency!(STN, u64, 1_00, "Db", "São Tomé and Príncipe Dobra", SuffixSpaced, true, false);
define_currency!(SYP, u64, 1_00, "LS", "Syrian Pound", SuffixSpaced, true, false);
define_currency!(SZL, u64, 1_00, "L", "Swazi Lilangeni", SuffixSpaced, true, false);
define_currency!(THB, u64, 1_00, "฿", "Thai Baht", SuffixSpaced, true, false);
define_currency!(TJS, u64, 1_00, "SM", "Tajikistani Somoni", SuffixSpaced, true, false);
define_currency!(TMT, u64, 1_00, "m", "Turkmenistan Manat", SuffixSpaced, true, false);
define_currency!(TND, u64, 1_000, "DT", "Tunisian Dinar", SuffixSpaced, true, false);
define_currency!(TOP, u64, 1_00, "T$", "Tongan Paʻanga", SuffixSpaced, true, false);
define_currency!(TRY, u64, 1_00, "₺", "Turkish Lira", SuffixSpaced, true, false);
define_currency!(TTD, u64, 1_00, "$", "Trinidad and Tobago Dollar", SuffixSpaced, true, false);
define_currency!(TWD, u64, 1_00, "$", "New Taiwan Dollar", SuffixSpaced, true, false);
define_currency!(TZS, u64, 1_00, "Sh", "Tanzanian Shilling", SuffixSpaced, true, false);
define_currency!(UAH, u64, 1_00, "₴", "Ukrainian Hryvnia", SuffixSpaced, true, false);
define_currency!(UGX, u64, 1, "Sh", "Ugandan Shilling", SuffixSpaced, true, false);
define_currency!(UYU, u64, 1_00, "$", "Uruguayan Peso", SuffixSpaced, true, false);
define_currency!(UZS, u64, 1_00, "soum", "Uzbekistan Sum", SuffixSpaced, true, false);
define_currency!(VED, u64, 1_00, "Bs.D", "Venezuelan Digital Bolívar", SuffixSpaced, true, false);
define_currency!(VES, u64, 1_00, "Bs.S", "Venezuelan Sovereign Bolívar", SuffixSpaced, true, false);
define_currency!(VND, u64, 1_0, "₫", "Vietnamese đồng", SuffixSpaced, true, false);
define_currency!(VUV, u64, 1_00, "VT", "Vanuatu Vatu", SuffixSpaced, true, false);
define_currency!(WST, u64, 1_00, "$", "Samoan Tālā", SuffixSpaced, true, false);
define_currency!(XAF, u64, 1_00, "Fr", "Central African CFA Franc", SuffixSpaced, true, false);
define_currency!(XAG, u64, 1_00, "t oz", "Silver (Troy Ounce)", SuffixSpaced, true, false);
define_currency!(XAU, u64, 1_00, "t oz", "Gold (Troy Ounce", SuffixSpaced, true, false);
define_currency!(XCD, u64, 1_00, "$", "East Caribbean Dollar", SuffixSpaced, true, false);
define_currency!(XOF, u64, 1_00, "Fr", "West African CFA Franc", SuffixSpaced, true, false);
define_currency!(XPD, u64, 1_00, "t oz", "Palladium (Troy Ounce)", SuffixSpaced, true, false);
define_currency!(XPF, u64, 1_00, "Fr", "CFP Franc", SuffixSpaced, true, false);
define_currency!(XPT, u64, 1_00, "t oz", "Platinum (Troy Ounce)", SuffixSpaced, true, false);
define_currency!(YER, u64, 1_00, "Rl", "Yemeni Rial", SuffixSpaced, true, false);
define_currency!(ZAR, u64, 1_00, "R", "South African Rand", SuffixSpaced, true, false);
define_currency!(ZMW, u64, 1_00, "K", "Zambian Kwacha", SuffixSpaced, true, false);
