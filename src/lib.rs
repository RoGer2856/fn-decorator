#![allow(clippy::redundant_clone)]

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{
    bracketed, parenthesized,
    parse::{discouraged::Speculative, Parse},
    punctuated::Punctuated,
    Expr, FnArg, ImplItemFn, Pat, Path, Token,
};

fn read_exact_ident<'a>(
    ident_name: &'a str,
    input: &syn::parse::ParseStream,
) -> syn::Result<&'a str> {
    input.step(|cursor| {
        if let Some((ident, rest)) = cursor.ident() {
            if ident == ident_name {
                return Ok((ident, rest));
            }
        }
        Err(cursor.error(format!("expected `{ident_name}`")))
    })?;

    Ok(ident_name)
}

struct DecoratorFunctionCall {
    middleware_fn_path: Path,
    middleware_params: Punctuated<Expr, Token![,]>,
}

impl Parse for DecoratorFunctionCall {
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

struct HideParameterName(String);

impl Parse for HideParameterName {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.step(|cursor| {
            if let Some((ident, rest)) = cursor.ident() {
                Ok((ident.to_string(), rest))
            } else {
                Err(cursor.error("expected identifier"))
            }
        })?))
    }
}

struct HideParametersList(Vec<String>);

impl Parse for HideParametersList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        read_exact_ident("hide_parameters", &input)?;
        input.parse::<Token![=]>()?;
        let content;
        bracketed!(content in input);
        let parameters = content
            .parse_terminated(HideParameterName::parse, Token![,])?
            .into_iter()
            .map(|param| param.0)
            .collect();

        Ok(HideParametersList(parameters))
    }
}

struct OverrideReturnType(syn::Type);

impl Parse for OverrideReturnType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        read_exact_ident("override_return_type", &input)?;
        input.parse::<Token![=]>()?;
        let type_path = input.parse::<syn::Type>()?;
        Ok(OverrideReturnType(type_path))
    }
}

impl ToTokens for OverrideReturnType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let output_type = &self.0;
        tokens.extend(quote! { #output_type });
    }
}

struct UseDecoratorArg {
    debug: bool,
    decorator_function_call: DecoratorFunctionCall,
    hide_parameters_list: Option<HideParametersList>,
    override_return_type: Option<OverrideReturnType>,
}

impl Parse for UseDecoratorArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut override_return_type = None;
        let mut hide_parameters_list = None;
        let mut decorator_function_call = None;
        let mut debug = false;

        let mut first_item = true;

        while !input.is_empty() {
            if !first_item {
                input.parse::<Token![,]>()?;
            }

            let input_fork_0 = input.fork();
            let input_fork_1 = input.fork();
            let input_fork_2 = input.fork();
            let input_fork_3 = input.fork();
            if let Ok(parsed) = input_fork_0.parse::<HideParametersList>() {
                if hide_parameters_list.is_some() {
                    return Err(input.error("at most one hide_parameters list is allowed"));
                }

                hide_parameters_list = Some(parsed);
                input.advance_to(&input_fork_0);
            } else if let Ok(parsed) = input_fork_1.parse::<DecoratorFunctionCall>() {
                if decorator_function_call.is_some() {
                    return Err(input.error("exactly one decorator function call is allowed"));
                }

                decorator_function_call = Some(parsed);
                input.advance_to(&input_fork_1);
            } else if read_exact_ident("debug", &&input_fork_2).is_ok() {
                if debug {
                    return Err(input.error("exactly one `debug` is allowed"));
                }

                debug = true;

                input.advance_to(&input_fork_2);
            } else if let Ok(parsed) = input_fork_3.parse::<OverrideReturnType>() {
                if override_return_type.is_some() {
                    return Err(input.error("at most one override_return_type list is allowed"));
                }

                override_return_type = Some(parsed);
                input.advance_to(&input_fork_3);
            } else {
                return Err(
                    input.error("expected decorator function call, or hide_parameters = [...]")
                );
            }

            first_item = false;
        }

        Ok(Self {
            debug,
            decorator_function_call: decorator_function_call
                .ok_or_else(|| input.error("exactly one decorator function call is allowed"))?,
            hide_parameters_list,
            override_return_type,
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
    let use_decorator_arg: UseDecoratorArg = syn::parse_macro_input!(arg);

    let decorator_fn_path = &use_decorator_arg.decorator_function_call.middleware_fn_path;
    let decorator_fn_params = &use_decorator_arg.decorator_function_call.middleware_params;

    let mut item_impl: ImplItemFn = syn::parse_macro_input!(input);
    let decorated_fn_signature = item_impl.sig.clone();
    let wrapper_fn_signature_output =
        if let Some(override_return_type) = use_decorator_arg.override_return_type {
            quote! {
                -> #override_return_type
            }
        } else {
            let output = decorated_fn_signature.output.clone();
            quote! {
                #output
            }
        };

    let mut wrapper_fn_signature_without_output = decorated_fn_signature;
    wrapper_fn_signature_without_output.output = syn::ReturnType::Default;

    let new_fn_name = wrapper_fn_signature_without_output.ident.to_string();
    let new_fn_ident = Ident::new(&(new_fn_name + "_fn_decorator_original"), Span::call_site());
    item_impl.sig.ident = new_fn_ident.clone();

    let fn_param_names: Punctuated<DecoratedFnArgName, Token![,]> =
        wrapper_fn_signature_without_output
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

    let (closure_async, decorator_await) = if item_impl.sig.asyncness.is_some() {
        (quote! { async }, quote! { .await })
    } else {
        (quote! {}, quote! {})
    };

    let tokens = if let Some(hide_parameters_list) = use_decorator_arg.hide_parameters_list {
        let fn_param_names: Punctuated<Ident, Token![,]> = fn_param_names
            .iter()
            .map(|param_name| match param_name {
                DecoratedFnArgName::Receiver => Ident::new("_self", Span::call_site()),
                DecoratedFnArgName::Pat(pat) => {
                    Ident::new(&pat.to_token_stream().to_string(), Span::call_site())
                }
            })
            .collect();

        let closure_params: Punctuated<&Ident, Token![,]> = fn_param_names
            .iter()
            .filter(|param_name| {
                let ident_str = param_name.to_string();
                if ident_str == "_self" {
                    !hide_parameters_list.0.contains(&"self".to_string())
                } else {
                    !hide_parameters_list.0.contains(&ident_str)
                }
            })
            .collect();

        let self_redeclaration = if is_impl_decorator {
            quote! {let _self = self;}
        } else {
            quote! {}
        };

        quote! {
            #item_impl

            #wrapper_fn_signature_without_output #wrapper_fn_signature_output {
                #self_redeclaration

                #decorator_fn_path(
                    #decorator_fn_params
                    move |#closure_params| #closure_async { #new_fn_pointer(#fn_param_names)#decorator_await },
                    #closure_params)
                #decorator_await
            }
        }
    } else {
        quote! {
            #item_impl

            #wrapper_fn_signature_without_output #wrapper_fn_signature_output {
                #decorator_fn_path(#decorator_fn_params #new_fn_pointer, #fn_param_names)#decorator_await
            }
        }
    };

    if use_decorator_arg.debug {
        panic!("Generated code = `{}`", tokens);
    }

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
