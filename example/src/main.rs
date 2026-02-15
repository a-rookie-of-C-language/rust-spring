use spring_beans::factory::BeanDefinitionRegistry;
use spring_context::context::application_context::ApplicationContext;
use spring_context::context::ConfigurableApplicationContext;
use spring_context::context::support::AbstractApplicationContext;
use spring_macro::Component;

#[derive(Component, Debug)]
#[derive(Default)]
struct Person {
    id: i32,
    name: String,
}

#[derive(Component, Debug)]
#[derive(Default)]
struct User {
    #[autowired]
    person: Person,
    id: i32,
    name: String,
}

fn main() {
    let mut context = AbstractApplicationContext::default();
    context.register_bean_definition(Person::bean_name(), Box::new(Person::bean_definition()));
    context.register_bean_definition(User::bean_name(), Box::new(User::bean_definition()));

    context.refresh();

    if let Some(bean) = context.get_bean("person") {
        if let Some(person) = bean.downcast_ref::<Person>() {
            println!("{:?}", person);
        }
    }

    if let Some(bean) = context.get_bean("user") {
        if let Some(user) = bean.downcast_ref::<User>() {
            println!("{:?}", user);
        }
    }

    let user_default = User::default();
    println!("{:?}", user_default);
}
