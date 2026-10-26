# UltraGPT-Connect 验证记录

本分支基于 SuperGPT Connect v0.1.6，源码提交 65c260eb1093341f44f9fbb2dc01fbb53cd72140。以下为 UltraGPT 定制版实际完成的验证，不沿用上游报告冒充本次结果。

## 已完成（Windows x64）

- 前端 17 项测试通过，包含默认 UltraGPT 站点、旧站点偏好不能恢复默认旧网关、URL 校验、模型选择、客户端隔离、Key 不持久化到偏好和授权声明保留。
- TypeScript 完整检查通过，Vite 生产构建通过。沙箱内 Vite 使用同等 React 插件与配置的程序化构建入口，避免沙箱父目录枚举限制；交付源码保留正常 pnpm build:web。
- Rust 核心/平台测试 94 项通过，全部使用临时配置目录和虚构 Key，覆盖配置合并、原有 MCP/hooks/权限保留、模型目录、重复导入、恢复、冲突阻止与 Unicode Windows 路径。
- 新增原生回归确认 Codex 实际生成 model_provider=ultragpt-connect 与 base_url=https://ultragpt.xyz.org.mx/v1，Claude 实际生成 ANTHROPIC_BASE_URL=https://ultragpt.xyz.org.mx，恢复后返回原状态。
- UI 浏览器模拟 Tauri IPC 完成默认站点→客户端→输入虚构 Key→模型勾选→导入成功流程；只调用一次导入，浏览器存储不包含测试 Key，页面无运行错误。该项是界面模拟，不是实际客户端联调。
- Windows Tauri release 原生构建通过：Rust 1.99.1、MSVC 14.44.35207、Windows SDK 10.0.26100.0，依赖按 Cargo.lock 校验；完整静态 CRT。
- EXE 版本信息 ProductName/FileDescription 为 UltraGPT-Connect，ProductVersion 为 0.1.6。启动冒烟检查中进程正常保持运行；未在发布版内输入真实密钥或点击真实导入，检查后关闭该测试进程。
- 打包脚本验证 x64 PE32+ GUI、asInvoker、版本一致性及不依赖外置 VC++ Runtime/WebView2Loader DLL。需要系统已有 Microsoft Edge WebView2 Runtime。

## 尚未完成

- 未访问用户真实 Key 的模型目录、未调用付费模型接口，也未修改当前用户的 Claude/Codex 配置。用户须用自己 UltraGPT 分组的 Key 导入后，在目标客户端发一条短消息检查实际线路。
- 未进行本分支真实 Claude/Codex app-server 端到端测试；原项目历史测试记录归档在 docs/upstream/，仅供参考。
- 当前运行环境为 Windows，macOS Apple Silicon 尚未原生构建、启动或验签。本次不会提供改名后的原 Mac 应用。对应代码、图标、包名、构建脚本和 GitHub Actions 工作流均已准备。
- Windows 未做发布者代码签名；macOS 构建脚本采用 ad-hoc 签名，不包含 Apple Developer ID 签名或公证。

## 使用边界

获取模型目录只说明该 Key 的列表接口可用，不保证每个模型或工具调用可用。Claude 客户端须使用 Messages 兼容线路，Codex 须使用 Responses 兼容线路。账号和分组限制仍由 UltraGPT 网站决定。

一键导入前仍需用户填写自己的 API Key、选择客户端和默认模型。不会给所有客户共享一个管理员密钥，也不会把密钥预埋在安装包里。
