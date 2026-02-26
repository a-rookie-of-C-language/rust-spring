use spring_boot::{Application, ApplicationContext, Aspect, Bean, Before, After, Around, Component, JoinPoint, AopProxyRegistry, Lazy, Scope, Value};

// ── 基础 bean ──────────────────────────────────────────────────────────────────

#[Component]
#[derive(Debug, Default, Clone)]
struct Person {
    id: i32,
    name: String,
}

// ── #[autowired] 依赖注入 ──────────────────────────────────────────────────────

#[Component]
#[derive(Debug, Default, Clone)]
struct User {
    #[autowired]
    person: Person,
    id: i32,
    name: String,
}

// ── #[Scope("prototype")] ─────────────────────────────────────────────────────
// 每次 do_create_bean 都创建新实例，不缓存到 singleton_objects

#[Component]
#[Scope("prototype")]
#[derive(Debug, Default, Clone)]
struct RequestContext {
    request_id: i32,
}

// ── #[Lazy] ───────────────────────────────────────────────────────────────────
// refresh() 时不主动创建，第一次 get_bean() 时才初始化

#[Component]
#[Lazy]
#[derive(Debug, Default, Clone)]
struct HeavyService {
    initialized: bool,
}

// ── #[Bean] ───────────────────────────────────────────────────────────────────
// 函数式定义 bean，类似 Java @Configuration + @Bean

#[derive(Debug, Clone)]
struct AppConfig {
    version: String,
    max_connections: u32,
}

#[Bean(name = "appConfig")]
fn create_app_config() -> AppConfig {
    AppConfig {
        version: "1.0.0".to_string(),
        max_connections: 100,
    }
}

// ── #[Value] 配置注入 ────────────────────────────────────────────────────────────────────
// 字段从 application.properties 注入，相当于 Java @Value

#[Component]
#[derive(Debug, Default, Clone)]
struct ServerConfig {
    #[Value("${server.port:8080}")]
    port: i32,
    #[Value("${app.name:rust-spring}")]
    app_name: String,
    #[Value("${app.version:1.0.0}")]
    version: String,
    #[Value("${app.max-connections:100}")]
    max_connections: u32,
}


// ── AOP 切面演示 ────────────────────────────────────────────────────────────────
//
// OrderService: 被拦截的 bean

#[Component]
#[derive(Debug, Default, Clone)]
struct OrderService {
    order_count: u32,
}

impl OrderService {
    /// 下单方法 —— 由 LogAspect 进行 Before / After 拦截。
    pub fn place_order(&self, item: &str) {
        // 在方法体开始时通知 Before advice
        AopProxyRegistry::fire_before("orderService", "place_order");
        println!("[OrderService] placing order for: {}", item);
        // 在方法体结束时通知 After advice
        AopProxyRegistry::fire_after("orderService", "place_order");
    }
}

// LogAspect: 切面类（用于标识切面——提示性）
#[Aspect]
struct LogAspect;

// 切面函数必须是模块级别的独立函数（非 impl 方法）
#[Before("orderService::place_order")]
fn log_before(jp: &JoinPoint) {
    println!(
        "[AOP][Before]  {}.{}() is about to execute",
        jp.bean_name, jp.method_name
    );
}

#[After("orderService::place_order")]
fn log_after(jp: &JoinPoint) {
    println!(
        "[AOP][After]   {}.{}() has finished",
        jp.bean_name, jp.method_name
    );
}

#[Around("orderService::place_order")]
fn log_around(jp: &JoinPoint) {
    // Around fires on both sides (before via fire_before, after via fire_after).
    println!(
        "[AOP][Around]  intercepting {}.{}()",
        jp.bean_name, jp.method_name
    );
}
// ── main ──────────────────────────────────────────────────────────────────────

fn main() {
    let mut context = Application::run();

    // 1. 普通 singleton bean
    if let Some(bean) = context.get_bean("person") {
        if let Some(person) = bean.downcast_ref::<Person>() {
            println!("[Singleton]  person bean: {:?}", person);
        }
    }

    // 2. autowired 注入
    if let Some(bean) = context.get_bean("user") {
        if let Some(user) = bean.downcast_ref::<User>() {
            println!("[Autowired]  user bean:   {:?}", user);
        }
    }

    // 3. Prototype bean — 每次 do_create_bean 产生新实例
    context.do_create_bean("requestContext");
    println!("[Prototype]  requestContext: prototype bean (not cached in singleton store)");

    // 4. Lazy singleton — refresh() 时跳过，首次 get_bean 时触发创建
    if context.get_bean("heavyService").is_none() {
        println!(
            "[Lazy]       heavyService: not yet initialized (lazy=true, needs do_create_bean)"
        );
        context.do_create_bean("heavyService");
    }
    if let Some(bean) = context.get_bean("heavyService") {
        if let Some(svc) = bean.downcast_ref::<HeavyService>() {
            println!("[Lazy]       heavyService initialized: {:?}", svc);
        }
    }

    // 5. @Bean 函数式定义
    if let Some(bean) = context.get_bean("appConfig") {
        if let Some(cfg) = bean.downcast_ref::<AppConfig>() {
            println!("[Bean]       appConfig: {:?}", cfg);
        }
    }

    // 6. #[Value] 配置注入
    if let Some(bean) = context.get_bean("serverConfig") {
        if let Some(cfg) = bean.downcast_ref::<ServerConfig>() {
            println!("[Value]      serverConfig: {:?}", cfg);
        }
    }

    // 7. AOP 切面拦截演示
    if let Some(bean) = context.get_bean("orderService") {
        if let Some(svc) = bean.downcast_ref::<OrderService>() {
            println!("\n[AOP] Calling orderService.place_order(\"laptop\")...");
            svc.place_order("laptop");
        }
    }
}
