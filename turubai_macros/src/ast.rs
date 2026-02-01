use std::{collections, mem::uninitialized};

use syn::{Expr, Ident, LitInt, PathSegment, Token, braced, parenthesized, parse::{Parse, ParseStream}, punctuated::Punctuated, token::{Brace, Paren}};

use quote::ToTokens;
use quote::quote;

use crate::map::{ELEMENTS};

pub struct ExprElement {
    tag: syn::Path,
    _paren_token: Option<Paren>,
    required_args: Punctuated<Expr, Token![,]>,
    optional_args: Punctuated<OptionalAttrExpr, Token![,]>,
    brace_token: Option<Brace>,
    children: Punctuated<ExprElement, Token![,]>,
}

impl ExprElement {
    pub fn tag(&self) -> String {
        self.tag.segments.last().unwrap().ident.to_string()
    }

    pub fn path(&self) -> syn::Path {
        self.tag.clone()
    }

    pub fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let tag = self.tag();
        let path = self.path();

        //let record = ELEMENTS.get(tag.as_str()).expect("Element is not registered in the Turubai Macros database");

        let required_args = self.required_args();
        let optional_args = self.optional_args();

        let mut children = vec![];
        let mut children_names = Punctuated::<Ident, Token![,]>::new();
        for (i, child) in self.children.iter().enumerate() {
            let render = child.to_token_stream();
            let brace_token = self.brace_token.expect("Failed to find any brace tokens!");
            let child_name = Ident::new(&format!("ch_{i}"), brace_token.span.open().clone());
            children.push(quote! {let #child_name = Box::new(#render);});
            children_names.push(child_name);
        }
        children.push(quote! {
            vec![#children_names]
        });

        let wrapped_children_function = quote! {
            move |modifiers| {#(#children)*}
        };

        let method_name = Ident::new(&format!("new_{}", self.required_args.len()), path.segments.last().unwrap().ident.span());
        let result = if required_args.is_empty() {
            quote! { #path::#method_name(#optional_args, #wrapped_children_function) }
        } else {
            quote! { #path::#method_name(#required_args, #optional_args, #wrapped_children_function) }
        };

        if cfg!(feature = "debug") {
            eprintln!("\n=== {} ===", tag);
            eprintln!("{}", result.to_string().replace(" :: ", "::"));
        }

        result
    }

    pub fn required_args(&self) -> proc_macro2::TokenStream {
        self.required_args.to_token_stream().into()
    }

    fn to_namespace(original: &str) -> String {
        let mut output = "".to_string();
        for (i, c) in original.chars().enumerate() {
            if c.is_ascii_uppercase() && i != 0 {
                output.push('_');
            }
            output.push(c.to_ascii_lowercase());
        }
        output
    }

    pub fn optional_args(&self) -> proc_macro2::TokenStream {
        let name = self.tag();
        let default_member = Self::to_namespace(&name);

        let mut set_tokens = vec![];
        for arg in &self.optional_args {
            let field_name = arg.name.clone();
            let val = arg.value.clone();

            // Use the namespace if provided, otherwise use the default member
            let member = if let Some(ref ns) = arg.namespace {
                ns.clone()
            } else {
                let token: Ident = syn::parse_str(&default_member).unwrap();
                token
            };

            set_tokens.push(quote!{fm_lock.#member.#field_name = #val;});
        }

        quote!{
            {
                let mut fm = modifiers.fork();
                let mut fm_lock = fm.lock().unwrap();
                #(#set_tokens)*
                std::mem::drop(fm_lock);
                fm
            }
        }
    }
}

fn parse_attributes(input: ParseStream) -> syn::Result<(Punctuated<Expr, Token![,]>, Punctuated<OptionalAttrExpr, Token![,]>)> {
    let mut required_args = Punctuated::<Expr, Token![,]>::new();
    let mut optional_args = Punctuated::<OptionalAttrExpr, Token![,]>::new();
    let mut seen_optional = false;

    while !input.is_empty() {
        // Check if this is an optional argument (name: value or namespace.name: value)
        // Use fork to check without consuming tokens
        let fork = input.fork();
        let is_optional = fork.parse::<OptionalAttrExpr>().is_ok();

        if is_optional {
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
        let tag = input.parse::<syn::Path>()?;

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
            _paren_token: paren_token,
            required_args,
            optional_args,
            brace_token,
            children,
        })
    }
}


pub struct OptionalAttrExpr {
    pub namespace: Option<Ident>,
    pub name: Ident,
    pub _div: Token![:],
    pub value: Expr,
}

impl Parse for OptionalAttrExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let namespace = input.parse::<Ident>()?;
        let look = input.lookahead1();

        if look.peek(Token![::]) {
            let _path_sep_token = input.parse::<Token![::]>()?;
            let name = input.parse::<Ident>()?;
            let div = input.parse::<Token![:]>()?;
            let value = input.parse::<Expr>()?;
            Ok(Self { namespace: Some(namespace), name, _div: div, value })
        } else if look.peek(Token![:]) {
            let div = input.parse::<Token![:]>()?;
            let value = input.parse::<Expr>()?;
            Ok(Self { namespace: None, name: namespace, _div: div, value })
        } else {
            Err(look.error())
        }
    }
}
