mod ast;
mod map;

use quote::quote;
use syn::parse::Parse;

use crate::ast::Ast;

#[proc_macro]
pub fn turubai(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = match syn::parse::<Ast>(input) {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error().into(),
    };

    let tokens = match ast.to_token_stream() {
        Ok(tokens) => tokens,
        Err(err) => return err.to_compile_error().into(),
    };

    quote! {{
        let modifiers = Modifiers::default();
        #tokens
    }}
    .into()
}
