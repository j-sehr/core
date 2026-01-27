pub(super) struct TemplateModule;
impl Module for TemplateModule {
    fn name() -> &'static str {
        "core-template"
    }

    async fn initialize(
        env_config: EnviromentConfiguration,
        file: FileConfiguration,
    ) -> anyhow::Result<Option<axum::Router>> {
        Ok(None)
    }
}
