use spring_beans::factory::BeanDefinitionRegistry;
use spring_beans::env::{Environment, PropertiesLoader, MapPropertySource};
use spring_context::context::support::AbstractApplicationContext;
use spring_context::context::ConfigurableApplicationContext;
use spring_beans::bean::bean_post_processor::DefaultBeanPostProcessor;

/// Spring Boot 应用入口，对标 Java 的 SpringApplication。
pub struct Application;

impl Application {
    /// 自动扫描所有 #[Component] bean，注册到容器，refresh 后返回。
    /// 对标 Java 的 SpringApplication.run()。
    pub fn run() -> AbstractApplicationContext {
        let mut context = AbstractApplicationContext::default();

        // 遍历所有通过 inventory::submit! 注册的 BeanRegistration
        for registration in inventory::iter::<spring_beans::registry::BeanRegistration> {
            let definition = (registration.definition)();
            let name = definition.get_name().to_string();
            context.register_bean_definition(&name, Box::new(definition));
        }

        // 加载 application.properties（当前目录查找，缺失则忽略）
        let mut environment = Environment::new();
        if let Ok(props) = PropertiesLoader::load("application.properties") {
            let source = MapPropertySource::new("application.properties", props);
            environment.merge_from(&source);
        }
        context.set_environment(environment);

        // 注册默认的 BeanPostProcessor
        context.register_post_processor(Box::new(DefaultBeanPostProcessor {}));

        context.refresh();
        context
    }
}
