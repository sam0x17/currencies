use currencies_core::{
    currency::USD,
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
        "USD" => parse_amount::<USD, SAFE>(&amount, &currency)?,
        _ => todo!(),
    };
    Ok(output)
}
