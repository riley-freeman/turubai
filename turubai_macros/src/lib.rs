
mod ast;
mod map;

use quote::quote;
use syn::{Token, parse::Parser, punctuated::Punctuated};

use crate::ast::ExprElement;

#[proc_macro]
pub fn turubai(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parser = Punctuated::<ExprElement, Token![,]>::parse_terminated;
    let element_expressions = match parser.parse(input) {
        Ok(elements) => elements,
        Err(err) => return err.to_compile_error().into(),
    };


    let elements: Vec<_> = element_expressions.iter().map(|expr| expr.to_token_stream())
        .collect();

    quote!{{
        let modifiers = turubai_types::Modifiers::default();
        #(#elements)*
    }}.into()
}



