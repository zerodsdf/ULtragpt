# 兼容性说明

本分支沿用 SuperGPT Connect v0.1.6 的客户端适配实现，并替换品牌、默认站点与本地配置命名空间。

原项目的详细适配范围与限制保留于 [docs/upstream/COMPATIBILITY.md](docs/upstream/COMPATIBILITY.md)。该记录仅说明来源实现，不代表 UltraGPT 上游线路均已验证。

本分支实际检查见 [VALIDATION.md](VALIDATION.md)。目标客户端必须支持对应第三方 API 配置，Claude 使用 Messages 兼容分组，Codex 使用 Responses 兼容分组；获取模型列表和导入成功不等同于模型调用已通过。
