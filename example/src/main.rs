use spring_macro::component;  // 移除 autowired，不再需要
use spring_context::{DefaultApplicationContext, ApplicationContext};
use spring_core::registry::BeanDefinitionRegistry;
use spring_core::bean::factory::BeanFactory;
use std::sync::Arc;

#[component(lazy = false, scope = "Singleton")]
#[derive(Default)]
struct Database {
    connection: String,
}

impl Database {
    fn query(&self) -> String {
        format!("查询数据库: {}", self.connection)
    }
}

#[component(lazy = false, scope = "Singleton")]
#[derive(Default)]
struct UserRepository {
    // 自动识别 Arc<T> 类型为依赖，无需 #[autowired]
    db: Arc<Database>,
}

#[component(lazy = false, scope = "Singleton")]
#[derive(Default)]
struct UserService {
    // 自动识别 Arc<T> 类型为依赖，无需 #[autowired]
    repo: Arc<UserRepository>,
}

fn main() {
    println!("🚀 启动 Spring Context 测试\n");

    // 创建容器
    let mut context = DefaultApplicationContext::new();

    // 刷新容器（自动加载并初始化所有 Bean）
    context.refresh();

    println!("\n--- 获取 Bean ---");

    // 获取 Bean
    if let Some(_service) = context.get_bean("UserService") {
        println!("✅ UserService 获取成功");
    }

    // 检查 Bean 信息
    println!("\n--- Bean 信息 ---");
    println!("包含 Database: {}", context.contains_bean("Database"));
    println!("包含 UserRepository: {}", context.contains_bean("UserRepository"));
    println!("UserService 是单例: {}", context.is_singleton("UserService"));

    // 列出所有 Bean
    println!("\n--- 所有注册的 Bean ---");
    for name in context.get_bean_definition_names() {
        println!("  - {}", name);
    }
}