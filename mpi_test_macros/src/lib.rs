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

    let mut input_fn = parse_macro_input!(item as ItemFn);

    // Remove the #[test] attribute if present (added e.g. by rstest)
    input_fn.attrs.retain(|attr| !attr.path().is_ident("test"));

    let fn_name = &input_fn.sig.ident;
    // let mod_ident = format_ident!("{}_mpi", fn_name);

    // Create the MPI wrapper tests
    let mut wrapper_tests = Vec::new();
    for np in nps {
        let wrapper_name = format_ident!("mpi_np_{}", np);
        wrapper_tests.push(quote! {
            #[test]
            fn #wrapper_name() {
                // Strip the crate name from module_path and get parent module
                let full_path = module_path!();
                let parent_path = full_path.rsplitn(2, "::").nth(1).unwrap();
                let test_path = if let Some(stripped) = parent_path.split_once("::") {
                    format!("{}::{}", stripped.1, stringify!(#fn_name))
                } else {
                    stringify!(#fn_name).to_string()
                };
                mpi_test_runner::run_mpi(&test_path, #np)
                    .expect("MPI test failed");
            }
        });
    }

    let expanded = quote! {
        // must be included as regular test and ignored, otherwise `cargo test` won't see it
        #[test]
        #[ignore]
        #input_fn

        #[cfg(test)]
        mod #fn_name {
            use super::*;
            use mpi_test_runner::run_mpi;

            #(#wrapper_tests)*
        }
    };

    // let ts = std::time::SystemTime::now()
    //     .duration_since(std::time::UNIX_EPOCH)
    //     .unwrap()
    //     .as_micros();

    // std::fs::write(
    //     format!("/home/philipp/Documents/mpi_test/target/mpi_marco_expansion_{ts}.rs"),
    //     expanded.to_string(),
    // )
    // .unwrap();

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
