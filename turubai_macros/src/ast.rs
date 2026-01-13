use std::collections::LinkedList;
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{Expr, Ident, Token, braced, parenthesized, parse::{Parse, ParseStream}, punctuated::Punctuated, token::{Brace, Paren}};

use quote::ToTokens;
use quote::quote;

use crate::map::ELEMENTS;

struct Ast {
    markups: LinkedList<Markup>
}
        

enum Markup {
}

pub struct ExprElement {
    tag: Ident,
    paren_token: Option<Paren>,
    required_args: Punctuated<Expr, Token![,]>,
    optional_args: Punctuated<OptionalAttrExpr, Token![,]>,
    brace_token: Option<Brace>,
    children: Punctuated<ExprElement, Token![,]>,
}

impl ExprElement {
    pub fn tag(&self) -> String {
        self.tag.to_string()
    }

    pub fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let name = self.tag.to_string();
        let record = ELEMENTS.get(name.as_str()).unwrap();

        let path = record.path().into_token_stream();
        let required_args = self.required_args();
        let optional_args = self.optional_args();

        let mut children = vec![];
        let mut children_names = Punctuated::<Ident, Token![,]>::new();
        for (i, child) in self.children.iter().enumerate() {
            let render = child.to_token_stream();
            let child_name = Ident::new(&format!("ch_{i}"), self.brace_token.unwrap().span.open().clone());
            children.push(quote! {let #child_name = Box::new(#render);});
            children_names.push(child_name);
        }
        children.push(quote! {
            vec![#children_names]
        });

        let wrapped_children_function = quote! {
            || {#(#children)*}
        };

        let result = if required_args.is_empty() {
            quote! { #path::new(#optional_args, #wrapped_children_function) }
        } else {
            quote! { #path::new(#required_args, #optional_args, #wrapped_children_function) }
        };

        result
    }

    pub fn required_args(&self) -> proc_macro2::TokenStream {
        self.required_args.to_token_stream().into()
    }

    pub fn optional_args(&self) -> proc_macro2::TokenStream {
        let name = self.tag.to_string();
        let record = ELEMENTS.get(name.as_str()).unwrap();

        let structure = record.parameter_struct();

        let mut set_tokens = vec![];
        for arg in &self.optional_args {
            let name = arg.name.clone();
            let val = arg.value.clone();
            set_tokens.push(quote!{optional.#name = #val;});
        }

        quote!{
            {
                let mut optional = #structure::default();
                #(#set_tokens)*
                optional
            }
        }
    }
}

fn parse_attributes(input: ParseStream) -> syn::Result<(Punctuated<Expr, Token![,]>, Punctuated<OptionalAttrExpr, Token![,]>)> {
    let mut required_args = Punctuated::<Expr, Token![,]>::new();
    let mut optional_args = Punctuated::<OptionalAttrExpr, Token![,]>::new();
    let mut seen_optional = false;

    while !input.is_empty() {
        // Check if this is an optional argument (name: value)
        // Use fork to check without consuming tokens
        let fork = input.fork();
        if fork.parse::<Ident>().is_ok() && fork.parse::<Token![:]>().is_ok() {
            // This is an optional argument
            seen_optional = true;
            optional_args.push(input.parse()?);
        } else {
            // This should be a required argument
            if seen_optional {
                return Err(input.error("Required arguments cannot come after optional arguments"));
            }
            required_args.push(input.parse()?);
        }

        // Check for comma
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        } else {
            break;
        }
    }

    Ok((required_args, optional_args))
}

impl Parse for ExprElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tag = input.parse::<Ident>()?;

        let mut required_args = Punctuated::<Expr, Token![,]>::new();
        let mut optional_args = Punctuated::<OptionalAttrExpr, Token![,]>::new();
        let mut paren_token = None;

        let lookahead = input.lookahead1();
        if lookahead.peek(Paren) {
            let content;
            paren_token = Some(parenthesized!(content in input));
            let (req, opt) = parse_attributes(&content)?;
            required_args = req;
            optional_args = opt;
        }

        let mut children = Punctuated::<ExprElement, Token![,]>::new();
        let mut brace_token = None;
        let lookahead = input.lookahead1();
        if lookahead.peek(Brace) {
            let content;
            brace_token = Some(braced!(content in input));
            children = content.parse_terminated(ExprElement::parse, Token![,])?;
        }

        Ok(Self {
            tag,
            paren_token,
            required_args,
            optional_args,
            brace_token,
            children,
        })
    }
}


pub struct OptionalAttrExpr {
    pub name: Ident,
    pub div: Token![:],
    pub value: Expr,
}

impl Parse for OptionalAttrExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let div = input.parse::<Token![:]>()?;
        let value = input.parse::<Expr>()?;
        Ok(Self { name, div, value })
    }
}
