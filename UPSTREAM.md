# 来源和修改说明

UltraGPT-Connect 基于 SuperGPT Connect v0.1.6（MIT），对应源码提交 65c260eb1093341f44f9fbb2dc01fbb53cd72140。

原源码：https://github.com/superpilot69/supergpt-connect/tree/v0.1.6

已保留原 LICENSE、CC Switch MIT 许可和第三方声明。原项目关于 CC Switch、模型能力数据及其他依赖的详细说明保存在 docs/upstream/UPSTREAM.md。原项目验证记录归档于 docs/upstream/VERIFICATION.md，不能视为本分支已独立完成相同验证。

本分支修改：

- 默认站点和 API 根地址设为 UltraGPT / https://ultragpt.xyz.org.mx。
- 应用名 UltraGPT-Connect；独立应用 ID mx.org.xyz.ultragpt.connect。
- 使用新制作的 U 字图标，替换原 SuperGPT S 品牌图标。
- 本地偏好与备份目录改为 .ultragpt-connect；Codex provider ID 为 ultragpt-connect，模型目录为 ultragpt-models.json，Claude 桌面 profile 使用新的独立 ID。
- 用户仍需提供自己在 UltraGPT 创建的 Key；保留原站点选择、模型读取、预览、备份与恢复流程。
- 不覆盖或自动迁移原 SuperGPT Connect 的恢复记录；它们留给原程序管理。若安装过原连接器，新程序首次备份的是当前配置，恢复后回到本次导入前状态。
- 能力查询兼容原协议标识 supergpt.connect_capabilities，也接受 ultragpt.connect_capabilities；这个字符串是协议兼容标识，不是请求目的地址。

此次分支的验证范围见 VALIDATION.md。本程序不是 Claude、Codex 或 OpenAI 官方客户端。
