use proc_macro::TokenStream;

mod component;
mod bean;
mod value;
mod data;
mod getter;
mod setter;
mod accessors;
mod no_arg_constructor;
mod all_args_constructor;
mod aspect;
#[proc_macro_attribute]
pub fn component(attribute: TokenStream, item: TokenStream) -> TokenStream {
    component::component_impl(attribute, item)
}

/// derive macro 内部别名（保持向后兼容）
#[proc_macro_derive(ComponentDerive, attributes(autowired))]
    pub fn component_derive(item: TokenStream) -> TokenStream {
    component::component_derive_impl(item)
}

/// #[Component] attribute macro —— Spring 风格的主入口，自动处理 #[autowired] 字段注入
#[proc_macro_attribute]
#[allow(non_snake_case)]
    pub fn Component(attribute: TokenStream, item: TokenStream) -> TokenStream {
    component::component_impl(attribute, item)
}

/// #[Scope("prototype")] / #[Scope("singleton")] —— 附加在 #[Component] struct 上，指定 bean 作用域
/// 本宏仅作 helper attribute 使用，真正逻辑由 #[Component] 处理。
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Scope(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    item  // 透传，内容由 #[Component] 处理
}

/// #[Lazy] / #[Lazy(false)] —— 附加在 #[Component] struct 上，指定 bean 是否延迟初始化
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Lazy(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    item  // 透传，内容由 #[Component] 处理
}

/// #[Bean] —— 方法级别注解，类似 Java @Bean。标注在函数上，函数返回值就是 bean 实例。
/// 支持: #[Bean] / #[Bean(name="foo")] / #[Bean(scope="prototype")] / #[Bean(lazy=true)]
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Bean(attribute: TokenStream, item: TokenStream) -> TokenStream {
    bean::bean_impl(attribute, item)
}

/// #[Value("${key:default}")] —— 字段级注解，从 Environment 注入配置值。
/// 本宏仅作 helper attribute 使用，真正逻辑由 #[Component] 处理。
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Value(attribute: TokenStream, item: TokenStream) -> TokenStream {
    value::value_impl(attribute, item)
}

#[proc_macro_attribute]
pub fn data(attribute: TokenStream, item: TokenStream) -> TokenStream {
    data::data_impl(attribute, item)
}

#[proc_macro_attribute]
pub fn getter(attribute: TokenStream, item: TokenStream) -> TokenStream {
    getter::getter_impl(attribute, item)
}

#[proc_macro_attribute]
pub fn setter(attribute: TokenStream, item: TokenStream) -> TokenStream {
    setter::setter_impl(attribute, item)
}

#[proc_macro_attribute]
pub fn no_arg_constructor(attribute: TokenStream, item: TokenStream) -> TokenStream {
    no_arg_constructor::no_arg_constructor_impl(attribute, item)
}

#[proc_macro_attribute]
pub fn all_args_constructor(attribute: TokenStream, item: TokenStream) -> TokenStream {
    all_args_constructor::all_args_constructor_impl(attribute, item)
}

/// #[Aspect] —— Marks a struct as an aspect container (pass-through).
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Aspect(attribute: TokenStream, item: TokenStream) -> TokenStream {
    aspect::aspect_impl(attribute, item)
}

/// #[Before("beanName::methodName")] —— registers a Before advice.
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Before(attribute: TokenStream, item: TokenStream) -> TokenStream {
    aspect::before_impl(attribute, item)
}

/// #[After("beanName::methodName")] —— registers an After advice.
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn After(attribute: TokenStream, item: TokenStream) -> TokenStream {
    aspect::after_impl(attribute, item)
}

/// #[Around("beanName::methodName")] —— registers an Around (before+after) advice.
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Around(attribute: TokenStream, item: TokenStream) -> TokenStream {
    aspect::around_impl(attribute, item)
}
