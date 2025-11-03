use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, ItemFn, LitStr, ReturnType, Type, meta, parse::Parser, spanned::Spanned};

/// Automatically installs Greentic telemetry at runtime entry-points.
///
/// ```ignore
/// #[greentic_types::telemetry::main(service_name = "runner")]
/// async fn main() -> anyhow::Result<()> {
///     // tracing/logging is ready here.
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    expand_main(args, item).unwrap_or_else(|err| err.to_compile_error().into())
}

fn expand_main(args: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let service_name = parse_service_name(args)?;

    let mut item_fn: ItemFn = syn::parse(item)?;
    ensure_async(&item_fn)?;
    ensure_no_args(&item_fn)?;

    strip_self_attr(&mut item_fn.attrs);

    let vis = item_fn.vis.clone();
    let user_ident = item_fn.sig.ident.clone();
    let inner_ident = format_ident!("__greentic_types_main");
    item_fn.sig.ident = inner_ident.clone();
    item_fn.vis = syn::Visibility::Inherited;

    let generics = item_fn.sig.generics.clone();
    let where_clause = generics.where_clause.clone();
    let inputs = item_fn.sig.inputs.clone();
    let output = item_fn.sig.output.clone();
    let returns_result = is_result_return(&output);
    let install_stmt = if returns_result {
        quote! {
            ::greentic_types::telemetry::install_telemetry(#service_name)
                .map_err(::core::convert::Into::into)?;
        }
    } else {
        quote! {
            ::greentic_types::telemetry::install_telemetry(#service_name)
                .expect("greentic telemetry initialization failed");
        }
    };

    let expanded = quote! {
        #[::greentic_types::telemetry::__tokio_main]
        #vis async fn #user_ident #generics (#inputs) #output #where_clause {
            #install_stmt
            #inner_ident().await
        }

        #item_fn
    };

    Ok(expanded.into())
}

fn parse_service_name(args: TokenStream) -> syn::Result<LitStr> {
    let mut service_name = None;
    let parser = meta::parser(|meta| {
        if meta.path.is_ident("service_name") {
            let lit: LitStr = meta.value()?.parse()?;
            if service_name.is_some() {
                return Err(meta.error("service_name specified more than once"));
            }
            service_name = Some(lit);
            Ok(())
        } else {
            Err(meta.error("expected `service_name = \"...\"`"))
        }
    });

    parser.parse2(proc_macro2::TokenStream::from(args))?;

    service_name.ok_or_else(|| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "missing `service_name = \"...\"` argument",
        )
    })
}

fn ensure_async(item_fn: &ItemFn) -> syn::Result<()> {
    if item_fn.sig.asyncness.is_none() {
        return Err(syn::Error::new(
            item_fn.sig.span(),
            "`#[greentic_types::telemetry::main]` requires an `async fn`",
        ));
    }
    Ok(())
}

fn ensure_no_args(item_fn: &ItemFn) -> syn::Result<()> {
    if !item_fn.sig.inputs.is_empty() {
        return Err(syn::Error::new(
            item_fn.sig.inputs.span(),
            "`main` must not take arguments",
        ));
    }
    Ok(())
}

fn strip_self_attr(attrs: &mut Vec<Attribute>) {
    attrs.retain(|attr| !is_self_attr(attr));
}

fn is_self_attr(attr: &Attribute) -> bool {
    let path = attr.path();
    let segments: Vec<_> = path
        .segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect();
    segments == ["greentic_types", "telemetry", "main"]
}

fn is_result_return(output: &ReturnType) -> bool {
    match output {
        ReturnType::Type(_, ty) => is_result_type(ty),
        ReturnType::Default => false,
    }
}

fn is_result_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .map(|segment| segment.ident == "Result")
            .unwrap_or(false),
        _ => false,
    }
}
