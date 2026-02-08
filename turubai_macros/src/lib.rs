mod ast;
mod map;

use quote::quote;
use syn::{parse::Parser, punctuated::Punctuated, Token};

use crate::ast::{ExprElement, ExprPostProcessStack};

#[proc_macro]
pub fn turubai(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parser = Punctuated::<ExprPostProcessStack, Token![,]>::parse_terminated;
    let element_expressions: Punctuated<ExprElement, Token![,]> = match parser.parse(input) {
        Ok(elements) => elements
            .iter()
            .map(|post| post.into_expr_element())
            .collect(),
        Err(err) => return err.to_compile_error().into(),
    };

    let element_tokens: Vec<_> = element_expressions
        .iter()
        .map(|expr| {
            let tokens = expr.to_token_stream();
            quote! {Box::new(#tokens)}
        })
        .collect();

    quote! {{
        let modifiers = turubai::elements::Modifiers::default();
        #(#element_tokens)*
    }}
    .into()
}
