use currencies_core::{
    currency::*,
    safety::{Checked, Unchecked},
    Currency, ParsedAmount,
};
use derive_syn_parse::Parse;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse2, Error, Ident, LitStr, Result, Token};

#[derive(Parse)]
struct AmountInput {
    currency: Ident,
    _comma: Token![,],
    amount: LitStr,
}

#[proc_macro]
pub fn amt(input: TokenStream) -> TokenStream {
    match amt_internal::<false>(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn amt_checked(input: TokenStream) -> TokenStream {
    match amt_internal::<true>(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn filter_error(err: impl ToString) -> String {
    format!("invalid amount: {}", err.to_string().trim_start_matches("error: "))
}

fn parse_amount<C: Currency, const SAFE: bool>(amount: &LitStr, currency_ident: &Ident) -> Result<TokenStream2> {
    let krate = quote!(::currencies);
    if SAFE {
        let amount: ParsedAmount<C, Checked> = amount.value().parse().map_err(|err| Error::new(amount.span(), filter_error(err)))?;
        let amount = amount.amount;
        let amount: TokenStream2 = format!("{:?}", amount.raw_backing()).parse().unwrap();
        Ok(quote! {
            #krate::Amount::<#currency_ident, #krate::safety::Checked>::from_raw(#amount)
        })
    } else {
        let amount: ParsedAmount<C, Unchecked> = amount.value().parse().map_err(|err| Error::new(amount.span(), filter_error(err)))?;
        let amount = amount.amount;
        let amount: TokenStream2 = format!("{:?}", amount.raw_backing()).parse().unwrap();
        Ok(quote! {
            #krate::Amount::<#currency_ident, #krate::safety::Unchecked>::from_raw(#amount)
        })
    }
}

fn amt_internal<const SAFE: bool>(tokens: impl Into<TokenStream2>) -> Result<TokenStream2> {
    let input = parse2::<AmountInput>(tokens.into())?;
    let currency = input.currency;
    let amount = input.amount;
    let output = match currency.to_string().to_uppercase().as_str() {
        "USDC" => parse_amount::<USDC, SAFE>(&amount, &currency)?,
        "BTC" => parse_amount::<BTC, SAFE>(&amount, &currency)?,
        "ETH" => parse_amount::<ETH, SAFE>(&amount, &currency)?,
        "SOL" => parse_amount::<SOL, SAFE>(&amount, &currency)?,
        "ALGO" => parse_amount::<SOL, SAFE>(&amount, &currency)?,
        "ORCA" => parse_amount::<SOL, SAFE>(&amount, &currency)?,
        "AVAX" => parse_amount::<SOL, SAFE>(&amount, &currency)?,
        "ZEC" => parse_amount::<SOL, SAFE>(&amount, &currency)?,
        "XMR" => parse_amount::<SOL, SAFE>(&amount, &currency)?,
        "DOGE" => parse_amount::<SOL, SAFE>(&amount, &currency)?,
        "LTC" => parse_amount::<SOL, SAFE>(&amount, &currency)?,
        "MATIC" => parse_amount::<SOL, SAFE>(&amount, &currency)?,
        "XLM" => parse_amount::<SOL, SAFE>(&amount, &currency)?,
        "TAO" => parse_amount::<SOL, SAFE>(&amount, &currency)?,
        "NEAR" => parse_amount::<SOL, SAFE>(&amount, &currency)?,
        "AAVE" => parse_amount::<AAVE, SAFE>(&amount, &currency)?,
        "ADA" => parse_amount::<ADA, SAFE>(&amount, &currency)?,
        "BOOK" => parse_amount::<BOOK, SAFE>(&amount, &currency)?,
        "DOT" => parse_amount::<DOT, SAFE>(&amount, &currency)?,
        "KSM" => parse_amount::<KSM, SAFE>(&amount, &currency)?,
        "USD" => parse_amount::<USD, SAFE>(&amount, &currency)?,
        "BAM" => parse_amount::<BAM, SAFE>(&amount, &currency)?,
        "AED" => parse_amount::<AED, SAFE>(&amount, &currency)?,
        "AFN" => parse_amount::<AFN, SAFE>(&amount, &currency)?,
        "ALL" => parse_amount::<ALL, SAFE>(&amount, &currency)?,
        "AMD" => parse_amount::<AMD, SAFE>(&amount, &currency)?,
        "ANG" => parse_amount::<ANG, SAFE>(&amount, &currency)?,
        "AOA" => parse_amount::<AOA, SAFE>(&amount, &currency)?,
        "ARS" => parse_amount::<ARS, SAFE>(&amount, &currency)?,
        "AUD" => parse_amount::<AUD, SAFE>(&amount, &currency)?,
        "AWG" => parse_amount::<AWG, SAFE>(&amount, &currency)?,
        "AZN" => parse_amount::<AZN, SAFE>(&amount, &currency)?,
        "BBD" => parse_amount::<BBD, SAFE>(&amount, &currency)?,
        "BDT" => parse_amount::<BDT, SAFE>(&amount, &currency)?,
        "BGN" => parse_amount::<BGN, SAFE>(&amount, &currency)?,
        "BHD" => parse_amount::<BHD, SAFE>(&amount, &currency)?,
        "BIF" => parse_amount::<BIF, SAFE>(&amount, &currency)?,
        "BMD" => parse_amount::<BMD, SAFE>(&amount, &currency)?,
        "BND" => parse_amount::<BND, SAFE>(&amount, &currency)?,
        "BOB" => parse_amount::<BOB, SAFE>(&amount, &currency)?,
        "BOV" => parse_amount::<BOV, SAFE>(&amount, &currency)?,
        "BRL" => parse_amount::<BRL, SAFE>(&amount, &currency)?,
        "BSD" => parse_amount::<BSD, SAFE>(&amount, &currency)?,
        "BTN" => parse_amount::<BTN, SAFE>(&amount, &currency)?,
        "BWP" => parse_amount::<BWP, SAFE>(&amount, &currency)?,
        "BYN" => parse_amount::<BYN, SAFE>(&amount, &currency)?,
        "BZD" => parse_amount::<BZD, SAFE>(&amount, &currency)?,
        "CAD" => parse_amount::<CAD, SAFE>(&amount, &currency)?,
        "CDF" => parse_amount::<CDF, SAFE>(&amount, &currency)?,
        "CHE" => parse_amount::<CHE, SAFE>(&amount, &currency)?,
        "CHF" => parse_amount::<CHF, SAFE>(&amount, &currency)?,
        "CHW" => parse_amount::<CHW, SAFE>(&amount, &currency)?,
        "CLF" => parse_amount::<CLF, SAFE>(&amount, &currency)?,
        "CLP" => parse_amount::<CLP, SAFE>(&amount, &currency)?,
        "COP" => parse_amount::<COP, SAFE>(&amount, &currency)?,
        "COU" => parse_amount::<COU, SAFE>(&amount, &currency)?,
        "CRC" => parse_amount::<CRC, SAFE>(&amount, &currency)?,
        "CUC" => parse_amount::<CUC, SAFE>(&amount, &currency)?,
        "CUP" => parse_amount::<CUP, SAFE>(&amount, &currency)?,
        "CVE" => parse_amount::<CVE, SAFE>(&amount, &currency)?,
        "CZK" => parse_amount::<CZK, SAFE>(&amount, &currency)?,
        "DJF" => parse_amount::<DJF, SAFE>(&amount, &currency)?,
        "DKK" => parse_amount::<DKK, SAFE>(&amount, &currency)?,
        "DOP" => parse_amount::<DOP, SAFE>(&amount, &currency)?,
        "DZD" => parse_amount::<DZD, SAFE>(&amount, &currency)?,
        "EGP" => parse_amount::<EGP, SAFE>(&amount, &currency)?,
        "ERN" => parse_amount::<ERN, SAFE>(&amount, &currency)?,
        "ETB" => parse_amount::<ETB, SAFE>(&amount, &currency)?,
        "EUR" => parse_amount::<EUR, SAFE>(&amount, &currency)?,
        "FJD" => parse_amount::<FJD, SAFE>(&amount, &currency)?,
        "FKP" => parse_amount::<FKP, SAFE>(&amount, &currency)?,
        "GBP" => parse_amount::<GBP, SAFE>(&amount, &currency)?,
        "GEL" => parse_amount::<GEL, SAFE>(&amount, &currency)?,
        "GHS" => parse_amount::<GHS, SAFE>(&amount, &currency)?,
        "GIP" => parse_amount::<GIP, SAFE>(&amount, &currency)?,
        "GMD" => parse_amount::<GMD, SAFE>(&amount, &currency)?,
        "GNF" => parse_amount::<GNF, SAFE>(&amount, &currency)?,
        "GTQ" => parse_amount::<GTQ, SAFE>(&amount, &currency)?,
        "HKD" => parse_amount::<HKD, SAFE>(&amount, &currency)?,
        "HNL" => parse_amount::<HNL, SAFE>(&amount, &currency)?,
        "HTG" => parse_amount::<HTG, SAFE>(&amount, &currency)?,
        "HUF" => parse_amount::<HUF, SAFE>(&amount, &currency)?,
        "IDR" => parse_amount::<IDR, SAFE>(&amount, &currency)?,
        "ILS" => parse_amount::<ILS, SAFE>(&amount, &currency)?,
        "INR" => parse_amount::<INR, SAFE>(&amount, &currency)?,
        "IQD" => parse_amount::<IQD, SAFE>(&amount, &currency)?,
        "IRR" => parse_amount::<IRR, SAFE>(&amount, &currency)?,
        "ISK" => parse_amount::<ISK, SAFE>(&amount, &currency)?,
        "JMD" => parse_amount::<JMD, SAFE>(&amount, &currency)?,
        "JOD" => parse_amount::<JOD, SAFE>(&amount, &currency)?,
        "JPY" => parse_amount::<JPY, SAFE>(&amount, &currency)?,
        "KES" => parse_amount::<KES, SAFE>(&amount, &currency)?,
        "KGS" => parse_amount::<KGS, SAFE>(&amount, &currency)?,
        "KHR" => parse_amount::<KHR, SAFE>(&amount, &currency)?,
        "KMF" => parse_amount::<KMF, SAFE>(&amount, &currency)?,
        "KPW" => parse_amount::<KPW, SAFE>(&amount, &currency)?,
        "KRW" => parse_amount::<KRW, SAFE>(&amount, &currency)?,
        "KWD" => parse_amount::<KWD, SAFE>(&amount, &currency)?,
        "KYD" => parse_amount::<KYD, SAFE>(&amount, &currency)?,
        "KZT" => parse_amount::<KZT, SAFE>(&amount, &currency)?,
        "LAK" => parse_amount::<LAK, SAFE>(&amount, &currency)?,
        "LBP" => parse_amount::<LBP, SAFE>(&amount, &currency)?,
        "LKR" => parse_amount::<LKR, SAFE>(&amount, &currency)?,
        "LRD" => parse_amount::<LRD, SAFE>(&amount, &currency)?,
        "LSL" => parse_amount::<LSL, SAFE>(&amount, &currency)?,
        "LYD" => parse_amount::<LYD, SAFE>(&amount, &currency)?,
        "MAD" => parse_amount::<MAD, SAFE>(&amount, &currency)?,
        "MDL" => parse_amount::<MDL, SAFE>(&amount, &currency)?,
        "MGA" => parse_amount::<MGA, SAFE>(&amount, &currency)?,
        "MKD" => parse_amount::<MKD, SAFE>(&amount, &currency)?,
        "MMK" => parse_amount::<MMK, SAFE>(&amount, &currency)?,
        "MNT" => parse_amount::<MNT, SAFE>(&amount, &currency)?,
        "MOP" => parse_amount::<MOP, SAFE>(&amount, &currency)?,
        "MRU" => parse_amount::<MRU, SAFE>(&amount, &currency)?,
        "MUR" => parse_amount::<MUR, SAFE>(&amount, &currency)?,
        "MVR" => parse_amount::<MVR, SAFE>(&amount, &currency)?,
        "MWK" => parse_amount::<MWK, SAFE>(&amount, &currency)?,
        "MXN" => parse_amount::<MXN, SAFE>(&amount, &currency)?,
        "MXV" => parse_amount::<MXV, SAFE>(&amount, &currency)?,
        "MYR" => parse_amount::<MYR, SAFE>(&amount, &currency)?,
        "MZN" => parse_amount::<MZN, SAFE>(&amount, &currency)?,
        "NAD" => parse_amount::<NAD, SAFE>(&amount, &currency)?,
        "NGN" => parse_amount::<NGN, SAFE>(&amount, &currency)?,
        "NIO" => parse_amount::<NIO, SAFE>(&amount, &currency)?,
        "NOK" => parse_amount::<NOK, SAFE>(&amount, &currency)?,
        "NPR" => parse_amount::<NPR, SAFE>(&amount, &currency)?,
        "NZD" => parse_amount::<NZD, SAFE>(&amount, &currency)?,
        "OMR" => parse_amount::<OMR, SAFE>(&amount, &currency)?,
        "PAB" => parse_amount::<PAB, SAFE>(&amount, &currency)?,
        "PEN" => parse_amount::<PEN, SAFE>(&amount, &currency)?,
        "PGK" => parse_amount::<PGK, SAFE>(&amount, &currency)?,
        "PHP" => parse_amount::<PHP, SAFE>(&amount, &currency)?,
        "PKR" => parse_amount::<PKR, SAFE>(&amount, &currency)?,
        "PLN" => parse_amount::<PLN, SAFE>(&amount, &currency)?,
        "PYG" => parse_amount::<PYG, SAFE>(&amount, &currency)?,
        "QAR" => parse_amount::<QAR, SAFE>(&amount, &currency)?,
        "RON" => parse_amount::<RON, SAFE>(&amount, &currency)?,
        "RSD" => parse_amount::<RSD, SAFE>(&amount, &currency)?,
        "CNY" => parse_amount::<CNY, SAFE>(&amount, &currency)?,
        "RUB" => parse_amount::<RUB, SAFE>(&amount, &currency)?,
        "RWF" => parse_amount::<RWF, SAFE>(&amount, &currency)?,
        "SAR" => parse_amount::<SAR, SAFE>(&amount, &currency)?,
        "SBD" => parse_amount::<SBD, SAFE>(&amount, &currency)?,
        "SCR" => parse_amount::<SCR, SAFE>(&amount, &currency)?,
        "SDG" => parse_amount::<SDG, SAFE>(&amount, &currency)?,
        "SEK" => parse_amount::<SEK, SAFE>(&amount, &currency)?,
        "SGD" => parse_amount::<SGD, SAFE>(&amount, &currency)?,
        "SHP" => parse_amount::<SHP, SAFE>(&amount, &currency)?,
        "SLE" => parse_amount::<SLE, SAFE>(&amount, &currency)?,
        "SOS" => parse_amount::<SOS, SAFE>(&amount, &currency)?,
        "SRD" => parse_amount::<SRD, SAFE>(&amount, &currency)?,
        "SSP" => parse_amount::<SSP, SAFE>(&amount, &currency)?,
        "STN" => parse_amount::<STN, SAFE>(&amount, &currency)?,
        "SYP" => parse_amount::<SYP, SAFE>(&amount, &currency)?,
        "SZL" => parse_amount::<SZL, SAFE>(&amount, &currency)?,
        "THB" => parse_amount::<THB, SAFE>(&amount, &currency)?,
        "TJS" => parse_amount::<TJS, SAFE>(&amount, &currency)?,
        "TMT" => parse_amount::<TMT, SAFE>(&amount, &currency)?,
        "TND" => parse_amount::<TND, SAFE>(&amount, &currency)?,
        "TOP" => parse_amount::<TOP, SAFE>(&amount, &currency)?,
        "TRY" => parse_amount::<TRY, SAFE>(&amount, &currency)?,
        "TTD" => parse_amount::<TTD, SAFE>(&amount, &currency)?,
        "TWD" => parse_amount::<TWD, SAFE>(&amount, &currency)?,
        "TZS" => parse_amount::<TZS, SAFE>(&amount, &currency)?,
        "UAH" => parse_amount::<UAH, SAFE>(&amount, &currency)?,
        "UGX" => parse_amount::<UGX, SAFE>(&amount, &currency)?,
        "UYU" => parse_amount::<UYU, SAFE>(&amount, &currency)?,
        "UZS" => parse_amount::<UZS, SAFE>(&amount, &currency)?,
        "VED" => parse_amount::<VED, SAFE>(&amount, &currency)?,
        "VES" => parse_amount::<VES, SAFE>(&amount, &currency)?,
        "VND" => parse_amount::<VND, SAFE>(&amount, &currency)?,
        "VUV" => parse_amount::<VUV, SAFE>(&amount, &currency)?,
        "WST" => parse_amount::<WST, SAFE>(&amount, &currency)?,
        "XAF" => parse_amount::<XAF, SAFE>(&amount, &currency)?,
        "XAG" => parse_amount::<XAG, SAFE>(&amount, &currency)?,
        "XAU" => parse_amount::<XAU, SAFE>(&amount, &currency)?,
        "XCD" => parse_amount::<XCD, SAFE>(&amount, &currency)?,
        "XOF" => parse_amount::<XOF, SAFE>(&amount, &currency)?,
        "XPD" => parse_amount::<XPD, SAFE>(&amount, &currency)?,
        "XPF" => parse_amount::<XPF, SAFE>(&amount, &currency)?,
        "XPT" => parse_amount::<XPT, SAFE>(&amount, &currency)?,
        "YER" => parse_amount::<YER, SAFE>(&amount, &currency)?,
        "ZAR" => parse_amount::<ZAR, SAFE>(&amount, &currency)?,
        "ZMW" => parse_amount::<ZMW, SAFE>(&amount, &currency)?,
        _ => {
            return Err(Error::new(
                currency.span(),
                "Unsupported currency. The `amt!` macro only works for types defined by the `currencies` crate.",
            ))
        }
    };
    Ok(output)
}
