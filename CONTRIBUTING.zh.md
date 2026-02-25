# 贡献指南

感谢你考虑为 rust-spring 做贡献！本文档说明了参与方式。

[English](CONTRIBUTING.md) | 中文

---

## 目录

- [行为准则](#行为准则)
- [快速开始](#快速开始)
- [贡献方式](#贡献方式)
- [开发环境](#开发环境)
- [提交规范](#提交规范)
- [Pull Request 流程](#pull-request-流程)
- [报告 Bug](#报告-bug)
- [建议新功能](#建议新功能)

---

## 行为准则

请保持尊重。欢迎建设性批评，不接受人身攻击。

---

## 快速开始

1. 在 GitHub 上 **Fork** 本仓库。
2. 将 fork 克隆到本地：
   ```bash
   git clone https://github.com/<your-username>/rust-spring.git
   cd rust-spring
   ```
3. 验证项目能正常构建：
   ```bash
   cargo build --workspace
   cargo test --workspace
   ```

---

## 贡献方式

| 类型 | 建议 |
|---|---|
| Bug 修复 | 先开 Issue，再提 PR |
| 新注解 / 新功能 | 先开 Issue 讨论设计方案 |
| 文档改进 | 直接提 PR，无需 Issue |
| 重构 / 清理 | 提 PR 并附上清晰说明 |

---

## 开发环境

### 环境要求

- Rust stable 工具链（`rustup update stable`）
- `rustfmt` 和 `clippy`（随 `rustup` 一起安装）：
  ```bash
  rustup component add rustfmt clippy
  ```

### 常用命令

```bash
# 构建全部 crate
cargo build --workspace

# 运行全部测试
cargo test --workspace

# 检查代码格式
cargo fmt --all -- --check

# 自动格式化
cargo fmt --all

# 运行 Clippy（必须零警告才能通过 CI）
cargo clippy --workspace --all-targets -- -D warnings

# 运行示例
cargo run -p example

# 生成一个新的演示项目
cargo run -p initializer -- --name demo --output /tmp
```

### 项目模块说明

```
spring-core        基础 trait 与抽象
spring-beans       BeanFactory、BeanDefinition、Environment、PropertySource
spring-context     ApplicationContext、bean 生命周期管理
spring-boot        对外入口，re-export 用户所需的全部内容
spring-macro       过程宏 crate：#[Component]、#[Bean]、#[Value] 等
spring-aop         AOP 模块（开发中）
spring-expression  表达式引擎（开发中）
spring-util        共享工具函数
example            集成演示，始终保持可运行状态
initializer        CLI 脚手架工具
```

---

## 提交规范

使用 [Conventional Commits](https://www.conventionalcommits.org/zh-hans/) 格式：

```
<类型>(<范围>): <简短描述>
```

| 类型 | 使用场景 |
|---|---|
| `feat` | 新功能或新注解 |
| `fix` | Bug 修复 |
| `refactor` | 不改变行为的代码重构 |
| `docs` | 仅修改文档 |
| `test` | 新增或修复测试 |
| `chore` | 构建脚本、CI、依赖更新 |
| `ci` | CI/CD 配置变更 |

示例：
```
feat(macro): 新增 #[ConditionalOnProperty] 注解
fix(beans): 修复首次 get_bean 调用时单例缓存未命中的问题
docs(readme): 补充 #[Scope] 使用示例
```

---

## Pull Request 流程

1. 从 `main` 创建新分支：
   ```bash
   git checkout -b feat/my-feature
   ```
2. 进行修改，保持每次提交原子化。
3. 在本地确认**所有检查通过**：
   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```
4. 向 `main` 分支**提交 PR**，填写 PR 说明。
5. 至少需要一个审核批准后方可合并。
6. 如果提交历史较乱，请在合并前 squash。

---

## 报告 Bug

请在 [GitHub Issues](https://github.com/arookieofc/rust-spring/issues) 中提交，包含以下信息：

- **Rust 版本**（`rustc --version`）
- **最小可复现示例**
- **预期行为**
- **实际行为**（附完整的错误信息或 panic 输出）

---

## 建议新功能

在 [GitHub Issues](https://github.com/arookieofc/rust-spring/issues) 中提交，打上 `enhancement` 标签，并描述：

- **你想解决的问题**
- **你的解决方案**
- **你考虑过的替代方案**

涉及新 crate 或改变现有注解语义的大型功能，请先在 Issue 中讨论，再开始编码。

---

## 联系方式

维护者：**arookieofc** — [2128194521hzz@gmail.com](mailto:2128194521hzz@gmail.com)
