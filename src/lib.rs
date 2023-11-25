#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, Token, Expr};

struct With {
    fields: Vec<WithField>,
    over: Option<Expr>,
}

struct WithField {
    name: Ident,
    init: Option<Expr>,
}

impl Parse for With {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut fields = vec![];
        let mut over: Option<Expr> = None;
        let mut found_default = false;

        while !input.is_empty() {
            if input.peek(Token![..]) {
                input.parse::<Token![..]>()?;
                if !input.is_empty() {
                    over = Some(input.parse::<Expr>()?);
                }
                found_default = true;
                break;
            }
            let name = input.parse::<Ident>()?;
            let init: Option<Expr>;
            if input.peek(Token![:]) {
                input.parse::<Token![:]>()?;
                init = Some(input.parse::<Expr>()?);
            } else {
                init = None;
            }
            fields.push(WithField { name, init });

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }
        if !found_default {
            if !fields.is_empty() {
                input.parse::<Token![,]>()?;
            }
            input.parse::<Token![..]>()?;
        }
        Ok(With { fields, over })
    }
}

/// Initializes a structure without specifying its name.
/// 
/// # Syntax
/// 
/// ```
/// with! {
///     field_1: value,
///     field_2,
///     .. // ..Default::default()
/// }
///
///
/// with! {
///     field_1: value,
///     field_2,
///     ..base
/// }
/// ```
#[proc_macro]
pub fn with(input: TokenStream) -> TokenStream {
    let With { fields, over } = parse_macro_input!(input as With);

    let mut expanded_fields = proc_macro2::TokenStream::new();

    for WithField { name, init } in fields {
        if let Some(init) = init {
            expanded_fields.extend::<proc_macro2::TokenStream>(quote! {
                _with_obj_1.#name = #init;
            }.try_into().unwrap());
        } else {
            expanded_fields.extend::<proc_macro2::TokenStream>(quote! {
                _with_obj_1.#name = #name;
            }.try_into().unwrap());
        }
    }

    let expanded_over = over.map(|over| over.into_token_stream()).unwrap_or(quote! {::std::default::Default::default()});

    let mut expanded = TokenStream::new();

    expanded.extend::<TokenStream>(quote! {
        {
            let mut _with_obj_1 = #expanded_over;
            if false {
                _with_obj_1
            } else {
                #expanded_fields
                _with_obj_1
            }
        }
    }.try_into().unwrap());

    expanded
}