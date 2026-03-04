use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, ImplItem, ItemImpl, LitStr, Visibility};

// ── #[AopMethods] ─────────────────────────────────────────────────────────────
//
// Apply to an `impl` block.  Every `pub fn` that takes `&self` or `&mut self`
// is automatically wrapped with `fire_before` / `fire_after` (via `AopGuard`).
//
// Usage:
//
//   #[Component]
//   #[derive(Debug, Default, Clone)]
//   struct OrderService { order_count: u32 }
//
//   #[AopMethods]
//   impl OrderService {
//       pub fn place_order(&self, item: &str) {
//           println!("[OrderService] placing order for: {}", item);  // no manual fire_* calls
//       }
//   }
//
// The bean name is derived from the struct name (lowercase first letter).
// The method name is the function identifier as a string literal.

pub fn aop_methods_impl(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let mut impl_block = parse_macro_input!(item as ItemImpl);

    // Derive bean name from the self_ty, e.g. `OrderService` → `"orderService"`
    let bean_name = extract_bean_name(&impl_block.self_ty);

    for impl_item in &mut impl_block.items {
        if let ImplItem::Fn(method) = impl_item {
            // Only intercept `pub` methods with a `self` / `&self` / `&mut self` receiver
            if !matches!(method.vis, Visibility::Public(_)) {
                continue;
            }
            if !has_self_receiver(method) {
                continue;
            }

            let bn = LitStr::new(&bean_name, Span::call_site());
            let mn = LitStr::new(&method.sig.ident.to_string(), Span::call_site());

            // Take ownership of the original body statements
            let original_stmts = std::mem::take(&mut method.block.stmts);

            // Rebuild the body:
            //   1. fire_before (explicit call)
            //   2. _aop_guard  (Drop impl → fire_after, even on early return)
            //   3. original statements
            let new_stmts: Vec<syn::Stmt> = syn::parse_quote! {
                spring_boot::AopProxyRegistry::fire_before(#bn, #mn);
                let _aop_guard = spring_boot::AopGuard::new(#bn, #mn);
                #(#original_stmts)*
            };
            method.block.stmts = new_stmts;
        }
    }

    quote! { #impl_block }.into()
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Extract the last path segment from a `Type::Path` and lowercase its first letter.
/// `impl OrderService` → `"orderService"`
fn extract_bean_name(ty: &syn::Type) -> String {
    if let syn::Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            let raw = seg.ident.to_string();
            let mut chars = raw.chars();
            return match chars.next() {
                Some(c) => format!("{}{}", c.to_lowercase(), chars.collect::<String>()),
                None => raw,
            };
        }
    }
    "unknown".to_string()
}

/// Returns `true` if the method signature has a `self`, `&self`, or `&mut self` receiver.
fn has_self_receiver(method: &syn::ImplItemFn) -> bool {
    method
        .sig
        .inputs
        .iter()
        .any(|arg| matches!(arg, syn::FnArg::Receiver(_)))
}
