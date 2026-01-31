use std::any::TypeId;
use spring_beans::factory::BeanDefinitionRegistry;
use spring_beans::factory::config::RootBeanDefinition;
use spring_context::context::application_context::ApplicationContext;
use spring_context::context::ConfigurableApplicationContext;
use spring_context::context::support::AbstractApplicationContext;
use spring_macro::{data, no_arg_constructor, all_args_constructor};

#[data]
#[no_arg_constructor]
#[all_args_constructor]
#[derive(Debug)]
#[derive(Default)]
struct User {
    id: i32,
    name: String,
}

fn main() {
    let mut context = AbstractApplicationContext::default();
    let definition = RootBeanDefinition::new(
        "user".to_string(),
        TypeId::of::<User>(),
        "singleton".to_string(),
        false,
        Vec::new(),
        Box::new(|| Box::new(User::new(1, "test".to_string()))),
    );
    context.register_bean_definition("user", Box::new(definition));
    context.refresh();

    if let Some(bean) = context.get_bean("user") {
        if let Some(user) = bean.downcast_ref::<User>() {
            println!("{:?}", user);
        }
    }

    let user_default = User::new_no_args();
    println!("{:?}", user_default);
}
