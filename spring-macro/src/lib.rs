use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, Fields, ItemStruct, Type};
use std::collections::HashMap;

#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let attrs = parse_component_attrs(attr);
    let is_lazy = attrs.get("lazy").map(|v| v == "true").unwrap_or(false);
    let scope = attrs.get("scope").map(|s| s.as_str()).unwrap_or("Singleton"); 
    let dependencies = extract_dependencies(&input.fields);

    let scope_variant = match scope {
        "Prototype" => quote! { spring_beans::BeanScope::Prototype },
        _ => quote! { spring_beans::BeanScope::Singleton },
    };

    // 为每个组件生成唯一的注册函数名
    let register_fn_name = format_ident!("__register_{}", name);
    
    // 生成依赖注入的代码（保留用于未来实现真正的依赖注入）
    let _dependency_fields = generate_dependency_fields(&input.fields);
    let _dependency_init = generate_dependency_init(&input.fields);

    let expanded = quote! {
        #input

        impl #impl_generics #name #ty_generics #where_clause {
            pub fn __bean_name() -> &'static str {
                stringify!(#name)
            }
            
            pub fn __create_bean_definition() -> spring_beans::BeanDefinition {
                spring_beans::BeanDefinition {
                    name: stringify!(#name).to_string(),
                    type_id: std::any::TypeId::of::<Self>(),
                    scope: #scope_variant,
                    is_lazy: #is_lazy,
                    dependencies: vec![#(#dependencies.to_string()),*],
                }
            }
            
            pub fn __new_instance() -> Box<dyn std::any::Any> {
                Box::new(Self::default())
            }
        }

        impl #impl_generics spring_beans::Component for #name #ty_generics #where_clause {
            fn __bean_name() -> &'static str {
                stringify!(#name)
            }
            
            fn __create_bean_definition() -> spring_beans::BeanDefinition {
                spring_beans::BeanDefinition {
                    name: stringify!(#name).to_string(),
                    type_id: std::any::TypeId::of::<Self>(),
                    scope: #scope_variant,
                    is_lazy: #is_lazy,
                    dependencies: vec![#(#dependencies.to_string()),*],
                }
            }
            
            fn __new_instance() -> Box<dyn std::any::Any> {
                Box::new(Self::default())
            }
        }

        // 使用 ctor 实现自动注册（每个组件都有唯一的注册函数名）
        #[::ctor::ctor]
        fn #register_fn_name() {
            spring_context::scanner::component_registry::__register_component(
                stringify!(#name),
                || #name::__create_bean_definition(),
                || #name::__new_instance()
            );
        }
    };

    TokenStream::from(expanded)
}

/// 解析 #[component(lazy = true, scope = "Prototype")] 属性
fn parse_component_attrs(attr: TokenStream) -> HashMap<String, String> {
    let mut attrs = HashMap::new();
    let attr_str = attr.to_string();
    
    // 简单解析 key=value 对
    for pair in attr_str.split(',') {
        let parts: Vec<&str> = pair.trim().split('=').collect();
        if parts.len() == 2 {
            attrs.insert(
                parts[0].trim().to_string(),
                parts[1].trim().trim_matches('"').to_string(),
            );
        }
    }
    
    attrs
}

/// #[autowired] 标记自动注入字段
/// 注意：由于 Rust 限制，proc_macro_attribute 不能直接用在 struct 字段上
/// 作为替代方案，#[component] 宏会自动识别 Arc<T> 类型的字段作为依赖
#[proc_macro_attribute]
pub fn autowired(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 这个宏目前不做任何事情，仅保留用于未来扩展
    // 实际的依赖识别基于字段类型（Arc<T>）而不是属性
    item
}

/// 提取所有需要注入的依赖（基于字段类型）
/// 自动识别 Arc<T>、Box<T>、Rc<T> 包装的非基础类型字段
fn extract_dependencies(fields: &Fields) -> Vec<String> {
    let mut deps = Vec::new();

    if let Fields::Named(named_fields) = fields {
        for field in &named_fields.named {
            // 方法1: 检查是否有 #[autowired] 属性（如果存在）
            let has_autowired_attr = field.attrs.iter().any(|attr| {
                if let Some(ident) = attr.path().get_ident() {
                    ident == "autowired"
                } else {
                    false
                }
            });

            // 方法2: 自动识别智能指针包装的类型（Arc<T>、Box<T> 等）
            let is_smart_pointer = if let Type::Path(type_path) = &field.ty {
                if let Some(segment) = type_path.path.segments.first() {
                    let ident = segment.ident.to_string();
                    matches!(ident.as_str(), "Arc" | "Box" | "Rc")
                } else {
                    false
                }
            } else {
                false
            };

            // 如果有 #[autowired] 属性或者是智能指针类型，则提取依赖
            if has_autowired_attr || is_smart_pointer {
                if let Some(dep_name) = extract_type_name(&field.ty) {
                    if !deps.contains(&dep_name) {  // 避免重复
                        deps.push(dep_name);
                    }
                }
            }
        }
    }

    deps
}

/// 从类型中提取依赖名称
fn extract_type_name(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(type_path) => {
            // 检查包装类型
            if let Some(segment) = type_path.path.segments.first() {
                let ident = segment.ident.to_string();
                
                if ident == "Arc" || ident == "Box" || ident == "Rc" {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                            return extract_type_name(inner_ty);
                        }
                    }
                }
            }
            
            // 提取直接类型
            if let Some(segment) = type_path.path.segments.last() {
                let type_name = segment.ident.to_string();
                if !is_primitive_type(&type_name) {
                    return Some(type_name);
                }
            }
            
            None
        }
        _ => None,
    }
}

fn is_primitive_type(type_name: &str) -> bool {
    matches!(
        type_name,
        "String" | "str" |
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" |
        "f32" | "f64" | "bool" | "char" |
        "Vec" | "HashMap" | "HashSet" | "BTreeMap" | "BTreeSet" |
        "Option" | "Result" | "Box" | "Arc" | "Rc" | "Cell" | "RefCell"
    )
}

/// 生成依赖字段的元数据（用于调试）
fn generate_dependency_fields(fields: &Fields) -> proc_macro2::TokenStream {
    if let Fields::Named(named_fields) = fields {
        let field_info: Vec<_> = named_fields.named.iter()
            .filter_map(|field| {
                // 检查是否是智能指针类型
                let is_smart_pointer = if let Type::Path(type_path) = &field.ty {
                    if let Some(segment) = type_path.path.segments.first() {
                        let ident = segment.ident.to_string();
                        matches!(ident.as_str(), "Arc" | "Box" | "Rc")
                    } else {
                        false
                    }
                } else {
                    false
                };
                
                if is_smart_pointer {
                    if let Some(field_name) = &field.ident {
                        if let Some(type_name) = extract_type_name(&field.ty) {
                            return Some(quote! {
                                (stringify!(#field_name), #type_name)
                            });
                        }
                    }
                }
                None
            })
            .collect();
        
        quote! { vec![#(#field_info),*] }
    } else {
        quote! { vec![] }
    }
}

/// 生成依赖初始化代码（暂时返回空，后续实现真正的依赖注入）
fn generate_dependency_init(fields: &Fields) -> proc_macro2::TokenStream {
    if let Fields::Named(named_fields) = fields {
        let inits: Vec<_> = named_fields.named.iter()
            .filter_map(|field| {
                // 检查是否是智能指针类型
                let is_smart_pointer = if let Type::Path(type_path) = &field.ty {
                    if let Some(segment) = type_path.path.segments.first() {
                        let ident = segment.ident.to_string();
                        matches!(ident.as_str(), "Arc" | "Box" | "Rc")
                    } else {
                        false
                    }
                } else {
                    false
                };
                
                if is_smart_pointer {
                    if let Some(field_name) = &field.ident {
                        // 暂时使用 Default，实际应该从容器获取
                        return Some(quote! {
                            #field_name: Default::default()
                        });
                    }
                }
                None
            })
            .collect();
        
        if inits.is_empty() {
            quote! {}
        } else {
            quote! { #(#inits),* }
        }
    } else {
        quote! {}
    }
}
