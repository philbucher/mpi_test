use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, ExprArray, ExprLit, ItemFn, Lit, Meta, parse_macro_input};

#[proc_macro_attribute]
pub fn mpi_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_ts = proc_macro2::TokenStream::from(attr);

    // Parse "np" argument manually from the attribute tokenstream
    let nps = match syn::parse2::<Meta>(attr_ts) {
        Ok(Meta::List(list)) => {
            // e.g. #[mpi_test(np(2,4))]
            parse_np_list(&list)
        }
        Ok(Meta::NameValue(nv)) => {
            // e.g. #[mpi_test(np = [2,4])]
            parse_np_namevalue(&nv)
        }
        _ => panic!("Invalid mpi_test attribute syntax. Expected np = [...] or np(...)."),
    };

    if nps.is_empty() {
        panic!("mpi_test requires np values!");
    }

    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let mod_ident = format_ident!("{}_mpi_wrappers", fn_name);

    // Create the MPI wrapper tests
    let mut wrapper_tests = Vec::new();
    for np in nps {
        let wrapper_name = format_ident!("{}_np_{}", fn_name, np);
        wrapper_tests.push(quote! {
            #[test]
            fn #wrapper_name() {
                mpi_test_runner::run_mpi(stringify!(#fn_name), #np)
                    .expect("MPI test failed");
            }
        });
    }

    let expanded = quote! {
        #input_fn

        #[cfg(test)]
        mod #mod_ident {
            use super::*;
            use mpi_test_runner::run_mpi;

            #(#wrapper_tests)*
        }
    };

    expanded.into()
}

/// Parses `#[mpi_test(np(2,4))]`
fn parse_np_list(list: &syn::MetaList) -> Vec<u32> {
    let mut out = Vec::new();

    for token in list.tokens.clone() {
        if let Ok(expr) = syn::parse2::<Expr>(token.into()) {
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(i), ..
            }) = expr
            {
                out.push(i.base10_parse::<u32>().unwrap());
            }
        }
    }

    out
}

/// Parses `#[mpi_test(np = [2,4])]`
fn parse_np_namevalue(nv: &syn::MetaNameValue) -> Vec<u32> {
    let mut out = Vec::new();

    match &nv.value {
        Expr::Array(ExprArray { elems, .. }) => {
            for elem in elems {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Int(i), ..
                }) = elem
                {
                    out.push(i.base10_parse::<u32>().unwrap());
                }
            }
        }
        Expr::Lit(ExprLit {
            lit: Lit::Int(i), ..
        }) => {
            out.push(i.base10_parse::<u32>().unwrap());
        }
        _ => panic!("np must be an array like `[2,4]` or a single integer"),
    }

    out
}
