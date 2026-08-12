use EfficiencyCode_cloud_config::cloud_config_bundle_loader_for_storage;
use EfficiencyCode_config::CloudConfigBundleLoader;
use EfficiencyCode_config::ConfigLoadOptions;
use EfficiencyCode_core::config::Config;
use EfficiencyCode_core::config::ConfigBuilder;
use EfficiencyCode_core::config::LoaderOverrides;
use EfficiencyCode_core::config::bootstrap_auth_config;
use EfficiencyCode_core::config::find_EfficiencyCode_home;
use EfficiencyCode_core::config::load_config_toml_with_layer_stack;
use EfficiencyCode_utils_absolute_path::AbsolutePathBuf;
use EfficiencyCode_utils_cli::CliConfigOverrides;
use anyhow::Context;
use anyhow::Result;

pub(super) async fn load_mcp_config(
    config_overrides: &CliConfigOverrides,
    loader_overrides: LoaderOverrides,
) -> Result<Config> {
    let cli_overrides = config_overrides
        .parse_overrides()
        .map_err(anyhow::Error::msg)?;
    let EfficiencyCode_home =
        find_EfficiencyCode_home().context("failed to resolve EfficiencyCode_HOME")?;
    let cwd = AbsolutePathBuf::current_dir().context("failed to resolve current directory")?;
    let bootstrap_config = load_config_toml_with_layer_stack(
        EfficiencyCode_home.as_path(),
        Some(&cwd),
        cli_overrides.clone(),
        ConfigLoadOptions {
            loader_overrides: loader_overrides.clone(),
            strict_config: false,
            cloud_config_bundle: CloudConfigBundleLoader::default(),
        },
    )
    .await
    .context("failed to load bootstrap configuration")?;
    let cloud_config_bundle = cloud_config_bundle_loader_for_storage(
        bootstrap_auth_config(EfficiencyCode_home.as_path(), &bootstrap_config)
            .context("failed to resolve cloud configuration authentication")?,
        /*enable_EfficiencyCode_api_key_env*/ false,
    )
    .await;

    ConfigBuilder::default()
        .EfficiencyCode_home(EfficiencyCode_home.to_path_buf())
        .cli_overrides(cli_overrides)
        .loader_overrides(loader_overrides)
        .cloud_config_bundle(cloud_config_bundle)
        .build()
        .await
        .context("failed to load configuration")
}
