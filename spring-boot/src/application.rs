use spring_beans::factory::BeanDefinitionRegistry;
use spring_beans::env::{Environment, PropertiesLoader, MapPropertySource};
use spring_context::context::support::AbstractApplicationContext;
use spring_context::context::ConfigurableApplicationContext;
use spring_beans::bean::bean_post_processor::DefaultBeanPostProcessor;
use spring_aop::initialize_aop;

/// Spring Boot 应用入口，对标 Java 的 SpringApplication。
pub struct Application;

impl Application {
    /// 自动扫描所有 #[Component] bean，注册到容器，refresh 后返回。
    /// 对标 Java 的 SpringApplication.run()。
    pub fn run() -> AbstractApplicationContext {
        let mut context = AbstractApplicationContext::default();

        // 先加载环境，供条件过滤使用
        let mut environment = Environment::new();
        if let Ok(props) = PropertiesLoader::load("application.properties") {
            let source = MapPropertySource::new("application.properties", props);
            environment.merge_from(&source);
        }

        // 遍历所有通过 inventory::submit! 注册的 BeanRegistration
        // 按条件过滤后再注册
        for registration in inventory::iter::<spring_beans::registry::BeanRegistration> {
            let definition = (registration.definition)();

            // 检查 #[ConditionalOnProperty] 条件
            if let Some((key, expected)) = definition.get_condition() {
                let actual = environment.get_property(key).unwrap_or("");
                if actual != expected {
                    continue; // 条件不满足，跳过该 bean
                }
            }

            let name = definition.get_name().to_string();
            context.register_bean_definition(&name, Box::new(definition));
        }

        context.set_environment(environment);

        // 注册默认的 BeanPostProcessor
        context.register_post_processor(Box::new(DefaultBeanPostProcessor {}));

        // 初始化 AOP：将所有 inventory 提交的 AspectRegistration 转为 Advisor
        initialize_aop();

        context.refresh();
        context
    }
}