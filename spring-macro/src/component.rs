use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::Parser;
use syn::{parse_macro_input, Expr, ExprArray, ExprLit, Fields, GenericArgument, ItemStruct, Lit, LitBool, LitStr, PathArguments, Type};

pub fn component_impl(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let args = match parse_component_args(attribute) {
        Ok(args) => args,
        Err(err) => return err.to_compile_error().into(),
    };
    let ident = &input.ident;
    let default_name = default_bean_name(ident);
    let name = args.name.unwrap_or(default_name);
    let scope = args.scope.unwrap_or_else(|| "singleton".to_string());
    let lazy = args.lazy.unwrap_or(false);
    let name_lit = LitStr::new(&name, Span::call_site());
    let scope_token = match scope.as_str() {
        "singleton" => quote! { spring_beans::factory::config::BeanScope::Singleton },
        "prototype" => quote! { spring_beans::factory::config::BeanScope::Prototype },
        _ => return syn::Error::new_spanned(&input, "scope must be \"singleton\" or \"prototype\"").to_compile_error().into(),
    };
    let deps_list = if !args.deps.is_empty() {
        args.deps
    } else if args.autowire.unwrap_or(false) {
        infer_dependencies(&input)
    } else {
        Vec::new()
    };
    let deps = deps_list
        .into_iter()
        .map(|dep| LitStr::new(&dep, Span::call_site()))
        .collect::<Vec<_>>();
    let expanded = quote! {
        impl #ident {
            pub fn bean_name() -> &'static str {
                #name_lit
            }

            pub fn bean_definition() -> spring_beans::factory::config::RootBeanDefinition {
                spring_beans::factory::config::RootBeanDefinition::new(
                    #name_lit.to_string(),
                    std::any::TypeId::of::<#ident>(),
                    #scope_token,
                    #lazy,
                    vec![#(#deps.to_string()),*],
                    Box::new(|| Box::new(#ident::default())),
                )
            }
        }
    };
    expanded.into()
}

pub fn component_derive_impl(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let ident = &input.ident;
    let name = default_bean_name(ident);
    let name_lit = LitStr::new(&name, Span::call_site());
    let deps_list = infer_autowired_dependencies(&input);
    let deps = deps_list
        .into_iter()
        .map(|dep| LitStr::new(&dep, Span::call_site()))
        .collect::<Vec<_>>();
    let expanded = quote! {
        impl #ident {
            pub fn bean_name() -> &'static str {
                #name_lit
            }

            pub fn bean_definition() -> spring_beans::factory::config::RootBeanDefinition {
                spring_beans::factory::config::RootBeanDefinition::new(
                    #name_lit.to_string(),
                    std::any::TypeId::of::<#ident>(),
                    spring_beans::factory::config::BeanScope::Singleton,
                    false,
                    vec![#(#deps.to_string()),*],
                    Box::new(|| Box::new(#ident::default())),
                )
            }
        }
    };
    expanded.into()
}

#[derive(Default)]
struct ComponentArgs {
    name: Option<String>,
    scope: Option<String>,
    lazy: Option<bool>,
    autowire: Option<bool>,
    deps: Vec<String>,
}

fn parse_component_args(attribute: TokenStream) -> syn::Result<ComponentArgs> {
    let mut args = ComponentArgs::default();
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("name") {
            let value: LitStr = meta.value()?.parse()?;
            args.name = Some(value.value());
            return Ok(());
        }
        if meta.path.is_ident("scope") {
            let value: LitStr = meta.value()?.parse()?;
            args.scope = Some(value.value());
            return Ok(());
        }
        if meta.path.is_ident("lazy") {
            let value: LitBool = meta.value()?.parse()?;
            args.lazy = Some(value.value());
            return Ok(());
        }
        if meta.path.is_ident("autowire") {
            let value: LitBool = meta.value()?.parse()?;
            args.autowire = Some(value.value());
            return Ok(());
        }
        if meta.path.is_ident("deps") {
            let expr: Expr = meta.value()?.parse()?;
            match expr {
                Expr::Array(ExprArray { elems, .. }) => {
                    for elem in elems {
                        match elem {
                            Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => args.deps.push(s.value()),
                            _ => return Err(meta.error("deps must be string literals")),
                        }
                    }
                }
                Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => {
                    args.deps.push(s.value());
                }
                _ => return Err(meta.error("deps must be string literals")),
            }
            return Ok(());
        }
        Err(meta.error("unsupported component attribute"))
    });
    parser.parse(attribute)?;
    Ok(args)
}

fn default_bean_name(ident: &syn::Ident) -> String {
    let raw = ident.to_string();
    let mut chars = raw.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_lowercase(), chars.collect::<String>()),
        None => raw,
    }
}

fn infer_dependencies(input: &ItemStruct) -> Vec<String> {
    let mut deps = Vec::new();
    let fields = match &input.fields {
        Fields::Named(fields) => &fields.named,
        _ => return deps,
    };
    for field in fields {
        if let Some(name) = extract_dependency_name(&field.ty) {
            deps.push(name);
        }
    }
    deps
}

fn infer_autowired_dependencies(input: &ItemStruct) -> Vec<String> {
    let mut deps = Vec::new();
    let fields = match &input.fields {
        Fields::Named(fields) => &fields.named,
        _ => return deps,
    };
    for field in fields {
        if field.attrs.iter().any(|attr| attr.path().is_ident("autowired")) {
            if let Some(name) = extract_dependency_name(&field.ty) {
                deps.push(name);
            }
        }
    }
    deps
}

fn extract_dependency_name(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(path) => {
            let segment = path.path.segments.first()?;
            let ident = segment.ident.to_string();
            if ident == "Option" || ident == "Box" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    let inner = args.args.first()?;
                    if let GenericArgument::Type(inner_ty) = inner {
                        return extract_dependency_name(inner_ty);
                    }
                }
            }
            if is_primitive_type(&ident) {
                return None;
            }
            Some(lowercase_first(&ident))
        }
        _ => None,
    }
}

fn is_primitive_type(ident: &str) -> bool {
    matches!(
        ident,
        "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "f32"
            | "f64"
            | "bool"
            | "String"
    )
}

fn lowercase_first(raw: &str) -> String {
    let mut chars = raw.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_lowercase(), chars.collect::<String>()),
        None => raw.to_string(),
    }
}
