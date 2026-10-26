# UltraGPT-Connect

UltraGPT 本地 API 配置助手，基于 SuperGPT Connect v0.1.6 的 MIT 开源代码定制。默认接入 **https://ultragpt.xyz.org.mx**，不需要用户手动填写中转地址。

## 使用

1. 解压对应系统的软件包，打开 UltraGPT-Connect。
2. 保持默认站点 UltraGPT，选择已安装的 Claude 或 Codex 客户端。
3. 填写自己在 UltraGPT 网站“API 密钥”页面创建的 Key。不要填写上游供应商 Key，也不要分享管理员 Key。
4. 点击“获取可用模型”，勾选需要的模型并指定默认模型，然后点击导入。
5. 完全退出目标客户端后重新打开，新建会话确认生效。获取模型和写入配置不代表已验证实际模型调用；首次可发一条短消息核对网站使用记录。

Claude 客户端请选择网站 Anthropic/Claude 分组的 Key，Codex 请选择支持 Responses 的 OpenAI 分组 Key。一个 Key 可调用哪些模型由网站分组决定。没有自动免费额度，也不会自动创建网站账号、密钥或充值。

原有“恢复原配置”功能保留。导入前备份，发生配置冲突时提示，不静默覆盖用户后续修改。MCP、hooks、无关服务商和其他客户端设置沿用上游配置引擎的保留规则。

## 品牌与数据

- 程序名称：UltraGPT-Connect。
- 默认 API 根地址：https://ultragpt.xyz.org.mx；程序会按接口需要使用 /v1/models、/v1/responses 或 /v1/messages。
- Codex 服务商名称/ID：UltraGPT-Connect / ultragpt-connect。
- 备份和恢复记录：用户目录下 .ultragpt-connect。可能包含原配置中的凭据，不能公开上传。
- Codex 模型目录：ultragpt-models.json。
- API Key 不写入网页 localStorage，导入时只写入所选客户端需要的认证配置。
- 与原软件使用独立恢复目录和 Claude 桌面 profile，不擅自清理原软件备份。若原来已经导入过别的服务商，新程序的恢复点是本次导入前的配置。
- 无自动更新请求指向原站点。保留的 supergpt.connect_capabilities 字符串仅作能力协议兼容，API 请求目的地址始终为用户选中的站点。

## 原生构建

需要 Node.js 22+、pnpm 10.28.0 和 Rust。Windows 原生构建需要 MSVC 与 Windows SDK；Mac Apple Silicon 构建需要 macOS 和 Xcode Command Line Tools。

```sh
pnpm install --frozen-lockfile
pnpm check
pnpm test:ui
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --locked
```

Windows x64：

```sh
pnpm build:windows
```

macOS Apple Silicon：

```sh
rustup target add aarch64-apple-darwin
pnpm build:macos
```

输出位于 release/，文件名分别为 UltraGPT-Connect-0.1.6-Windows-x64-portable.zip 和 UltraGPT-Connect-0.1.6-macOS-arm64.zip。

源码内已提供 .github/workflows/ultragpt-build.yml：把完整源码放入自己的 GitHub 仓库，在 Actions 手动运行“Build UltraGPT-Connect”。它分别在 Windows 和 macOS runner 执行测试、构建并上传两个可下载 artifacts，不发布 Release、不修改你的网站。下载 artifact 后里面才是供客户解压使用的 ZIP。

本地网页预览只用于界面检查，不能直接替代桌面程序读写客户端配置：pnpm dev:web。

## 验证和许可

本分支已执行与未执行的检查见 VALIDATION.md。原项目的历史验证报告留在 docs/upstream/，不能当成本次改版的独立验证结论。

原 LICENSE 与第三方声明全部保留，修改允许依据压缩包所附 MIT 许可。来源及修改清单见 UPSTREAM.md。程序不是 Claude 或 Codex 官方客户端。Windows 软件包未签署发布者证书；macOS 构建脚本使用 ad-hoc 签名，未做 Apple Developer ID 公证。
