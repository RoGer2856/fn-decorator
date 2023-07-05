#![allow(clippy::redundant_clone)]

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{
    parenthesized, parse::Parse, punctuated::Punctuated, Expr, FnArg, ImplItemFn, Pat, Path, Token,
};

struct DecoratorArg {
    middleware_fn_path: Path,
    middleware_params: Punctuated<Expr, Token![,]>,
}

impl Parse for DecoratorArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let middleware_fn_path = input.parse::<Path>()?;
        let content;
        parenthesized!(content in input);
        let middleware_params = content
            .parse_terminated(Expr::parse, Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            middleware_fn_path,
            middleware_params,
        })
    }
}

enum DecoratedFnArgName {
    Receiver,
    Pat(Pat),
}

impl ToTokens for DecoratedFnArgName {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            DecoratedFnArgName::Receiver => tokens.extend(quote! { self }),
            DecoratedFnArgName::Pat(p) => tokens.extend(quote! { #p }),
        }
    }
}

fn use_decorator_impl(
    arg: TokenStream,
    input: TokenStream,
    is_impl_decorator: bool,
) -> TokenStream {
    let decorator_arg: DecoratorArg = syn::parse_macro_input!(arg);

    let decorator_fn_path = &decorator_arg.middleware_fn_path;
    let decorator_fn_params = &decorator_arg.middleware_params;

    let mut item_impl: ImplItemFn = syn::parse_macro_input!(input);
    let old_fn_signature = item_impl.sig.clone();

    let new_fn_name = old_fn_signature.ident.to_string();
    let new_fn_ident = Ident::new(
        &("__decorator_original_".to_string() + &new_fn_name),
        Span::call_site(),
    );
    item_impl.sig.ident = new_fn_ident.clone();

    let fn_param_names: Punctuated<DecoratedFnArgName, Token![,]> = old_fn_signature
        .inputs
        .iter()
        .map(|param| match param {
            FnArg::Receiver(_) => DecoratedFnArgName::Receiver,
            FnArg::Typed(p) => DecoratedFnArgName::Pat(*p.pat.clone()),
        })
        .collect();

    let decorator_fn_params = if decorator_fn_params.is_empty() {
        quote! {}
    } else {
        quote! {#decorator_fn_params,}
    };

    let new_fn_pointer = if is_impl_decorator {
        quote! {Self::#new_fn_ident}
    } else {
        quote! {#new_fn_ident}
    };

    let decorator_await = if item_impl.sig.asyncness.is_some() {
        quote! { .await }
    } else {
        quote! {}
    };

    let tokens = quote! {
        #item_impl

        #old_fn_signature {
            #decorator_fn_path(#decorator_fn_params #new_fn_pointer, #fn_param_names)#decorator_await
        }
    };

    tokens.into()
}

#[proc_macro_attribute]
pub fn use_decorator(arg: TokenStream, input: TokenStream) -> TokenStream {
    use_decorator_impl(arg, input, false)
}

#[proc_macro_attribute]
pub fn use_impl_decorator(arg: TokenStream, input: TokenStream) -> TokenStream {
    use_decorator_impl(arg, input, true)
}
