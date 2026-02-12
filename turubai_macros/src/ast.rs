use proc_macro2::{Span, TokenStream};
use syn::{
    Expr, ExprCall, Ident, Token, braced, parenthesized, parse::{Parse, ParseStream}, punctuated::Punctuated, token::{Brace, Paren}
};

use quote::quote;
use quote::ToTokens;

#[derive(Clone)]
pub struct ExprElement {
    tag: syn::Path,
    _paren_token: Option<Paren>,
    required_args: Punctuated<Expr, Token![,]>,
    optional_args: Punctuated<OptionalAttrExpr, Token![,]>,
    brace_token: Option<Brace>,
    children: Vec<Partition>,
}

fn to_snake_case(original: &str) -> String {
    let mut output = String::new();
    for (i, c) in original.chars().enumerate() {
        if c.is_ascii_uppercase() && i != 0 {
            output.push('_');
        }
        output.push(c.to_ascii_lowercase());
    }
    output
}

/// Build the modifier-forking block from optional args.
/// `default_member` is the fallback namespace (e.g. "text", "padding") when
/// an arg doesn't specify its own `namespace::` prefix.
fn build_modifiers_block(
    optional_args: &Punctuated<OptionalAttrExpr, Token![,]>,
    default_member: &str,
) -> proc_macro2::TokenStream {
    let mut set_tokens = vec![];
    for arg in optional_args {
        let field_name = arg.name.clone();
        let val = arg.value.clone();

        let member = if let Some(ref ns) = arg.namespace {
            ns.clone()
        } else {
            syn::parse_str::<Ident>(default_member).unwrap()
        };

        set_tokens.push(quote! { fm_lock.#member.#field_name = #val; });
    }

    quote! {
        {
            let mut fm = modifiers.fork();
            let mut fm_lock = fm.lock().unwrap();
            #(#set_tokens)*
            std::mem::drop(fm_lock);
            fm
        }
    }
}

impl ExprElement {
    pub fn tag(&self) -> String {
        self.tag.segments.last().unwrap().ident.to_string()
    }

    pub fn path(&self) -> syn::Path {
        self.tag.clone()
    }

    pub fn to_token_stream(&self) -> syn::Result<proc_macro2::TokenStream> {
        let tag = self.tag();
        let path = self.path();

        let required_args = self.required_args();
        let optional_args = self.optional_args();

        // Collect all children from all partitions
        let mut child_stmts = vec![];
        let mut child_names = Punctuated::<Ident, Token![,]>::new();
        let mut idx = 0_usize;
        for partition in &self.children {
            partition.collect_children(&mut child_stmts, &mut child_names, &mut idx)?;
        }
        child_stmts.push(quote! {
            vec![#child_names]
        });

        let wrapped_children_function = quote! {
            move |modifiers| {#(#child_stmts)*}
        };

        let method_name = Ident::new(
            &format!("turubai_new_with_{}_args", self.required_args.len()),
            path.segments.last().unwrap().ident.span(),
        );
        let result = if required_args.is_empty() {
            quote! { #path::#method_name(#optional_args, #wrapped_children_function) }
        } else {
            quote! { #path::#method_name(#required_args, #optional_args, #wrapped_children_function) }
        };

        if cfg!(feature = "debug") {
            eprintln!("\n=== {} ===", tag);
            eprintln!("{}", result.to_string().replace(" :: ", "::"));
        }

        Ok(result)
    }

    pub fn required_args(&self) -> proc_macro2::TokenStream {
        self.required_args.to_token_stream().into()
    }

    pub fn optional_args(&self) -> proc_macro2::TokenStream {
        let default_member = to_snake_case(&self.tag());
        build_modifiers_block(&self.optional_args, &default_member)
    }
}

fn parse_attributes(
    input: ParseStream,
) -> syn::Result<(
    Punctuated<Expr, Token![,]>,
    Punctuated<OptionalAttrExpr, Token![,]>,
)> {
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

        let mut children = Vec::new();
        let mut brace_token = None;
        let lookahead = input.lookahead1();
        if lookahead.peek(Brace) {
            let content;
            brace_token = Some(braced!(content in input));
            children = parse_partitions(&content)?;
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

/// A simple method call: `method_name(args)`
/// This is simpler than syn::ExprCall and doesn't try to parse method chains.
#[derive(Clone)]
pub struct SimpleMethodCall {
    pub name: Ident,
    pub paren_token: Paren,
    pub required_args: Punctuated<Expr, Token![,]>,
    pub optional_args: Punctuated<OptionalAttrExpr, Token![,]>,
}

impl Parse for SimpleMethodCall {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;

        let content;
        let paren_token = parenthesized!(content in input);
        let (required_args, optional_args) = parse_attributes(&content)?;

        Ok(Self {
            name,
            paren_token,
            required_args,
            optional_args,
        })
    }
}

#[derive(Clone)]
pub struct ExprPostProcessStack {
    element: ExprElement,
    _period_token: Option<Token![.]>,
    stack: Punctuated<SimpleMethodCall, Token![.]>,
}

impl ExprPostProcessStack {
    pub fn to_token_stream(&self) -> syn::Result<TokenStream> {
        let child = self.element.to_token_stream()?;
        let boxed_child = quote! { Box::new(#child) };

        let mut prev = syn::parse2::<Expr>(boxed_child)?;
        for call in self.stack.iter() {
            let default_member = to_snake_case(&call.name.to_string());
            let modifiers_block = build_modifiers_block(&call.optional_args, &default_member);

            let mut args = Punctuated::new();
            args.extend(call.required_args.iter().cloned());
            args.push(prev);
            args.push(syn::parse2::<Expr>(modifiers_block)?);

            let syn_call = ExprCall {
                func: Box::new(syn::parse_str::<Expr>(&call.name.to_string())?),
                args,
                attrs: Vec::new(),
                paren_token: call.paren_token.clone()
            };
            let call_expr = Expr::Call(syn_call);
            prev = syn::parse2::<Expr>(quote! { Box::new(#call_expr) })?;
        }

        Ok(quote! { #prev }.into())
    }
}

impl Parse for ExprPostProcessStack {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let element = input.parse::<ExprElement>()?;

        let lookahead = input.lookahead1();
        if lookahead.peek(Token![.]) {
            let period_token = Some(input.parse::<Token![.]>()?);
            let parser = Punctuated::<SimpleMethodCall, Token![.]>::parse_separated_nonempty;
            let stack = parser(input)?;
            Ok(ExprPostProcessStack {
                element,
                stack,
                _period_token: period_token,
            })
        } else {
            Ok(ExprPostProcessStack {
                element,
                stack: Punctuated::new(),
                _period_token: None,
            })
        }
    }
}

#[derive(Clone)]
pub enum Partition {
    Element(ExprPostProcessStack),
    // Future: Loop, Conditional, etc.
}

impl Partition {
    /// Append boxed child statements and names for use in a children vec.
    fn collect_children(
        &self,
        stmts: &mut Vec<TokenStream>,
        names: &mut Punctuated<Ident, Token![,]>,
        idx: &mut usize,
    ) -> syn::Result<()> {
        match self {
            Partition::Element(post) => {
                let render = post.to_token_stream()?;
                let child_name = Ident::new(&format!("ch_{}", *idx), Span::call_site());
                stmts.push(quote! { let #child_name = #render; });
                names.push(child_name);
                *idx += 1;
            }
        }
        Ok(())
    }
}

/// Parse a sequence of partitions. Elements are separated by optional commas.
fn parse_partitions(input: ParseStream) -> syn::Result<Vec<Partition>> {
    let mut partitions = Vec::new();
    while !input.is_empty() {
        partitions.push(Partition::Element(input.parse::<ExprPostProcessStack>()?));
        // Consume optional trailing comma
        let _ = input.parse::<Option<Token![,]>>();
    }
    Ok(partitions)
}

pub struct Ast {
    partitions: Vec<Partition>,
}

impl Parse for Ast {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let partitions = parse_partitions(input)?;
        Ok(Ast { partitions })
    }
}

impl Ast {
    pub fn to_token_stream(&self) -> syn::Result<TokenStream> {
        // The top-level AST should produce a single element expression.
        // Collect all partition outputs; the last one is the result.
        let mut stmts = Vec::new();
        let mut names = Punctuated::<Ident, Token![,]>::new();
        let mut idx = 0_usize;
        for p in &self.partitions {
            p.collect_children(&mut stmts, &mut names, &mut idx)?;
        }

        // Return the last element as the result
        if let Some(last) = names.last() {
            Ok(quote! {
                #(#stmts)*
                #last
            })
        } else {
            Ok(quote! {})
        }
    }
}

#[derive(Clone)]
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
            Ok(Self {
                namespace: Some(namespace),
                name,
                _div: div,
                value,
            })
        } else if look.peek(Token![:]) {
            let div = input.parse::<Token![:]>()?;
            let value = input.parse::<Expr>()?;
            Ok(Self {
                namespace: None,
                name: namespace,
                _div: div,
                value,
            })
        } else {
            Err(look.error())
        }
    }
}
